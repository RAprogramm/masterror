// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Display implementation generation for enum error types.
//!
//! This module handles the generation of `Display` trait implementations for
//! enum-based error types. It creates a match expression with an arm for each
//! variant, supporting transparent delegation, template-based formatting, and
//! custom formatter functions per variant.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Error;

use super::{
    format_args::FormatArgumentsEnv,
    formatter::needs_pointer_value,
    placeholder::ResolvedPlaceholderExpr,
    struct_impl::{binding_ident, formatter_path_call},
    template::render_template
};
use crate::{
    input::{
        DisplaySpec, ErrorInput, Field, Fields, FormatArgsSpec, VariantData, placeholder_error
    },
    template_support::{DisplayTemplate, TemplateIdentifierSpec}
};

/// Generates the Display trait implementation for an enum error type.
///
/// Creates a Display implementation with a match expression that handles
/// each variant according to its display specification.
///
/// # Arguments
///
/// * `input` - The error type input with generics and metadata
/// * `variants` - The enum variants with their display specifications
///
/// # Returns
///
/// Token stream containing the complete Display trait implementation
pub fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<TokenStream, Error> {
    let mut arms = Vec::new();

    for variant in variants {
        arms.push(render_variant(variant)?);
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}

/// Renders a single match arm for an enum variant.
///
/// Dispatches to the appropriate rendering function based on the variant's
/// display specification (transparent, template, or formatter path).
///
/// # Arguments
///
/// * `variant` - The variant data with display specification
///
/// # Returns
///
/// Token stream containing the match arm for this variant
pub fn render_variant(variant: &VariantData) -> Result<TokenStream, Error> {
    match &variant.display {
        DisplaySpec::Transparent {
            ..
        } => render_variant_transparent(variant),
        DisplaySpec::Template(template) => render_variant_template(variant, template, None),
        DisplaySpec::TemplateWithArgs {
            template,
            args
        } => render_variant_template(variant, template, Some(args)),
        DisplaySpec::FormatterPath {
            path, ..
        } => render_variant_formatter_path(variant, path)
    }
}

/// Renders a transparent display match arm for a variant.
///
/// For transparent variants, the Display implementation delegates to the
/// single field. Returns an error if the variant doesn't have exactly one
/// field.
///
/// # Arguments
///
/// * `variant` - The variant data
///
/// # Returns
///
/// Token stream containing the match arm with Display delegation
pub fn render_variant_transparent(variant: &VariantData) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;

    match &variant.fields {
        Fields::Unit => Err(Error::new(
            variant.span,
            "#[error(transparent)] requires exactly one field"
        )),
        Fields::Named(fields) | Fields::Unnamed(fields) => {
            if fields.len() != 1 {
                return Err(Error::new(
                    variant.span,
                    "#[error(transparent)] requires exactly one field"
                ));
            }

            let binding = binding_ident(&fields[0]);
            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field_ident = fields[0].ident.clone().expect("named field");
                    quote!(Self::#variant_ident { #field_ident: #binding })
                }
                Fields::Unnamed(_) => {
                    quote!(Self::#variant_ident(#binding))
                }
                Fields::Unit => unreachable!()
            };

            Ok(quote! {
                #pattern => core::fmt::Display::fmt(#binding, f)
            })
        }
    }
}

/// Renders a match arm for a variant using a custom formatter function.
///
/// # Arguments
///
/// * `variant` - The variant data
/// * `path` - The path to the custom formatter function
///
/// # Returns
///
/// Token stream containing the match arm with formatter function call
pub fn render_variant_formatter_path(
    variant: &VariantData,
    path: &syn::ExprPath
) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            let call = formatter_path_call(path, Vec::new());
            Ok(quote! {
                Self::#variant_ident => {
                    #call
                }
            })
        }
        Fields::Unnamed(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let pattern = quote!(Self::#variant_ident(#(#bindings),*));
            let call = formatter_path_call(path, variant_formatter_arguments(&bindings));
            Ok(quote! {
                #pattern => {
                    #call
                }
            })
        }
        Fields::Named(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let pattern = quote!(Self::#variant_ident { #(#bindings),* });
            let call = formatter_path_call(path, variant_formatter_arguments(&bindings));
            Ok(quote! {
                #pattern => {
                    #call
                }
            })
        }
    }
}

