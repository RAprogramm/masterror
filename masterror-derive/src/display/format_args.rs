// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Format argument resolution and environment management.
//!
//! This module manages the resolution of format arguments in display templates.
//! It handles named arguments, positional arguments, implicit arguments, and
//! shorthand field projections. The environment tracks all format arguments
//! and resolves placeholders to concrete expressions during code generation.

use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

use super::{placeholder::ResolvedPlaceholderExpr, projection::append_projection_segments};
use crate::{
    input::{
        Field, Fields, FormatArg, FormatArgProjection, FormatArgProjectionSegment,
        FormatArgShorthand, FormatArgValue, FormatArgsSpec, FormatBindingKind
    },
    template_support::TemplatePlaceholderSpec
};

/// Environment for resolving format arguments in templates.
///
/// This type maintains the state needed to resolve placeholders in display
/// templates, including mappings from argument names/positions to their
/// expressions and handling of both struct and variant contexts.
#[derive(Debug)]
pub struct FormatArgumentsEnv<'a> {
    context:    FormatArgContext<'a>,
    args:       Vec<EnvFormatArg<'a>>,
    named:      HashMap<String, usize>,
    positional: HashMap<usize, usize>,
    implicit:   Vec<Option<usize>>
}

/// Context for format argument resolution.
///
/// Distinguishes between struct and variant contexts, as field access
/// differs between direct struct field access and variant pattern bindings.
#[derive(Debug)]
enum FormatArgContext<'a> {
    Struct(&'a Fields),
    Variant {
        fields:   &'a Fields,
        bindings: Vec<Ident>
    }
}

/// A format argument with its optional binding identifier.
///
/// Expression-based arguments get temporary bindings, while shorthand
/// arguments reference fields directly.
#[derive(Debug)]
struct EnvFormatArg<'a> {
    binding: Option<Ident>,
    arg:     &'a FormatArg
}

/// A resolved format argument ready for code generation.
///
/// Contains the argument kind (named, positional, or implicit) along with
/// the expression token stream that produces the argument value.
#[derive(Debug)]
pub struct ResolvedFormatArgument {
    /// The kind of argument (named, positional, or implicit)
    pub kind: ResolvedFormatArgumentKind,
    /// The expression that produces the argument value
    pub expr: TokenStream
}

/// The kind of format argument binding.
///
/// Format arguments can be referenced by name, by position, or implicitly
/// based on their order in the template.
#[derive(Debug)]
pub enum ResolvedFormatArgumentKind {
    /// Named argument like `{name}`
    Named(Ident),
    /// Positional argument like `{0}`
    Positional(usize),
    /// Implicit argument referenced by position
    Implicit(usize)
}

impl<'a> FormatArgumentsEnv<'a> {
    /// Creates a new format arguments environment for a struct context.
    ///
    /// # Arguments
    ///
    /// * `spec` - The format arguments specification from the error attribute
    /// * `fields` - The struct's fields for resolving shorthand references
    ///
    /// # Returns
    ///
    /// A new `FormatArgumentsEnv` configured for struct field resolution
    pub fn new_struct(spec: &'a FormatArgsSpec, fields: &'a Fields) -> Self {
        Self::new_with_context(spec, FormatArgContext::Struct(fields))
    }

    /// Creates a new format arguments environment for a variant context.
    ///
    /// # Arguments
    ///
    /// * `spec` - The format arguments specification from the error attribute
    /// * `fields` - The variant's fields for resolving shorthand references
    /// * `bindings` - The binding identifiers from the match pattern
    ///
    /// # Returns
    ///
    /// A new `FormatArgumentsEnv` configured for variant binding resolution
    pub fn new_variant(spec: &'a FormatArgsSpec, fields: &'a Fields, bindings: &[Ident]) -> Self {
        Self::new_with_context(
            spec,
            FormatArgContext::Variant {
                fields,
                bindings: bindings.to_vec()
            }
        )
    }

