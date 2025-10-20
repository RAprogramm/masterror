// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Protocol mapping implementations for error types.
//!
//! This module generates constant mappings for converting errors to various
//! protocols and formats:
//!
//! - HTTP status codes and categories
//! - gRPC status codes
//! - Problem JSON (RFC 7807) types
//!
//! For struct types, it generates single mapping constants. For enum types,
//! it generates arrays of mappings corresponding to each variant.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprPath, Index};

use crate::input::{ErrorInput, MasterrorSpec, VariantData};

/// Generates protocol mapping constants for struct error types.
///
/// Creates three const items on the error type:
/// - `HTTP_MAPPING` - Always present, maps to HTTP status
/// - `GRPC_MAPPING` - Optional, maps to gRPC status code
/// - `PROBLEM_MAPPING` - Optional, maps to Problem JSON type
///
/// # Arguments
///
/// * `input` - The parsed error type definition
/// * `spec` - Masterror specification with mapping configurations
///
/// # Returns
///
/// A `TokenStream` containing the impl block with mapping constants.
///
/// # Examples
///
/// ```ignore
/// #[derive(Masterror)]
/// #[masterror(
///     code = "AUTH_001",
///     category = ErrorCategory::Authentication,
///     map_grpc = "tonic::Code::Unauthenticated"
/// )]
/// struct AuthError;
///
/// // Generates:
/// impl AuthError {
///     pub const HTTP_MAPPING: masterror::mapping::HttpMapping = ...;
///     pub const GRPC_MAPPING: Option<masterror::mapping::GrpcMapping> = Some(...);
///     pub const PROBLEM_MAPPING: Option<masterror::mapping::ProblemMapping> = None;
/// }
/// ```
pub fn struct_mapping_impl(input: &ErrorInput, spec: &MasterrorSpec) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let code = &spec.code;
    let category = &spec.category;
    let grpc_mapping =
        mapping_option_tokens(spec.map_grpc.as_ref(), code, category, MappingKind::Grpc);
    let problem_mapping = mapping_option_tokens(
        spec.map_problem.as_ref(),
        code,
        category,
        MappingKind::Problem
    );

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// HTTP mapping for this error type.
            pub const HTTP_MAPPING: masterror::mapping::HttpMapping =
                masterror::mapping::HttpMapping::new((#code), (#category));

            /// gRPC mapping for this error type.
            pub const GRPC_MAPPING: Option<masterror::mapping::GrpcMapping> = #grpc_mapping;

            /// Problem JSON mapping for this error type.
            pub const PROBLEM_MAPPING: Option<masterror::mapping::ProblemMapping> = #problem_mapping;
        }
    }
}

