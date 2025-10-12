// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Test for Rust 2024 edition compatibility
//!
//! This test ensures that the macro-generated code does not trigger the
//! `non_shorthand_field_patterns` lint introduced in Rust 2024 edition.

#![deny(non_shorthand_field_patterns)]

use std::error::Error as StdError;

use masterror::Error;

#[derive(Debug, Error)]
#[error("parse error: {source}")]
pub struct ParseError {
    #[source]
    source: std::num::ParseIntError
}

#[derive(Debug, Error)]
#[error("io error")]
pub struct IoError {
    #[source]
    source: std::io::Error
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("failed to parse: {source}")]
    Parse {
        #[source]
        source: std::num::ParseIntError
    },
    #[error("io failure: {source}")]
    Io {
        #[source]
        source: std::io::Error
    },
    #[error("network error: {0}")]
    Network(#[source] std::io::Error),
    #[error("unknown error")]
    Unknown
}

#[derive(Debug, Error)]
#[error("multi-field error: {message}, context: {context:?}")]
pub struct MultiFieldError {
    message: String,
    #[source]
    source:  std::io::Error,
    context: Option<String>
}

#[derive(Debug, Error)]
pub enum ComplexError {
    #[error("complex variant: {message}, code: {code}, caused by: {source}")]
    Complex {
        message: String,
        #[source]
        source:  std::io::Error,
        code:    u16
    }
}

#[test]
fn test_struct_with_source() {
    let inner = "not a number".parse::<i32>().unwrap_err();
    let error = ParseError {
        source: inner
    };
    assert!(error.source().is_some());
}

#[test]
fn test_enum_with_source() {
    let inner = "not a number".parse::<i32>().unwrap_err();
    let error = AppError::Parse {
        source: inner
    };
    assert!(error.source().is_some());
}

#[test]
fn test_multi_field_struct() {
    let io_error = std::io::Error::other("test");
    let error = MultiFieldError {
        message: "test message".to_string(),
        source:  io_error,
        context: Some("additional context".to_string())
    };
    assert!(error.source().is_some());
}

#[test]
fn test_complex_enum_variant() {
    let io_error = std::io::Error::other("test");
    let error = ComplexError::Complex {
        message: "test".to_string(),
        source:  io_error,
        code:    500
    };
    assert!(error.source().is_some());
}