    fn new_with_context(spec: &'a FormatArgsSpec, context: FormatArgContext<'a>) -> Self {
        let mut env = Self {
            context,
            args: Vec::new(),
            named: HashMap::new(),
            positional: HashMap::new(),
            implicit: Vec::new()
        };

        for (index, arg) in spec.args.iter().enumerate() {
            let binding = match &arg.value {
                FormatArgValue::Expr(_) => Some(format_ident!("__masterror_format_arg_{}", index)),
                FormatArgValue::Shorthand(_) => None
            };

            let arg_index = env.args.len();
            env.args.push(EnvFormatArg {
                binding: binding.clone(),
                arg
            });

            match &arg.kind {
                FormatBindingKind::Named(ident) => {
                    env.named.insert(ident.to_string(), arg_index);
                }
                FormatBindingKind::Positional(pos_index) => {
                    env.positional.insert(*pos_index, arg_index);
                    env.implicit.push(Some(arg_index));
                }
                FormatBindingKind::Implicit(implicit_index) => {
                    env.register_implicit(*implicit_index, arg_index);
                }
            }
        }

        env
    }

    /// Generates prelude statements for format argument bindings.
    ///
    /// Creates `let` bindings for expression-based format arguments that need
    /// to be evaluated before the format string is processed.
    ///
    /// # Returns
    ///
    /// Vector of token streams containing `let` binding statements
    pub fn prelude_tokens(&self) -> Vec<TokenStream> {
        self.args.iter().map(EnvFormatArg::prelude_tokens).collect()
    }

    /// Resolves all format arguments to their final expressions.
    ///
    /// Converts each format argument into its resolved form, handling both
    /// expression-based arguments and shorthand field references.
    ///
    /// # Returns
    ///
    /// Vector of resolved format arguments, or an error if resolution fails
    pub fn argument_tokens(&self) -> Result<Vec<ResolvedFormatArgument>, Error> {
        self.args
            .iter()
            .map(|arg| arg.argument_tokens(self))
            .collect()
    }

    /// Resolves a template placeholder to a concrete expression.
    ///
    /// Attempts to resolve a placeholder by looking it up in the format
    /// arguments environment. Returns `None` if the placeholder is not
    /// found in this environment (allowing fallback to field-based
    /// resolution).
    ///
    /// # Arguments
    ///
    /// * `placeholder` - The placeholder specification to resolve
    ///
    /// # Returns
    ///
    /// `Ok(Some(expr))` if resolved, `Ok(None)` if not found, or `Err` on
    /// resolution error
    pub fn resolve_placeholder(
        &mut self,
        placeholder: &TemplatePlaceholderSpec
    ) -> Result<Option<ResolvedPlaceholderExpr>, Error> {
        use crate::template_support::TemplateIdentifierSpec;

        let arg_index = match &placeholder.identifier {
            TemplateIdentifierSpec::Named(name) => self.named.get(name).copied(),
            TemplateIdentifierSpec::Positional(index) => self.positional.get(index).copied(),
            TemplateIdentifierSpec::Implicit(index) => {
                self.implicit.get(*index).and_then(|slot| *slot)
            }
        };

        let index = match arg_index {
            Some(index) => index,
            None => return Ok(None)
        };

        let resolved = self.args[index].resolved_expr(self, placeholder)?;
        Ok(Some(resolved))
    }

    fn register_implicit(&mut self, index: usize, arg_index: usize) {
        if self.implicit.len() <= index {
            self.implicit.resize(index + 1, None);
        }
        self.implicit[index] = Some(arg_index);
    }

    fn resolve_shorthand(
        &self,
        shorthand: &FormatArgShorthand,
        placeholder: &TemplatePlaceholderSpec
    ) -> Result<ResolvedPlaceholderExpr, Error> {
        match &self.context {
            FormatArgContext::Struct(fields) => {
                resolve_struct_shorthand(fields, shorthand, placeholder)
            }
            FormatArgContext::Variant {
                fields,
                bindings
            } => resolve_variant_shorthand(fields, bindings, shorthand, placeholder)
        }
    }

    fn resolve_shorthand_argument(
        &self,
        shorthand: &FormatArgShorthand
    ) -> Result<TokenStream, Error> {
        match &self.context {
            FormatArgContext::Struct(fields) => {
                resolve_struct_shorthand_argument(fields, shorthand)
            }
            FormatArgContext::Variant {
                fields,
                bindings
            } => resolve_variant_shorthand_argument(fields, bindings, shorthand)
        }
    }
}