/// Generates protocol mapping constants for enum error types.
///
/// Creates three const items on the error type:
/// - `HTTP_MAPPINGS` - Array of mappings for all variants
/// - `GRPC_MAPPINGS` - Slice of mappings for variants with gRPC config
/// - `PROBLEM_MAPPINGS` - Slice of mappings for variants with Problem config
///
/// # Arguments
///
/// * `input` - The parsed error type definition
/// * `variants` - List of enum variants with their specifications
///
/// # Returns
///
/// A `TokenStream` containing the impl block with mapping constants.
///
/// # Examples
///
/// ```ignore
/// #[derive(Masterror)]
/// enum AppError {
///     #[masterror(code = "IO_001", category = ErrorCategory::Internal)]
///     Io(std::io::Error),
///     #[masterror(
///         code = "AUTH_001",
///         category = ErrorCategory::Authentication,
///         map_grpc = "tonic::Code::Unauthenticated"
///     )]
///     Auth,
/// }
///
/// // Generates:
/// impl AppError {
///     pub const HTTP_MAPPINGS: [HttpMapping; 2] = [...];
///     pub const GRPC_MAPPINGS: &'static [GrpcMapping] = &[...]; // Only Auth variant
///     pub const PROBLEM_MAPPINGS: &'static [ProblemMapping] = &[];
/// }
/// ```
pub fn enum_mapping_impl(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let http_entries: Vec<_> = variants
        .iter()
        .map(|variant| {
            let spec = variant.masterror.as_ref().expect("presence checked");
            let code = &spec.code;
            let category = &spec.category;
            quote!(masterror::mapping::HttpMapping::new((#code), (#category)))
        })
        .collect();

    let grpc_entries: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let spec = variant.masterror.as_ref().expect("presence checked");
            let code = &spec.code;
            let category = &spec.category;
            spec.map_grpc.as_ref().map(
                |expr| quote!(masterror::mapping::GrpcMapping::new((#code), (#category), (#expr)))
            )
        })
        .collect();

    let problem_entries: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let spec = variant.masterror.as_ref().expect("presence checked");
            let code = &spec.code;
            let category = &spec.category;
            spec.map_problem.as_ref().map(|expr| {
                quote!(masterror::mapping::ProblemMapping::new((#code), (#category), (#expr)))
            })
        })
        .collect();

    let http_len = Index::from(http_entries.len());

    let grpc_slice = if grpc_entries.is_empty() {
        quote!(&[] as &[masterror::mapping::GrpcMapping])
    } else {
        quote!(&[#(#grpc_entries),*])
    };

    let problem_slice = if problem_entries.is_empty() {
        quote!(&[] as &[masterror::mapping::ProblemMapping])
    } else {
        quote!(&[#(#problem_entries),*])
    };

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// HTTP mappings for enum variants.
            pub const HTTP_MAPPINGS: [masterror::mapping::HttpMapping; #http_len] = [#(#http_entries),*];

            /// gRPC mappings for enum variants.
            pub const GRPC_MAPPINGS: &'static [masterror::mapping::GrpcMapping] = #grpc_slice;

            /// Problem JSON mappings for enum variants.
            pub const PROBLEM_MAPPINGS: &'static [masterror::mapping::ProblemMapping] = #problem_slice;
        }
    }
}

/// Represents the type of protocol mapping being generated.
#[derive(Clone, Copy)]
enum MappingKind {
    /// gRPC status code mapping
    Grpc,
    /// Problem JSON (RFC 7807) type mapping
    Problem
}

/// Generates Option tokens for a specific mapping type.
///
/// Creates either `Some(Mapping::new(...))` or `None` depending on whether
/// the mapping expression is provided.
///
/// # Arguments
///
/// * `expr` - Optional mapping expression from attribute
/// * `code` - Error code expression
/// * `category` - Error category path
/// * `kind` - Type of mapping (gRPC or Problem)
///
/// # Returns
///
/// A `TokenStream` containing the Option expression.
///
/// # Examples
///
/// ```ignore
/// // With mapping:
/// Some(masterror::mapping::GrpcMapping::new(code, category, tonic::Code::Internal))
///
/// // Without mapping:
/// None
/// ```
fn mapping_option_tokens(
    expr: Option<&Expr>,
    code: &Expr,
    category: &ExprPath,
    kind: MappingKind
) -> TokenStream {
    match expr {
        Some(value) => match kind {
            MappingKind::Grpc => {
                quote!(Some(masterror::mapping::GrpcMapping::new((#code), (#category), (#value))))
            }
            MappingKind::Problem => {
                quote!(Some(masterror::mapping::ProblemMapping::new((#code), (#category), (#value))))
            }
        },
        None => quote!(None)
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_mapping_option_tokens_grpc_some() {
        let expr: Expr = parse_quote!(tonic::Code::Internal);
        let code: Expr = parse_quote!("E001");
        let category: ExprPath = parse_quote!(ErrorCategory::Internal);

        let result = mapping_option_tokens(Some(&expr), &code, &category, MappingKind::Grpc);
        let result_str = result.to_string();

        assert!(result_str.contains("GrpcMapping"));
        assert!(result_str.contains("Some"));
    }

    #[test]
    fn test_mapping_option_tokens_problem_some() {
        let expr: Expr = parse_quote!("about:blank");
        let code: Expr = parse_quote!("E001");
        let category: ExprPath = parse_quote!(ErrorCategory::Internal);

        let result = mapping_option_tokens(Some(&expr), &code, &category, MappingKind::Problem);
        let result_str = result.to_string();

        assert!(result_str.contains("ProblemMapping"));
        assert!(result_str.contains("Some"));
    }

    #[test]
    fn test_mapping_option_tokens_none() {
        let code: Expr = parse_quote!("E001");
        let category: ExprPath = parse_quote!(ErrorCategory::Internal);

        let result = mapping_option_tokens(None, &code, &category, MappingKind::Grpc);
        assert_eq!(result.to_string(), "None");
    }

    #[test]
    fn test_mapping_option_tokens_problem_none() {
        let code: Expr = parse_quote!("E002");
        let category: ExprPath = parse_quote!(ErrorCategory::NotFound);

        let result = mapping_option_tokens(None, &code, &category, MappingKind::Problem);
        assert_eq!(result.to_string(), "None");
    }
}
