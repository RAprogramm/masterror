// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Error trait implementation code generation.
//!
//! This module provides the main entry point for generating `std::error::Error`
//! trait implementations for custom error types. It supports both struct and
//! enum error types, generating appropriate implementations for the `source()`
//! and `provide()` methods based on the error's structure and attributes.
//!
//! # Architecture
//!
//! The error trait implementation is split into focused submodules:
//!
//! - [`source`] - Handles `source()` method generation for error cause chains
//! - [`provide`] - Handles `provide()` method generation for generic member
//!   access API
//! - [`binding`] - Utilities for generating field binding identifiers in
//!   patterns
//!
//! # Main Entry Point
//!
//! The [`expand`] function is the primary interface for generating Error trait
//! implementations. It takes parsed error input and produces the complete trait
//! implementation.
//!
//! # Examples
//!
//! ```rust,ignore
//! use crate::error_trait::expand;
//! use crate::input::ErrorInput;
//!
//! let input: ErrorInput = /* parsed from macro input */;
//! let tokens = expand(&input)?;
//! // tokens contains: impl std::error::Error for MyError { ... }
//! ```
//!
//! # Supported Features
//!
//! - **Source chaining**: Automatic delegation to underlying error causes
//! - **Backtrace capture**: Direct storage or delegation to source errors
//! - **Provide API**: Generic member access for additional error context
//! - **Transparent delegation**: Newtype pattern support for error wrapping
//! - **Option handling**: Automatic unwrapping for `Option<E>` fields

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    input::{ErrorData, ErrorInput, StructData, VariantData},
    lint::lifetime_lint_allows
};

pub mod binding;
pub mod provide;
pub mod source;

use provide::{enum_provide_method, struct_provide_method};
use source::{struct_source_body, variant_source_arm};

/// Generates Error trait implementation for an error type.
///
/// Dispatches to struct or enum-specific implementations based on the input
/// data structure. Generates complete trait impl including source and
/// provide methods as appropriate.
///
/// # Arguments
///
/// * `input` - The parsed error type definition
///
/// # Returns
///
/// Token stream containing the Error trait implementation
pub fn expand(input: &ErrorInput) -> TokenStream {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}

/// Generates Error trait implementation for struct error types.
///
/// Creates implementation with source and provide methods
/// based on field attributes and display specification.
///
/// # Arguments
///
/// * `input` - The error type definition
/// * `data` - The struct-specific data
///
/// # Returns
///
/// Token stream for struct Error trait impl
fn expand_struct(input: &ErrorInput, data: &StructData) -> TokenStream {
    let body = struct_source_body(&data.fields, &data.display);
    let provide_method = struct_provide_method(&data.fields).unwrap_or_default();
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let lint_allows = lifetime_lint_allows(&input.generics);
    quote! {
        #lint_allows
        impl #impl_generics std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                #body
            }
            #provide_method
        }
    }
}

/// Generates Error trait implementation for enum error types.
///
/// Creates implementation with pattern matching for source and provide
/// methods across all variants.
///
/// # Arguments
///
/// * `input` - The error type definition
/// * `variants` - The enum variants
///
/// # Returns
///
/// Token stream for enum Error trait impl
fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let mut arms = Vec::new();
    for variant in variants {
        arms.push(variant_source_arm(variant));
    }
    let provide_method = enum_provide_method(variants).unwrap_or_default();
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let lint_allows = lifetime_lint_allows(&input.generics);
    quote! {
        #lint_allows
        impl #impl_generics std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #(#arms),*
                }
            }
            #provide_method
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::parse_quote;

    use super::*;
    use crate::{
        input::{DisplaySpec, Fields, FormatArgsSpec},
        template_support::{DisplayTemplate, TemplateSegmentSpec}
    };

    fn make_simple_error_input(name: &str) -> ErrorInput {
        ErrorInput {
            ident:    syn::Ident::new(name, Span::call_site()),
            generics: parse_quote!(),
            data:     ErrorData::Struct(Box::new(StructData {
                fields:      Fields::Unit,
                display:     DisplaySpec::Template(DisplayTemplate {
                    segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
                }),
                masterror:   None,
                format_args: FormatArgsSpec::default(),
                app_error:   None
            }))
        }
    }

    #[test]
    fn test_expand_struct() {
        let input = make_simple_error_input("MyError");
        let tokens = expand(&input);
        let output = tokens.to_string();
        assert!(output.contains("impl"));
        assert!(output.contains("std :: error :: Error"));
        assert!(output.contains("MyError"));
    }

    #[test]
    fn test_expand_enum() {
        let variant = VariantData {
            ident:       syn::Ident::new("Variant", Span::call_site()),
            fields:      Fields::Unit,
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let input = ErrorInput {
            ident:    syn::Ident::new("MyError", Span::call_site()),
            generics: parse_quote!(),
            data:     ErrorData::Enum(vec![variant])
        };
        let tokens = expand(&input);
        let output = tokens.to_string();
        assert!(output.contains("impl"));
        assert!(output.contains("match self"));
    }

    #[test]
    fn test_expand_struct_generates_source_method() {
        let input = make_simple_error_input("TestError");
        let output = expand(&input).to_string();
        assert!(output.contains("fn source"));
    }
}
