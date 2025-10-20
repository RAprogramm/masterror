// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Display trait implementation generation for error types.
//!
//! This module provides the core functionality for generating `Display` trait
//! implementations for both struct and enum error types. It handles various
//! display strategies including:
//!
//! - Template-based formatting with placeholder substitution
//! - Transparent delegation to inner fields
//! - Custom formatter function paths
//! - Format argument resolution and projection
//!
//! The module dispatches to specialized implementations based on the error type
//! structure.

use proc_macro2::TokenStream;
use syn::Error;

use crate::input::{ErrorData, ErrorInput};

pub mod enum_impl;
pub mod format_args;
pub mod formatter;
pub mod placeholder;
pub mod projection;
pub mod struct_impl;
pub mod template;

use enum_impl::expand_enum;
use struct_impl::expand_struct;

/// Generates Display trait implementation for error types.
///
/// This function serves as the main entry point for Display code generation.
/// It dispatches to struct or enum-specific implementations based on the input
/// type.
///
/// # Arguments
///
/// * `input` - The parsed error type definition containing metadata and
///   structure information
///
/// # Returns
///
/// A `TokenStream` containing the Display trait implementation, or a compile
/// error if generation fails.
///
/// # Examples
///
/// For a struct error:
/// ```ignore
/// #[derive(Error)]
/// #[error("error occurred: {message}")]
/// struct MyError {
///     message: String,
/// }
/// ```
///
/// For an enum error:
/// ```ignore
/// #[derive(Error)]
/// enum MyError {
///     #[error("IO error: {0}")]
///     Io(std::io::Error),
/// }
/// ```
pub fn expand(input: &ErrorInput) -> Result<TokenStream, Error> {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}
