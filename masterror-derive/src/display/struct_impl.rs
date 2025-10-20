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