impl<'a> EnvFormatArg<'a> {
    fn prelude_tokens(&self) -> TokenStream {
        match (&self.binding, &self.arg.value) {
            (Some(binding), FormatArgValue::Expr(expr)) => {
                quote! { let #binding = #expr; }
            }
            _ => TokenStream::new()
        }
    }

    fn resolved_expr(
        &self,
        env: &FormatArgumentsEnv<'_>,
        placeholder: &TemplatePlaceholderSpec
    ) -> Result<ResolvedPlaceholderExpr, Error> {
        use super::formatter::needs_pointer_value;

        match (&self.binding, &self.arg.value) {
            (Some(binding), FormatArgValue::Expr(_)) => {
                if needs_pointer_value(&placeholder.formatter) {
                    Ok(ResolvedPlaceholderExpr::with(quote!(#binding), true))
                } else {
                    Ok(ResolvedPlaceholderExpr::new(quote!(&#binding)))
                }
            }
            (_, FormatArgValue::Shorthand(shorthand)) => {
                env.resolve_shorthand(shorthand, placeholder)
            }
            _ => unreachable!()
        }
    }

    fn argument_tokens(
        &self,
        env: &FormatArgumentsEnv<'_>
    ) -> Result<ResolvedFormatArgument, Error> {
        let expr = match (&self.binding, &self.arg.value) {
            (Some(binding), FormatArgValue::Expr(_)) => Ok(quote!(#binding)),
            (_, FormatArgValue::Shorthand(shorthand)) => env.resolve_shorthand_argument(shorthand),
            _ => Err(Error::new(
                self.arg.span,
                "format argument expression binding was not generated"
            ))
        }?;

        let kind = match &self.arg.kind {
            FormatBindingKind::Named(ident) => ResolvedFormatArgumentKind::Named(ident.clone()),
            FormatBindingKind::Positional(index) => ResolvedFormatArgumentKind::Positional(*index),
            FormatBindingKind::Implicit(index) => ResolvedFormatArgumentKind::Implicit(*index)
        };

        Ok(ResolvedFormatArgument {
            kind,
            expr
        })
    }
}

fn resolve_struct_shorthand(
    fields: &Fields,
    shorthand: &FormatArgShorthand,
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    use super::formatter::needs_pointer_value;

    let FormatArgShorthand::Projection(projection) = shorthand;

    let (expr, first_field, has_tail) = struct_projection_expr(fields, projection)?;

    if !has_tail && let Some(field) = first_field {
        return Ok(struct_field_expr(field, &placeholder.formatter));
    }

    if needs_pointer_value(&placeholder.formatter) {
        Ok(ResolvedPlaceholderExpr::with(expr, false))
    } else {
        Ok(ResolvedPlaceholderExpr::new(quote!(&(#expr))))
    }
}

fn resolve_variant_shorthand(
    fields: &Fields,
    bindings: &[Ident],
    shorthand: &FormatArgShorthand,
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    use super::formatter::needs_pointer_value;

    let FormatArgShorthand::Projection(projection) = shorthand;

    let Some(first_segment) = projection.segments.first() else {
        return Err(Error::new(
            projection.span,
            "empty shorthand projection is not supported"
        ));
    };

    match first_segment {
        FormatArgProjectionSegment::Field(ident) => {
            let Fields::Named(named_fields) = fields else {
                return Err(Error::new(
                    ident.span(),
                    format!(
                        "named field `{}` is not available for tuple variants",
                        ident
                    )
                ));
            };

            let position = named_fields.iter().position(|field| {
                field
                    .ident
                    .as_ref()
                    .is_some_and(|field_ident| field_ident == ident)
            });

            let index = position.ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("unknown field `{}` in format arguments", ident)
                )
            })?;

            let binding = bindings.get(index).ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("field `{}` is not available in format arguments", ident)
                )
            })?;

            let expr = if projection.segments.len() == 1 {
                quote!(#binding)
            } else {
                append_projection_segments(quote!(#binding), &projection.segments[1..])
            };

            if projection.segments.len() == 1 {
                Ok(ResolvedPlaceholderExpr::with(
                    expr,
                    needs_pointer_value(&placeholder.formatter)
                ))
            } else if needs_pointer_value(&placeholder.formatter) {
                Ok(ResolvedPlaceholderExpr::with(expr, false))
            } else {
                Ok(ResolvedPlaceholderExpr::new(quote!(&(#expr))))
            }
        }
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let Fields::Unnamed(_) = fields else {
                return Err(Error::new(
                    *span,
                    "positional fields are not available for struct variants"
                ));
            };

            let binding = bindings.get(*index).ok_or_else(|| {
                Error::new(
                    *span,
                    format!("field `{}` is not available in format arguments", index)
                )
            })?;

            let expr = if projection.segments.len() == 1 {
                quote!(#binding)
            } else {
                append_projection_segments(quote!(#binding), &projection.segments[1..])
            };

            if projection.segments.len() == 1 {
                Ok(ResolvedPlaceholderExpr::with(
                    expr,
                    needs_pointer_value(&placeholder.formatter)
                ))
            } else if needs_pointer_value(&placeholder.formatter) {
                Ok(ResolvedPlaceholderExpr::with(expr, false))
            } else {
                Ok(ResolvedPlaceholderExpr::new(quote!(&(#expr))))
            }
        }
        FormatArgProjectionSegment::MethodCall(call) => Err(Error::new(
            call.span,
            "variant format projections must start with a field or index"
        ))
    }
}

fn resolve_struct_shorthand_argument(
    fields: &Fields,
    shorthand: &FormatArgShorthand
) -> Result<TokenStream, Error> {
    let FormatArgShorthand::Projection(projection) = shorthand;
    let (expr, ..) = struct_projection_expr(fields, projection)?;
    Ok(expr)
}

fn resolve_variant_shorthand_argument(
    fields: &Fields,
    bindings: &[Ident],
    shorthand: &FormatArgShorthand
) -> Result<TokenStream, Error> {
    let FormatArgShorthand::Projection(projection) = shorthand;

    let Some(first_segment) = projection.segments.first() else {
        return Err(Error::new(
            projection.span,
            "empty shorthand projection is not supported"
        ));
    };

    match first_segment {
        FormatArgProjectionSegment::Field(ident) => {
            let Fields::Named(named_fields) = fields else {
                return Err(Error::new(
                    ident.span(),
                    format!(
                        "named field `{}` is not available for tuple variants",
                        ident
                    )
                ));
            };

            let position = named_fields.iter().position(|field| {
                field
                    .ident
                    .as_ref()
                    .is_some_and(|field_ident| field_ident == ident)
            });

            let index = position.ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("unknown field `{}` in format arguments", ident)
                )
            })?;

            let binding = bindings.get(index).ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("field `{}` is not available in format arguments", ident)
                )
            })?;

            if projection.segments.len() == 1 {
                Ok(quote!(#binding))
            } else {
                Ok(append_projection_segments(
                    quote!(#binding),
                    &projection.segments[1..]
                ))
            }
        }
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let Fields::Unnamed(_) = fields else {
                return Err(Error::new(
                    *span,
                    "positional fields are not available for struct variants"
                ));
            };

            let binding = bindings.get(*index).ok_or_else(|| {
                Error::new(
                    *span,
                    format!("field `{}` is not available in format arguments", index)
                )
            })?;

            if projection.segments.len() == 1 {
                Ok(quote!(#binding))
            } else {
                Ok(append_projection_segments(
                    quote!(#binding),
                    &projection.segments[1..]
                ))
            }
        }
        FormatArgProjectionSegment::MethodCall(call) => Err(Error::new(
            call.span,
            "variant format projections must start with a field or index"
        ))
    }
}

fn struct_projection_expr<'a>(
    fields: &'a Fields,
    projection: &'a FormatArgProjection
) -> Result<(TokenStream, Option<&'a Field>, bool), Error> {
    use super::projection::append_method_call;

    let Some(first) = projection.segments.first() else {
        return Err(Error::new(
            projection.span,
            "empty shorthand projection is not supported"
        ));
    };

    let mut first_field = None;
    let mut expr = match first {
        FormatArgProjectionSegment::Field(ident) => {
            let field = fields.get_named(&ident.to_string()).ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("unknown field `{}` in format arguments", ident)
                )
            })?;
            first_field = Some(field);
            let member = &field.member;
            quote!(self.#member)
        }
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let field = fields.get_positional(*index).ok_or_else(|| {
                Error::new(
                    *span,
                    format!("field `{}` is not available in format arguments", index)
                )
            })?;
            first_field = Some(field);
            let member = &field.member;
            quote!(self.#member)
        }
        FormatArgProjectionSegment::MethodCall(call) => append_method_call(quote!(self), call)
    };

    if projection.segments.len() > 1 {
        expr = append_projection_segments(expr, &projection.segments[1..]);
    }

    Ok((expr, first_field, projection.segments.len() > 1))
}

fn struct_field_expr(
    field: &Field,
    formatter: &masterror_template::template::TemplateFormatter
) -> ResolvedPlaceholderExpr {
    use super::{formatter::needs_pointer_value, placeholder::pointer_prefers_value};

    let member = &field.member;

    if needs_pointer_value(formatter) && pointer_prefers_value(&field.ty) {
        ResolvedPlaceholderExpr::pointer(quote!(self.#member))
    } else {
        ResolvedPlaceholderExpr::new(quote!(&self.#member))
    }
}

#[cfg(test)]
mod tests {
    use masterror_template::template::TemplateFormatter;
    use proc_macro2::Span;
    use quote::format_ident;
    use syn::{Member, parse_quote};

    use super::*;
    use crate::{
        input::{Field, FieldAttrs, FormatArgProjection, FormatArgProjectionMethodCall},
        template_support::TemplateIdentifierSpec
    };

    fn make_test_field(name: &str, ty: syn::Type, index: usize) -> Field {
        Field {
            ident: Some(format_ident!("{}", name)),
            member: Member::Named(format_ident!("{}", name)),
            ty,
            index,
            attrs: FieldAttrs::default(),
            span: Span::call_site()
        }
    }

    fn make_test_unnamed_field(ty: syn::Type, index: usize) -> Field {
        Field {
            ident: None,
            member: Member::Unnamed(syn::Index {
                index: index as u32,
                span:  Span::call_site()
            }),
            ty,
            index,
            attrs: FieldAttrs::default(),
            span: Span::call_site()
        }
    }

    #[test]
    fn test_format_arguments_env_new_struct() {
        let spec = FormatArgsSpec {
            args: vec![]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        assert_eq!(env.args.len(), 0);
    }

    #[test]
    fn test_format_arguments_env_new_variant() {
        let spec = FormatArgsSpec {
            args: vec![]
        };
        let fields = Fields::Unit;
        let bindings = vec![];
        let env = FormatArgumentsEnv::new_variant(&spec, &fields, &bindings);
        assert_eq!(env.args.len(), 0);
    }

    #[test]
    fn test_format_arguments_env_with_named_arg() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Named(format_ident!("foo")),
                value: FormatArgValue::Expr(parse_quote!(42)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        assert_eq!(env.args.len(), 1);
        assert_eq!(env.named.len(), 1);
        assert!(env.named.contains_key("foo"));
    }

    #[test]
    fn test_format_arguments_env_with_positional_arg() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Positional(0),
                value: FormatArgValue::Expr(parse_quote!(42)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        assert_eq!(env.args.len(), 1);
        assert_eq!(env.positional.len(), 1);
        assert!(env.positional.contains_key(&0));
        assert_eq!(env.implicit.len(), 1);
    }

    #[test]
    fn test_format_arguments_env_with_implicit_arg() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Implicit(0),
                value: FormatArgValue::Expr(parse_quote!(42)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        assert_eq!(env.args.len(), 1);
        assert_eq!(env.implicit.len(), 1);
        assert!(env.implicit[0].is_some());
    }

    #[test]
    fn test_format_arguments_env_with_shorthand_arg() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Named(format_ident!("bar")),
                value: FormatArgValue::Shorthand(FormatArgShorthand::Projection(
                    FormatArgProjection {
                        segments: vec![FormatArgProjectionSegment::Field(format_ident!("bar"))],
                        span:     Span::call_site()
                    }
                )),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        assert_eq!(env.args.len(), 1);
        assert!(env.args[0].binding.is_none());
    }

    #[test]
    fn test_format_arguments_env_prelude_tokens_with_expr() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Named(format_ident!("x")),
                value: FormatArgValue::Expr(parse_quote!(10 + 20)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let preludes = env.prelude_tokens();
        assert_eq!(preludes.len(), 1);
        assert!(preludes[0].to_string().contains("let"));
    }

    #[test]
    fn test_format_arguments_env_prelude_tokens_with_shorthand() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Named(format_ident!("y")),
                value: FormatArgValue::Shorthand(FormatArgShorthand::Projection(
                    FormatArgProjection {
                        segments: vec![FormatArgProjectionSegment::Field(format_ident!("y"))],
                        span:     Span::call_site()
                    }
                )),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let preludes = env.prelude_tokens();
        assert_eq!(preludes.len(), 1);
        assert!(preludes[0].is_empty());
    }

    #[test]
    fn test_format_arguments_env_resolve_placeholder_named() -> Result<(), Error> {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Named(format_ident!("test")),
                value: FormatArgValue::Expr(parse_quote!(100)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let mut env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("test".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = env.resolve_placeholder(&placeholder)?;
        assert!(result.is_some());
        Ok(())
    }

    #[test]
    fn test_format_arguments_env_resolve_placeholder_positional() -> Result<(), Error> {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Positional(0),
                value: FormatArgValue::Expr(parse_quote!(200)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let mut env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = env.resolve_placeholder(&placeholder)?;
        assert!(result.is_some());
        Ok(())
    }

    #[test]
    fn test_format_arguments_env_resolve_placeholder_implicit() -> Result<(), Error> {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Implicit(0),
                value: FormatArgValue::Expr(parse_quote!(300)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let mut env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Implicit(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = env.resolve_placeholder(&placeholder)?;
        assert!(result.is_some());
        Ok(())
    }

    #[test]
    fn test_format_arguments_env_resolve_placeholder_not_found() -> Result<(), Error> {
        let spec = FormatArgsSpec {
            args: vec![]
        };
        let fields = Fields::Unit;
        let mut env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("nonexistent".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = env.resolve_placeholder(&placeholder)?;
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn test_format_arguments_env_resolve_placeholder_with_pointer_formatter() -> Result<(), Error>
    {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Named(format_ident!("ptr")),
                value: FormatArgValue::Expr(parse_quote!(&value)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let mut env = FormatArgumentsEnv::new_struct(&spec, &fields);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("ptr".to_string()),
            formatter:  TemplateFormatter::Pointer {
                alternate: false
            },
            span:       Span::call_site()
        };
        let result = env.resolve_placeholder(&placeholder)?;
        assert!(result.is_some());
        let resolved = result.unwrap();
        assert!(resolved.pointer_value);
        Ok(())
    }

    #[test]
    fn test_resolve_struct_shorthand_single_field() -> Result<(), Error> {
        let field = make_test_field("value", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("value"))],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("value".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_struct_shorthand(&fields, &shorthand, &placeholder)?;
        assert!(result.expr.to_string().contains("self"));
        assert!(result.expr.to_string().contains("value"));
        Ok(())
    }

    #[test]
    fn test_resolve_struct_shorthand_with_projection() -> Result<(), Error> {
        let field = make_test_field("inner", parse_quote!(Inner), 0);
        let fields = Fields::Named(vec![field]);
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![
                FormatArgProjectionSegment::Field(format_ident!("inner")),
                FormatArgProjectionSegment::Field(format_ident!("value")),
            ],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("inner".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_struct_shorthand(&fields, &shorthand, &placeholder)?;
        assert!(result.expr.to_string().contains("inner"));
        assert!(result.expr.to_string().contains("value"));
        Ok(())
    }

    #[test]
    fn test_resolve_struct_shorthand_unknown_field() {
        let fields = Fields::Named(vec![]);
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("unknown"))],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("unknown".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_struct_shorthand(&fields, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_struct_shorthand_with_index() -> Result<(), Error> {
        let field = make_test_unnamed_field(parse_quote!(i32), 0);
        let fields = Fields::Unnamed(vec![field]);
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            }],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_struct_shorthand(&fields, &shorthand, &placeholder)?;
        assert!(result.expr.to_string().contains("self"));
        Ok(())
    }

    #[test]
    fn test_resolve_struct_shorthand_with_invalid_index() {
        let fields = Fields::Unnamed(vec![]);
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            }],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_struct_shorthand(&fields, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_named_field() -> Result<(), Error> {
        let field = make_test_field("message", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("message"))],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("message".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder)?;
        assert!(!result.pointer_value);
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_named_field_with_projection() -> Result<(), Error> {
        let field = make_test_field("inner", parse_quote!(Inner), 0);
        let fields = Fields::Named(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![
                FormatArgProjectionSegment::Field(format_ident!("inner")),
                FormatArgProjectionSegment::Field(format_ident!("value")),
            ],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("inner".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder)?;
        assert!(!result.pointer_value);
        assert!(result.expr.to_string().contains("value"));
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_unknown_named_field() {
        let fields = Fields::Named(vec![]);
        let bindings = vec![];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("unknown"))],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("unknown".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_named_field_on_tuple_variant() {
        let field = make_test_unnamed_field(parse_quote!(String), 0);
        let fields = Fields::Unnamed(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("name"))],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("name".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_index() -> Result<(), Error> {
        let field = make_test_unnamed_field(parse_quote!(i32), 0);
        let fields = Fields::Unnamed(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            }],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder)?;
        assert!(!result.pointer_value);
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_index_with_projection() -> Result<(), Error> {
        let field = make_test_unnamed_field(parse_quote!(Inner), 0);
        let fields = Fields::Unnamed(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![
                FormatArgProjectionSegment::Index {
                    index: 0,
                    span:  Span::call_site()
                },
                FormatArgProjectionSegment::Field(format_ident!("value")),
            ],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder)?;
        assert!(!result.pointer_value);
        assert!(result.expr.to_string().contains("value"));
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_index_on_struct_variant() {
        let field = make_test_field("value", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            }],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_out_of_bounds_index() {
        let field = make_test_unnamed_field(parse_quote!(i32), 0);
        let fields = Fields::Unnamed(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Index {
                index: 5,
                span:  Span::call_site()
            }],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(5),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_method_call_error() {
        let field = make_test_field("value", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::MethodCall(
                FormatArgProjectionMethodCall {
                    method:    format_ident!("to_string"),
                    turbofish: None,
                    args:      syn::punctuated::Punctuated::new(),
                    span:      Span::call_site()
                }
            )],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("method".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_empty_projection() {
        let field = make_test_field("value", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![],
            span:     Span::call_site()
        });
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("value".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = resolve_variant_shorthand(&fields, &bindings, &shorthand, &placeholder);
        assert!(result.is_err());
    }

    #[test]
    fn test_struct_projection_expr_single_field() -> Result<(), Error> {
        let field = make_test_field("name", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let projection = FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("name"))],
            span:     Span::call_site()
        };
        let (expr, first_field, has_tail) = struct_projection_expr(&fields, &projection)?;
        assert!(expr.to_string().contains("self"));
        assert!(expr.to_string().contains("name"));
        assert!(first_field.is_some());
        assert!(!has_tail);
        Ok(())
    }

    #[test]
    fn test_struct_projection_expr_with_method_call() -> Result<(), Error> {
        let fields = Fields::Unit;
        let projection = FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::MethodCall(
                FormatArgProjectionMethodCall {
                    method:    format_ident!("to_string"),
                    turbofish: None,
                    args:      syn::punctuated::Punctuated::new(),
                    span:      Span::call_site()
                }
            )],
            span:     Span::call_site()
        };
        let (expr, first_field, has_tail) = struct_projection_expr(&fields, &projection)?;
        assert!(expr.to_string().contains("self"));
        assert!(expr.to_string().contains("to_string"));
        assert!(first_field.is_none());
        assert!(!has_tail);
        Ok(())
    }

    #[test]
    fn test_struct_projection_expr_empty_projection() {
        let fields = Fields::Unit;
        let projection = FormatArgProjection {
            segments: vec![],
            span:     Span::call_site()
        };
        let result = struct_projection_expr(&fields, &projection);
        assert!(result.is_err());
    }

    #[test]
    fn test_struct_projection_expr_complex_chain() -> Result<(), Error> {
        let field = make_test_field("inner", parse_quote!(Inner), 0);
        let fields = Fields::Named(vec![field]);
        let projection = FormatArgProjection {
            segments: vec![
                FormatArgProjectionSegment::Field(format_ident!("inner")),
                FormatArgProjectionSegment::Field(format_ident!("value")),
                FormatArgProjectionSegment::Index {
                    index: 0,
                    span:  Span::call_site()
                },
            ],
            span:     Span::call_site()
        };
        let (expr, first_field, has_tail) = struct_projection_expr(&fields, &projection)?;
        assert!(expr.to_string().contains("inner"));
        assert!(expr.to_string().contains("value"));
        assert!(first_field.is_some());
        assert!(has_tail);
        Ok(())
    }

    #[test]
    fn test_struct_field_expr_with_pointer_formatter() {
        let field = make_test_field("ptr", parse_quote!(*const i32), 0);
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        let result = struct_field_expr(&field, &formatter);
        assert!(result.pointer_value);
        assert!(result.expr.to_string().contains("self"));
        assert!(result.expr.to_string().contains("ptr"));
    }

    #[test]
    fn test_struct_field_expr_with_display_formatter() {
        let field = make_test_field("value", parse_quote!(String), 0);
        let formatter = TemplateFormatter::Display {
            spec: None
        };
        let result = struct_field_expr(&field, &formatter);
        assert!(!result.pointer_value);
        assert!(result.expr.to_string().contains("self"));
        assert!(result.expr.to_string().contains("value"));
    }

    #[test]
    fn test_struct_field_expr_with_immutable_reference() {
        let field = make_test_field("ref_val", parse_quote!(&i32), 0);
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        let result = struct_field_expr(&field, &formatter);
        assert!(result.pointer_value);
    }

    #[test]
    fn test_format_arguments_env_register_implicit_extends_vec() {
        let spec = FormatArgsSpec {
            args: vec![FormatArg {
                kind:  FormatBindingKind::Implicit(5),
                value: FormatArgValue::Expr(parse_quote!(42)),
                span:  Span::call_site()
            }]
        };
        let fields = Fields::Unit;
        let env = FormatArgumentsEnv::new_struct(&spec, &fields);
        assert_eq!(env.implicit.len(), 6);
        assert!(env.implicit[5].is_some());
    }

    #[test]
    fn test_resolve_struct_shorthand_argument() -> Result<(), Error> {
        let field = make_test_field("value", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("value"))],
            span:     Span::call_site()
        });
        let result = resolve_struct_shorthand_argument(&fields, &shorthand)?;
        assert!(result.to_string().contains("self"));
        assert!(result.to_string().contains("value"));
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_argument_named_field() -> Result<(), Error> {
        let field = make_test_field("message", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Field(format_ident!("message"))],
            span:     Span::call_site()
        });
        let result = resolve_variant_shorthand_argument(&fields, &bindings, &shorthand)?;
        assert!(result.to_string().contains("__field0"));
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_argument_index() -> Result<(), Error> {
        let field = make_test_unnamed_field(parse_quote!(i32), 0);
        let fields = Fields::Unnamed(vec![field]);
        let bindings = vec![format_ident!("__field0")];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            }],
            span:     Span::call_site()
        });
        let result = resolve_variant_shorthand_argument(&fields, &bindings, &shorthand)?;
        assert!(result.to_string().contains("__field0"));
        Ok(())
    }

    #[test]
    fn test_resolve_variant_shorthand_argument_method_call_error() {
        let fields = Fields::Unit;
        let bindings = vec![];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![FormatArgProjectionSegment::MethodCall(
                FormatArgProjectionMethodCall {
                    method:    format_ident!("method"),
                    turbofish: None,
                    args:      syn::punctuated::Punctuated::new(),
                    span:      Span::call_site()
                }
            )],
            span:     Span::call_site()
        });
        let result = resolve_variant_shorthand_argument(&fields, &bindings, &shorthand);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_variant_shorthand_argument_empty_projection() {
        let fields = Fields::Unit;
        let bindings = vec![];
        let shorthand = FormatArgShorthand::Projection(FormatArgProjection {
            segments: vec![],
            span:     Span::call_site()
        });
        let result = resolve_variant_shorthand_argument(&fields, &bindings, &shorthand);
        assert!(result.is_err());
    }
}
