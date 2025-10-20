// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Input parsing for error derive macro.
//!
//! This module handles parsing of derive macro input, including:
//! - Data structure definitions ([`types`])
//! - Attribute parsing ([`parse_attr`])
//! - Format argument parsing ([`parse_format`])
//! - Main parsing logic ([`parse`])
//! - Validation utilities ([`utils`])

pub mod parse;
pub mod parse_attr;
pub mod parse_format;
pub mod types;
pub mod utils;

// Re-export main parsing function
pub use parse::parse_input;
// Re-export all public types
#[allow(unused_imports)]
pub use types::{
    AppErrorSpec, BacktraceField, BacktraceFieldKind, DisplaySpec, ErrorData, ErrorInput, Field,
    FieldAttrs, FieldRedactionKind, FieldRedactionSpec, Fields, FormatArg,
    FormatArgMethodTurbofish, FormatArgProjection, FormatArgProjectionMethodCall,
    FormatArgProjectionSegment, FormatArgShorthand, FormatArgValue, FormatArgsSpec,
    FormatBindingKind, MasterrorSpec, ProvideSpec, RedactSpec, StructData, VariantData
};
// Re-export crate-internal utility functions
pub(crate) use utils::{is_arc_type, is_backtrace_storage, option_inner_type};
// Re-export public utility functions
pub use utils::{is_option_type, placeholder_error};