/// Generates argument expressions for variant formatter function calls.
///
/// Creates expressions from variant field bindings to pass as arguments
/// to a custom formatter function.
///
/// # Arguments
///
/// * `bindings` - The binding identifiers from the variant pattern
///
/// # Returns
///
/// Vector of token streams representing the binding references
pub fn variant_formatter_arguments(bindings: &[Ident]) -> Vec<TokenStream> {
    bindings.iter().map(|binding| quote!(#binding)).collect()
}

/// Renders a match arm for a variant using a template.
///
/// Processes the template with the variant's fields and format arguments,
/// generating the appropriate formatting code.
///
/// # Arguments
///
/// * `variant` - The variant data
/// * `template` - The display template to render
/// * `format_args` - Optional format arguments specification
///
/// # Returns
///
/// Token stream containing the match arm with template rendering
pub fn render_variant_template(
    variant: &VariantData,
    template: &DisplayTemplate,
    format_args: Option<&FormatArgsSpec>
) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            let mut env = format_args
                .map(|args| FormatArgumentsEnv::new_variant(args, &variant.fields, &[]));
            let preludes = env
                .as_ref()
                .map(|env| env.prelude_tokens())
                .unwrap_or_default();
            let format_arguments = if let Some(env) = env.as_ref() {
                env.argument_tokens()?
            } else {
                Vec::new()
            };
            let span = variant.span;
            let body = render_template(template, preludes, format_arguments, |placeholder| {
                if let Some(env) = env.as_mut()
                    && let Some(resolved) = env.resolve_placeholder(placeholder)?
                {
                    return Ok(resolved);
                }
                Err(Error::new(span, "unit variants cannot reference fields"))
            })?;
            Ok(quote! {
                Self::#variant_ident => {
                    #body
                }
            })
        }
        Fields::Unnamed(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let mut env = format_args
                .map(|args| FormatArgumentsEnv::new_variant(args, &variant.fields, &bindings));
            let pattern = quote!(Self::#variant_ident(#(#bindings),*));
            let preludes = env
                .as_ref()
                .map(|env| env.prelude_tokens())
                .unwrap_or_default();
            let format_arguments = if let Some(env) = env.as_ref() {
                env.argument_tokens()?
            } else {
                Vec::new()
            };
            let body = render_template(template, preludes, format_arguments, |placeholder| {
                variant_tuple_placeholder(&bindings, placeholder, env.as_mut())
            })?;
            Ok(quote! {
                #pattern => {
                    #body
                }
            })
        }
        Fields::Named(fields) => {
            let bindings: Vec<_> = fields
                .iter()
                .map(|field| field.ident.clone().expect("named field"))
                .collect();
            let mut env = format_args
                .map(|args| FormatArgumentsEnv::new_variant(args, &variant.fields, &bindings));
            let pattern = quote!(Self::#variant_ident { #(#bindings),* });
            let preludes = env
                .as_ref()
                .map(|env| env.prelude_tokens())
                .unwrap_or_default();
            let format_arguments = if let Some(env) = env.as_ref() {
                env.argument_tokens()?
            } else {
                Vec::new()
            };
            let body = render_template(template, preludes, format_arguments, |placeholder| {
                variant_named_placeholder(fields, &bindings, placeholder, env.as_mut())
            })?;
            Ok(quote! {
                #pattern => {
                    #body
                }
            })
        }
    }
}

/// Resolves a placeholder for a tuple variant.
///
/// For tuple variants, placeholders can reference fields by position or use
/// format arguments. Named field access is not supported for tuple variants.
///
/// # Arguments
///
/// * `bindings` - The binding identifiers from the variant pattern
/// * `placeholder` - The placeholder specification to resolve
/// * `env` - Optional format arguments environment
///
/// # Returns
///
/// Resolved placeholder expression, or an error if resolution fails
pub fn variant_tuple_placeholder(
    bindings: &[Ident],
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
        TemplateIdentifierSpec::Named(_) => {
            Err(placeholder_error(placeholder.span, &placeholder.identifier))
        }
        TemplateIdentifierSpec::Positional(index) => bindings
            .get(*index)
            .map(|binding| {
                ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(&placeholder.formatter)
                )
            })
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier)),
        TemplateIdentifierSpec::Implicit(index) => bindings
            .get(*index)
            .map(|binding| {
                ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(&placeholder.formatter)
                )
            })
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

/// Resolves a placeholder for a named variant.
///
/// For named variants, placeholders can reference fields by name or use
/// format arguments. Positional access is not supported for named variants.
///
/// # Arguments
///
/// * `fields` - The variant's fields for name lookup
/// * `bindings` - The binding identifiers from the variant pattern
/// * `placeholder` - The placeholder specification to resolve
/// * `env` - Optional format arguments environment
///
/// # Returns
///
/// Resolved placeholder expression, or an error if resolution fails
pub fn variant_named_placeholder(
    fields: &[Field],
    bindings: &[Ident],
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
            if let Some(index) = fields
                .iter()
                .position(|field| field.ident.as_ref().is_some_and(|ident| ident == name))
            {
                let binding = &bindings[index];
                Ok(ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(&placeholder.formatter)
                ))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => Err(placeholder_error(
            placeholder.span,
            &TemplateIdentifierSpec::Positional(*index)
        )),
        TemplateIdentifierSpec::Implicit(index) => Err(placeholder_error(
            placeholder.span,
            &TemplateIdentifierSpec::Implicit(*index)
        ))
    }
}
