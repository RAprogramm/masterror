// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Display implementation generation for struct error types.
//!
//! This module handles the generation of `Display` trait implementations for
//! struct-based error types. It supports various display strategies including
//! transparent delegation, template-based formatting, and custom formatter
//! functions.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

use super::{
    format_args::FormatArgumentsEnv,
    formatter::needs_pointer_value,
    placeholder::{ResolvedPlaceholderExpr, pointer_prefers_value},
    template::render_template
};
use crate::{
    input::{DisplaySpec, ErrorInput, Field, Fields, StructData, placeholder_error},
    template_support::TemplateIdentifierSpec
};

/// Generates the Display trait implementation for a struct error type.
///
/// This function dispatches to the appropriate rendering strategy based on
/// the display specification (transparent, template, or formatter path).
///
/// # Arguments
///
/// * `input` - The error type input with generics and metadata
/// * `data` - The struct-specific data including fields and display spec
///
/// # Returns
///
/// Token stream containing the complete Display trait implementation
pub fn expand_struct(input: &ErrorInput, data: &StructData) -> Result<TokenStream, Error> {
    let body = match &data.display {
        DisplaySpec::Transparent {
            ..
        } => render_struct_transparent(&data.fields),
        DisplaySpec::Template(template) => {
            render_template(template, Vec::new(), Vec::new(), |placeholder| {
                struct_placeholder_expr(&data.fields, placeholder, None)
            })?
        }
        DisplaySpec::TemplateWithArgs {
            template,
            args
        } => {
            let mut env = FormatArgumentsEnv::new_struct(args, &data.fields);
            let preludes = env.prelude_tokens();
            let format_arguments = env.argument_tokens()?;
            render_template(template, preludes, format_arguments, |placeholder| {
                struct_placeholder_expr(&data.fields, placeholder, Some(&mut env))
            })?
        }
        DisplaySpec::FormatterPath {
            path, ..
        } => render_struct_formatter_path(&data.fields, path)
    };
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                #body
            }
        }
    })
}

