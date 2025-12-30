// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion trait implementations for error types.
//!
//! This module generates `From` trait implementations that convert error types
//! into `masterror::Error`. It handles both struct and enum variants,
//! supporting:
//!
//! - Message initialization from Display implementations
//! - Field destructuring and binding
//! - Code and category assignment
//! - Integration with attachment and metadata systems
//!
//! The conversion process ensures all error information is properly transferred
//! while respecting privacy settings and redaction policies.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Error;

use super::{
    attachment::{
        backtrace_attachment_tokens, metadata_attach_tokens, redact_tokens,
        source_attachment_tokens, telemetry_initialization
    },
    binding::{bind_struct_fields, bind_variant_fields, field_usage_tokens}
};
use crate::input::{ErrorInput, MasterrorSpec, StructData, VariantData};

/// Generates From trait implementation for struct error types.
///
/// Creates a conversion from the struct type to `masterror::Error`,
/// incorporating all configured error attributes including code, category,
/// message exposure, telemetry data, and field attachments.
///
/// # Arguments
///
/// * `input` - The parsed error type definition
/// * `data` - Struct-specific data and fields
/// * `spec` - Masterror specification with code, category, and options
///
/// # Returns
///
/// A `TokenStream` containing the From trait implementation.
///
/// # Examples
///
/// ```ignore
/// #[derive(Masterror)]
/// #[masterror(code = "AUTH_001", category = ErrorCategory::Authentication)]
/// struct AuthError {
///     message: String,
/// }
/// // Generates: impl From<AuthError> for masterror::Error { ... }
/// ```
pub fn struct_conversion_impl(
    input: &ErrorInput,
    data: &StructData,
    spec: &MasterrorSpec
) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let code = &spec.code;
    let category = &spec.category;
    let message_init = message_initialization(spec.expose_message, quote!(&value));
    let (destructure, bound_fields) = bind_struct_fields(ident, &data.fields);
    let field_usage = field_usage_tokens(&bound_fields);
    let telemetry_init = telemetry_initialization(&spec.telemetry);
    let metadata_attach = metadata_attach_tokens();
    let redact_tokens = redact_tokens(&spec.redact);
    let source_tokens = source_attachment_tokens(&bound_fields);
    let backtrace_tokens = backtrace_attachment_tokens(&data.fields, &bound_fields);
    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::Error #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #message_init
                #destructure
                #field_usage
                #telemetry_init
                let mut __masterror_error = match __masterror_message {
                    Some(message) => masterror::Error::with((#category), message),
                    None => masterror::Error::bare((#category))
                };
                __masterror_error = __masterror_error.with_code((#code));
                #metadata_attach
                #redact_tokens
                #source_tokens
                #backtrace_tokens
                __masterror_error
            }
        }
    }
}

/// Generates From trait implementation for enum error types.
///
/// Creates a conversion from the enum type to `masterror::Error` with separate
/// match arms for each variant. Each variant can have its own code, category,
/// and attachment configuration.
///
/// # Arguments
///
/// * `input` - The parsed error type definition
/// * `variants` - List of enum variants with their specifications
///
/// # Returns
///
/// A `TokenStream` containing the From trait implementation with match arms.
///
/// # Examples
///
/// ```ignore
/// #[derive(Masterror)]
/// enum AppError {
///     #[masterror(code = "IO_001", category = ErrorCategory::Internal)]
///     Io(std::io::Error),
///     #[masterror(code = "AUTH_001", category = ErrorCategory::Authentication)]
///     Auth(String),
/// }
/// // Generates: impl From<AppError> for masterror::Error with match arms
/// ```
pub fn enum_conversion_impl(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut arms = Vec::new();
    let mut message_arms = Vec::new();
    for variant in variants {
        let spec = variant.masterror.as_ref().expect("presence checked");
        let code = &spec.code;
        let category = &spec.category;
        let (pattern, bound_fields) = bind_variant_fields(ident, variant);
        let field_usage = field_usage_tokens(&bound_fields);
        let telemetry_init = telemetry_initialization(&spec.telemetry);
        let metadata_attach = metadata_attach_tokens();
        let redact_tokens = redact_tokens(&spec.redact);
        let source_tokens = source_attachment_tokens(&bound_fields);
        let backtrace_tokens = backtrace_attachment_tokens(&variant.fields, &bound_fields);
        message_arms.push(enum_message_arm(ident, variant, spec.expose_message));
        arms.push(quote! {
            #pattern => {
                #field_usage
                #telemetry_init
                let mut __masterror_error = match __masterror_message {
                    Some(message) => masterror::Error::with((#category), message),
                    None => masterror::Error::bare((#category))
                };
                __masterror_error = __masterror_error.with_code((#code));
                #metadata_attach
                #redact_tokens
                #source_tokens
                #backtrace_tokens
                __masterror_error
            }
        });
    }
    let message_match = quote! {
        let __masterror_message: Option<String> = match &value {
            #(#message_arms)*
        };
    };
    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::Error #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #message_match
                match value {
                    #(#arms),*
                }
            }
        }
    }
}

/// Generates message initialization code based on expose_message setting.
///
/// When message exposure is enabled, converts the error value to a String using
/// the Display trait. Otherwise, initializes message as None.
///
/// # Arguments
///
/// * `enabled` - Whether to expose the error message
/// * `value` - TokenStream representing the error value to convert
///
/// # Returns
///
/// A `TokenStream` that initializes `__masterror_message` variable.
///
/// # Examples
///
/// ```ignore
/// // With expose_message = true:
/// let __masterror_message = Some(std::string::ToString::to_string(&value));
///
/// // With expose_message = false:
/// let __masterror_message: Option<String> = None;
/// ```
pub fn message_initialization(enabled: bool, value: TokenStream) -> TokenStream {
    if enabled {
        quote! {
            let __masterror_message = Some(std::string::ToString::to_string(#value));
        }
    } else {
        quote! {
            let __masterror_message: Option<String> = None;
        }
    }
}

/// Validates that all enum variants have masterror attributes.
///
/// This is a requirement for deriving Masterror on enums - every variant
/// must specify its code and category.
///
/// # Arguments
///
/// * `variants` - List of enum variants to validate
///
/// # Errors
///
/// Returns an error if any variant is missing the #[masterror(...)] attribute.
///
/// # Examples
///
/// ```ignore
/// // Valid:
/// enum MyError {
///     #[masterror(code = "E001", category = ErrorCategory::Internal)]
///     Variant1,
/// }
///
/// // Invalid - will return error:
/// enum MyError {
///     Variant1, // Missing #[masterror(...)]
/// }
/// ```
pub fn ensure_all_variants_have_masterror(variants: &[VariantData]) -> Result<(), Error> {
    for variant in variants {
        if variant.masterror.is_none() {
            return Err(Error::new(
                variant.span,
                "all variants must use #[masterror(...)] to derive masterror::Error conversion"
            ));
        }
    }
    Ok(())
}

/// Generates a match arm for message extraction from enum variant.
///
/// Creates pattern matching code to extract Display string from enum variants
/// when expose_message is enabled.
///
/// # Arguments
///
/// * `enum_ident` - The enum type identifier
/// * `variant` - The specific variant data
/// * `expose_message` - Whether to expose the message
///
/// # Returns
///
/// A `TokenStream` containing the match arm for message extraction.
fn enum_message_arm(
    enum_ident: &Ident,
    variant: &VariantData,
    expose_message: bool
) -> TokenStream {
    use quote::format_ident;
    if expose_message {
        let binding = format_ident!("__masterror_variant_ref");
        let pattern = enum_message_pattern(enum_ident, variant, Some(&binding));
        quote! {
            #pattern => Some(std::string::ToString::to_string(#binding)),
        }
    } else {
        let pattern = enum_message_pattern(enum_ident, variant, None);
        quote! {
            #pattern => None,
        }
    }
}

/// Generates pattern for message extraction from enum variant.
///
/// Creates the appropriate pattern syntax based on variant field structure
/// (unit, named, or unnamed) and whether a binding is needed.
///
/// # Arguments
///
/// * `enum_ident` - The enum type identifier
/// * `variant` - The specific variant data
/// * `binding` - Optional binding identifier for the pattern
///
/// # Returns
///
/// A `TokenStream` containing the match pattern.
fn enum_message_pattern(
    enum_ident: &Ident,
    variant: &VariantData,
    binding: Option<&Ident>
) -> TokenStream {
    use crate::input::Fields;
    let variant_ident = &variant.ident;
    match (&variant.fields, binding) {
        (Fields::Unit, Some(binding)) => quote!(#binding @ #enum_ident::#variant_ident),
        (Fields::Unit, None) => quote!(#enum_ident::#variant_ident),
        (Fields::Named(_), Some(binding)) => quote!(#binding @ #enum_ident::#variant_ident { .. }),
        (Fields::Named(_), None) => quote!(#enum_ident::#variant_ident { .. }),
        (Fields::Unnamed(_), Some(binding)) => quote!(#binding @ #enum_ident::#variant_ident(..)),
        (Fields::Unnamed(_), None) => quote!(#enum_ident::#variant_ident(..))
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_message_initialization_enabled() {
        let result = message_initialization(true, quote!(&value));
        let expected = quote! {
            let __masterror_message = Some(std::string::ToString::to_string(&value));
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_message_initialization_disabled() {
        let result = message_initialization(false, quote!(&value));
        let expected = quote! {
            let __masterror_message: Option<String> = None;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    /// Tests with empty list since creating mock VariantData structures is
    /// complex.
    #[test]
    fn test_ensure_all_variants_have_masterror_valid() {
        let variants = vec![];
        assert!(ensure_all_variants_have_masterror(&variants).is_ok());
    }

    #[test]
    fn test_enum_message_pattern_unit() {
        use proc_macro2::Span;
        use quote::format_ident;

        use crate::input::{DisplaySpec, Fields};
        let variant = VariantData {
            ident:       format_ident!("NotFound"),
            fields:      Fields::Unit,
            display:     DisplaySpec::Template(crate::template_support::DisplayTemplate {
                segments: vec![]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let enum_ident = format_ident!("MyError");
        let result = enum_message_pattern(&enum_ident, &variant, None);
        let result_str = result.to_string();
        assert!(result_str.contains("MyError :: NotFound"));
    }

    #[test]
    fn test_enum_message_pattern_named() {
        use proc_macro2::Span;
        use quote::format_ident;
        use syn::parse_quote;

        use crate::input::{DisplaySpec, Field, FieldAttrs, Fields};
        let field = Field {
            ident:  Some(format_ident!("message")),
            member: syn::Member::Named(format_ident!("message")),
            ty:     parse_quote!(String),
            index:  0,
            attrs:  FieldAttrs::default(),
            span:   Span::call_site()
        };
        let variant = VariantData {
            ident:       format_ident!("Custom"),
            fields:      Fields::Named(vec![field]),
            display:     DisplaySpec::Template(crate::template_support::DisplayTemplate {
                segments: vec![]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let enum_ident = format_ident!("MyError");
        let result = enum_message_pattern(&enum_ident, &variant, None);
        let result_str = result.to_string();
        assert!(result_str.contains("MyError :: Custom"));
        assert!(result_str.contains(".."));
    }
}
