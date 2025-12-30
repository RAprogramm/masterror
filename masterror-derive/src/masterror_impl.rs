// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Masterror derive macro implementation.
//!
//! This module provides the core implementation for the `#[derive(Masterror)]`
//! macro, which generates `From` trait implementations converting error types
//! into `masterror::Error` along with protocol mappings (HTTP, gRPC, Problem
//! JSON).
//!
//! # Architecture
//!
//! The implementation is split into specialized modules:
//!
//! - [`conversion`] - Core `From` trait implementation generation
//! - [`mapping`] - Protocol mapping constant generation
//! - [`binding`] - Field destructuring and binding logic
//! - [`attachment`] - Error context attachment (source, backtrace, metadata)
//!
//! # Example
//!
//! ```ignore
//! use masterror::Masterror;
//!
//! #[derive(Masterror)]
//! #[masterror(
//!     code = "AUTH_001",
//!     category = ErrorCategory::Authentication,
//!     map_grpc = "tonic::Code::Unauthenticated"
//! )]
//! struct AuthError {
//!     message: String,
//! }
//! ```

use proc_macro2::TokenStream;
use syn::Error;

use crate::input::{ErrorData, ErrorInput, StructData, VariantData};

pub mod attachment;
pub mod binding;
pub mod conversion;
pub mod mapping;

use conversion::{
    ensure_all_variants_have_masterror, enum_conversion_impl, struct_conversion_impl
};
use mapping::{enum_mapping_impl, struct_mapping_impl};

/// Main entry point for Masterror derive macro expansion.
///
/// Dispatches to struct or enum-specific implementations based on the input
/// type structure.
///
/// # Arguments
///
/// * `input` - The parsed error type definition
///
/// # Returns
///
/// A `TokenStream` containing generated `From` trait implementation and
/// mapping constants, or a compile error if validation fails.
///
/// # Errors
///
/// Returns an error if:
/// - A struct is missing the required `#[masterror(...)]` attribute
/// - An enum variant is missing the required `#[masterror(...)]` attribute
/// - Code or category values are invalid
pub fn expand(input: &ErrorInput) -> Result<TokenStream, Error> {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}

/// Expands derive macro for struct error types.
///
/// Generates both conversion and mapping implementations for struct types.
///
/// # Arguments
///
/// * `input` - The parsed error type definition
/// * `data` - Struct-specific data and fields
///
/// # Returns
///
/// Combined conversion and mapping implementations.
fn expand_struct(input: &ErrorInput, data: &StructData) -> Result<TokenStream, Error> {
    let spec = data.masterror.as_ref().ok_or_else(|| {
        Error::new(
            input.ident.span(),
            "#[derive(Masterror)] requires #[masterror(...)] on structs"
        )
    })?;
    let conversion = struct_conversion_impl(input, data, spec);
    let mappings = struct_mapping_impl(input, spec);
    use quote::quote;
    Ok(quote! {
        #conversion
        #mappings
    })
}

/// Expands derive macro for enum error types.
///
/// Validates that all variants have masterror attributes, then generates
/// conversion and mapping implementations.
///
/// # Arguments
///
/// * `input` - The parsed error type definition
/// * `variants` - List of enum variants with their specifications
///
/// # Returns
///
/// Combined conversion and mapping implementations.
fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<TokenStream, Error> {
    ensure_all_variants_have_masterror(variants)?;
    let conversion = enum_conversion_impl(input, variants);
    let mappings = enum_mapping_impl(input, variants);
    use quote::quote;
    Ok(quote! {
        #conversion
        #mappings
    })
}

#[cfg(test)]
mod tests {
    // Unit tests for individual functions are located in their respective
    // submodules (conversion, mapping, binding, attachment).
    // Integration tests are in the tests/ directory.
}