/// Renders transparent display delegation for a struct.
///
/// For transparent structs, the Display implementation delegates to the
/// single field. Unit structs return `Ok(())`.
///
/// # Arguments
///
/// * `fields` - The struct's fields
///
/// # Returns
///
/// Token stream containing the Display delegation code
pub fn render_struct_transparent(fields: &Fields) -> TokenStream {
    if let Some(field) = fields.iter().next() {
        let member = &field.member;
        quote! {
            core::fmt::Display::fmt(&self.#member, f)
        }
    } else {
        quote! {
            Ok(())
        }
    }
}

/// Generates argument expressions for custom formatter function calls.
///
/// Creates references to all struct fields to pass as arguments to a
/// custom formatter function.
///
/// # Arguments
///
/// * `fields` - The struct's fields
///
/// # Returns
///
/// Vector of token streams, each representing a field reference
pub fn struct_formatter_arguments(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Unit => Vec::new(),
        Fields::Named(fields) | Fields::Unnamed(fields) => fields
            .iter()
            .map(|field| {
                let member = &field.member;
                quote!(&self.#member)
            })
            .collect()
    }
}

/// Generates a call to a custom formatter function.
///
/// Creates a function call expression with the provided arguments plus
/// the formatter argument `f`.
///
/// # Arguments
///
/// * `path` - The path to the formatter function
/// * `args` - The arguments to pass before the formatter
///
/// # Returns
///
/// Token stream containing the function call
pub fn formatter_path_call(path: &syn::ExprPath, mut args: Vec<TokenStream>) -> TokenStream {
    args.push(quote!(f));
    quote! {
        #path(#(#args),*)
    }
}

/// Renders a struct Display implementation using a custom formatter path.
///
/// # Arguments
///
/// * `fields` - The struct's fields
/// * `path` - The path to the custom formatter function
///
/// # Returns
///
/// Token stream containing the formatter function call
pub fn render_struct_formatter_path(fields: &Fields, path: &syn::ExprPath) -> TokenStream {
    let args = struct_formatter_arguments(fields);
    formatter_path_call(path, args)
}

/// Resolves a template placeholder to a struct field expression.
///
/// Attempts to resolve placeholders first from the format arguments
/// environment, then falls back to direct field access. Handles special
/// identifiers like `{self}`.
///
/// # Arguments
///
/// * `fields` - The struct's fields
/// * `placeholder` - The placeholder specification to resolve
/// * `env` - Optional format arguments environment for resolution
///
/// # Returns
///
/// Resolved placeholder expression, or an error if the placeholder cannot be
/// resolved
pub fn struct_placeholder_expr(
    fields: &Fields,
    placeholder: &crate::template_support::TemplatePlaceholderSpec,
    env: Option<&mut FormatArgumentsEnv<'_>>
) -> Result<ResolvedPlaceholderExpr, Error> {
    if matches!(
        &placeholder.identifier,
        TemplateIdentifierSpec::Named(name) if name == "self"
    ) {
        return Ok(ResolvedPlaceholderExpr::with(
            quote!(self),
            needs_pointer_value(&placeholder.formatter)
        ));
    }
    if let Some(env) = env
        && let Some(resolved) = env.resolve_placeholder(placeholder)?
    {
        return Ok(resolved);
    }
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) => {
            if let Some(field) = fields.get_named(name) {
                Ok(struct_field_expr(field, &placeholder.formatter))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => fields
            .get_positional(*index)
            .map(|field| struct_field_expr(field, &placeholder.formatter))
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier)),
        TemplateIdentifierSpec::Implicit(index) => fields
            .get_positional(*index)
            .map(|field| struct_field_expr(field, &placeholder.formatter))
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

/// Generates an expression for accessing a struct field with proper formatting.
///
/// Determines whether to pass the field by value or by reference based on
/// the formatter type and field type.
///
/// # Arguments
///
/// * `field` - The field to access
/// * `formatter` - The formatter that will be used on this field
///
/// # Returns
///
/// Resolved placeholder expression with appropriate reference/value handling
pub fn struct_field_expr(
    field: &Field,
    formatter: &masterror_template::template::TemplateFormatter
) -> ResolvedPlaceholderExpr {
    let member = &field.member;
    if needs_pointer_value(formatter) && pointer_prefers_value(&field.ty) {
        ResolvedPlaceholderExpr::pointer(quote!(self.#member))
    } else {
        ResolvedPlaceholderExpr::new(quote!(&self.#member))
    }
}

/// Generates a binding identifier for a field.
///
/// For named fields, uses the field's identifier. For unnamed fields,
/// generates a unique identifier based on the field index.
///
/// # Arguments
///
/// * `field` - The field to generate a binding for
///
/// # Returns
///
/// An identifier suitable for use as a pattern binding
pub fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::format_ident;
    use syn::{Member, parse_quote};

    use super::*;
    use crate::{
        input::{ErrorData, Field, FieldAttrs, FormatArgsSpec, StructData},
        template_support::{DisplayTemplate, TemplateSegmentSpec}
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

    fn make_error_input(ident: &str) -> ErrorInput {
        ErrorInput {
            ident:    format_ident!("{}", ident),
            generics: Default::default(),
            data:     ErrorData::Struct(Box::new(StructData {
                fields:      Fields::Unit,
                display:     DisplaySpec::Template(DisplayTemplate {
                    segments: vec![]
                }),
                format_args: FormatArgsSpec::default(),
                app_error:   None,
                masterror:   None
            }))
        }
    }

    #[test]
    fn test_expand_struct_transparent_unit() {
        let input = make_error_input("MyError");
        let data = StructData {
            fields:      Fields::Unit,
            display:     DisplaySpec::Transparent {
                attribute: Box::new(parse_quote!(#[error(transparent)]))
            },
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None
        };
        let result = expand_struct(&input, &data);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let output = tokens.to_string();
        assert!(output.contains("impl"));
        assert!(output.contains("core :: fmt :: Display"));
        assert!(output.contains("MyError"));
        assert!(output.contains("Ok (())"));
    }

    #[test]
    fn test_expand_struct_transparent_single_field() {
        let input = make_error_input("MyError");
        let field = make_test_field("inner", parse_quote!(String), 0);
        let data = StructData {
            fields:      Fields::Named(vec![field]),
            display:     DisplaySpec::Transparent {
                attribute: Box::new(parse_quote!(#[error(transparent)]))
            },
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None
        };
        let result = expand_struct(&input, &data);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let output = tokens.to_string();
        assert!(output.contains("core :: fmt :: Display :: fmt"));
        assert!(output.contains("inner"));
    }

    #[test]
    fn test_expand_struct_template() {
        let input = make_error_input("MyError");
        let data = StructData {
            fields:      Fields::Unit,
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error occurred".to_string())]
            }),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None
        };
        let result = expand_struct(&input, &data);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let output = tokens.to_string();
        assert!(output.contains("MyError"));
        assert!(output.contains("error occurred"));
    }

    #[test]
    fn test_expand_struct_formatter_path() {
        let input = make_error_input("MyError");
        let data = StructData {
            fields:      Fields::Unit,
            display:     DisplaySpec::FormatterPath {
                path: parse_quote!(custom_formatter),
                args: FormatArgsSpec::default()
            },
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None
        };
        let result = expand_struct(&input, &data);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let output = tokens.to_string();
        assert!(output.contains("custom_formatter"));
    }

    #[test]
    fn test_render_struct_transparent_unit() {
        let fields = Fields::Unit;
        let result = render_struct_transparent(&fields);
        let output = result.to_string();
        assert!(output.contains("Ok (())"));
    }

    #[test]
    fn test_render_struct_transparent_single_field() {
        let field = make_test_field("inner", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let result = render_struct_transparent(&fields);
        let output = result.to_string();
        assert!(output.contains("core :: fmt :: Display :: fmt"));
        assert!(output.contains("self"));
        assert!(output.contains("inner"));
    }

    #[test]
    fn test_render_struct_transparent_unnamed_field() {
        let field = make_test_unnamed_field(parse_quote!(i32), 0);
        let fields = Fields::Unnamed(vec![field]);
        let result = render_struct_transparent(&fields);
        let output = result.to_string();
        assert!(output.contains("core :: fmt :: Display :: fmt"));
        assert!(output.contains("self"));
        assert!(output.contains("0"));
    }

    #[test]
    fn test_struct_formatter_arguments_unit() {
        let fields = Fields::Unit;
        let result = struct_formatter_arguments(&fields);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_struct_formatter_arguments_named_fields() {
        let field1 = make_test_field("name", parse_quote!(String), 0);
        let field2 = make_test_field("value", parse_quote!(i32), 1);
        let fields = Fields::Named(vec![field1, field2]);
        let result = struct_formatter_arguments(&fields);
        assert_eq!(result.len(), 2);
        assert!(result[0].to_string().contains("self"));
        assert!(result[0].to_string().contains("name"));
        assert!(result[1].to_string().contains("self"));
        assert!(result[1].to_string().contains("value"));
    }

    #[test]
    fn test_struct_formatter_arguments_unnamed_fields() {
        let field1 = make_test_unnamed_field(parse_quote!(String), 0);
        let field2 = make_test_unnamed_field(parse_quote!(i32), 1);
        let fields = Fields::Unnamed(vec![field1, field2]);
        let result = struct_formatter_arguments(&fields);
        assert_eq!(result.len(), 2);
        assert!(result[0].to_string().contains("self"));
        assert!(result[0].to_string().contains("0"));
        assert!(result[1].to_string().contains("1"));
    }

    #[test]
    fn test_formatter_path_call_no_args() {
        let path: syn::ExprPath = parse_quote!(my_formatter);
        let args = vec![];
        let result = formatter_path_call(&path, args);
        let output = result.to_string();
        assert!(output.contains("my_formatter"));
        assert!(output.contains("f"));
    }

    #[test]
    fn test_formatter_path_call_with_args() {
        let path: syn::ExprPath = parse_quote!(my_formatter);
        let args = vec![quote::quote!(arg1), quote::quote!(arg2)];
        let result = formatter_path_call(&path, args);
        let output = result.to_string();
        assert!(output.contains("my_formatter"));
        assert!(output.contains("arg1"));
        assert!(output.contains("arg2"));
        assert!(output.contains("f"));
    }

    #[test]
    fn test_render_struct_formatter_path_unit() {
        let fields = Fields::Unit;
        let path: syn::ExprPath = parse_quote!(custom_fmt);
        let result = render_struct_formatter_path(&fields, &path);
        let output = result.to_string();
        assert!(output.contains("custom_fmt"));
        assert!(output.contains("f"));
    }

    #[test]
    fn test_render_struct_formatter_path_with_fields() {
        let field = make_test_field("message", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let path: syn::ExprPath = parse_quote!(custom_fmt);
        let result = render_struct_formatter_path(&fields, &path);
        let output = result.to_string();
        assert!(output.contains("custom_fmt"));
        assert!(output.contains("message"));
    }

    #[test]
    fn test_struct_placeholder_expr_self() {
        use masterror_template::template::TemplateFormatter;

        use crate::template_support::{TemplateIdentifierSpec, TemplatePlaceholderSpec};
        let fields = Fields::Unit;
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("self".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = struct_placeholder_expr(&fields, &placeholder, None);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert!(resolved.expr.to_string().contains("self"));
    }

    #[test]
    fn test_struct_placeholder_expr_named_field() {
        use masterror_template::template::TemplateFormatter;

        use crate::template_support::{TemplateIdentifierSpec, TemplatePlaceholderSpec};
        let field = make_test_field("message", parse_quote!(String), 0);
        let fields = Fields::Named(vec![field]);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("message".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = struct_placeholder_expr(&fields, &placeholder, None);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert!(resolved.expr.to_string().contains("self"));
        assert!(resolved.expr.to_string().contains("message"));
    }

    #[test]
    fn test_struct_placeholder_expr_unknown_named_field() {
        use masterror_template::template::TemplateFormatter;

        use crate::template_support::{TemplateIdentifierSpec, TemplatePlaceholderSpec};
        let fields = Fields::Unit;
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("unknown".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = struct_placeholder_expr(&fields, &placeholder, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_struct_placeholder_expr_positional() {
        use masterror_template::template::TemplateFormatter;

        use crate::template_support::{TemplateIdentifierSpec, TemplatePlaceholderSpec};
        let field = make_test_unnamed_field(parse_quote!(i32), 0);
        let fields = Fields::Unnamed(vec![field]);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = struct_placeholder_expr(&fields, &placeholder, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_struct_placeholder_expr_positional_out_of_bounds() {
        use masterror_template::template::TemplateFormatter;

        use crate::template_support::{TemplateIdentifierSpec, TemplatePlaceholderSpec};
        let fields = Fields::Unit;
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(5),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = struct_placeholder_expr(&fields, &placeholder, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_struct_placeholder_expr_implicit() {
        use masterror_template::template::TemplateFormatter;

        use crate::template_support::{TemplateIdentifierSpec, TemplatePlaceholderSpec};
        let field = make_test_unnamed_field(parse_quote!(String), 0);
        let fields = Fields::Unnamed(vec![field]);
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Implicit(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = struct_placeholder_expr(&fields, &placeholder, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_struct_field_expr_with_display() {
        use masterror_template::template::TemplateFormatter;
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
    fn test_struct_field_expr_with_pointer() {
        use masterror_template::template::TemplateFormatter;
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
    fn test_struct_field_expr_with_reference() {
        use masterror_template::template::TemplateFormatter;
        let field = make_test_field("ref_val", parse_quote!(&str), 0);
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        let result = struct_field_expr(&field, &formatter);
        assert!(result.pointer_value);
    }

    #[test]
    fn test_binding_ident_named_field() {
        let field = make_test_field("message", parse_quote!(String), 0);
        let ident = binding_ident(&field);
        assert_eq!(ident.to_string(), "message");
    }

    #[test]
    fn test_binding_ident_unnamed_field() {
        let field = make_test_unnamed_field(parse_quote!(i32), 5);
        let ident = binding_ident(&field);
        assert_eq!(ident.to_string(), "__field5");
    }

    #[test]
    fn test_binding_ident_multiple_unnamed_fields() {
        let field0 = make_test_unnamed_field(parse_quote!(String), 0);
        let field1 = make_test_unnamed_field(parse_quote!(i32), 1);
        let ident0 = binding_ident(&field0);
        let ident1 = binding_ident(&field1);
        assert_eq!(ident0.to_string(), "__field0");
        assert_eq!(ident1.to_string(), "__field1");
    }

    #[test]
    fn test_expand_struct_with_generics() {
        let mut input = make_error_input("MyError");
        input.generics = parse_quote!(<T>);
        let data = StructData {
            fields:      Fields::Unit,
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("generic error".to_string())]
            }),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None
        };
        let result = expand_struct(&input, &data);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let output = tokens.to_string();
        assert!(output.contains("MyError"));
        assert!(output.contains("< T >"));
    }

    #[test]
    fn test_struct_formatter_arguments_preserves_order() {
        let field1 = make_test_field("first", parse_quote!(String), 0);
        let field2 = make_test_field("second", parse_quote!(i32), 1);
        let field3 = make_test_field("third", parse_quote!(bool), 2);
        let fields = Fields::Named(vec![field1, field2, field3]);
        let result = struct_formatter_arguments(&fields);
        assert_eq!(result.len(), 3);
        assert!(result[0].to_string().contains("first"));
        assert!(result[1].to_string().contains("second"));
        assert!(result[2].to_string().contains("third"));
    }

    #[test]
    fn test_struct_field_expr_unnamed_field() {
        use masterror_template::template::TemplateFormatter;
        let field = make_test_unnamed_field(parse_quote!(String), 0);
        let formatter = TemplateFormatter::Display {
            spec: None
        };
        let result = struct_field_expr(&field, &formatter);
        assert!(!result.pointer_value);
        assert!(result.expr.to_string().contains("self"));
        assert!(result.expr.to_string().contains("0"));
    }
}
