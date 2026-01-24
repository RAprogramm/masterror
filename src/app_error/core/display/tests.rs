// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Tests for display module.

use core::fmt::{Formatter, Result as FmtResult};

use super::DisplayMode;
use crate::{AppError, field};

// ─────────────────────────────────────────────────────────────────────────────
// DisplayMode tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn display_mode_current_returns_valid_mode() {
    let mode = DisplayMode::current();
    assert!(matches!(
        mode,
        DisplayMode::Prod | DisplayMode::Local | DisplayMode::Staging
    ));
}

#[test]
fn display_mode_detect_auto_returns_local_in_debug() {
    if cfg!(debug_assertions) {
        assert_eq!(DisplayMode::detect_auto(), DisplayMode::Local);
    } else {
        assert_eq!(DisplayMode::detect_auto(), DisplayMode::Prod);
    }
}

#[test]
fn display_mode_current_caches_result() {
    let first = DisplayMode::current();
    let second = DisplayMode::current();
    assert_eq!(first, second);
}

#[test]
fn display_mode_detect_auto_returns_prod_in_release() {
    if !cfg!(debug_assertions) {
        assert_eq!(DisplayMode::detect_auto(), DisplayMode::Prod);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Production format tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn fmt_prod_outputs_json() {
    let error = AppError::not_found("User not found");
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""kind":"NotFound""#));
    assert!(output.contains(r#""code":"NOT_FOUND""#));
    assert!(output.contains(r#""message":"User not found""#));
}

#[test]
fn fmt_prod_excludes_redacted_message() {
    let error = AppError::internal("secret").redactable();
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(!output.contains("secret"));
}

#[test]
fn fmt_prod_includes_metadata() {
    let error = AppError::not_found("User not found").with_field(field::u64("user_id", 12345));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""metadata""#));
    assert!(output.contains(r#""user_id":12345"#));
}

#[test]
fn fmt_prod_excludes_sensitive_metadata() {
    let error = AppError::internal("Error").with_field(field::str("password", "secret"));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(!output.contains("secret"));
}

#[test]
fn fmt_prod_escapes_special_chars() {
    let error = AppError::internal("Line\nwith\"quotes\"");
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#"\n"#));
    assert!(output.contains(r#"\""#));
}

#[test]
fn fmt_prod_handles_infinity_in_metadata() {
    let error = AppError::internal("Error").with_field(field::f64("ratio", f64::INFINITY));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains("null"));
}

#[test]
fn fmt_prod_formats_duration_metadata() {
    use core::time::Duration;
    let error = AppError::internal("Error")
        .with_field(field::duration("elapsed", Duration::from_millis(1500)));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""secs":1"#));
    assert!(output.contains(r#""nanos":500000000"#));
}

#[test]
fn fmt_prod_formats_bool_metadata() {
    let error = AppError::internal("Error").with_field(field::bool("active", true));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""active":true"#));
}

#[cfg(feature = "std")]
#[test]
fn fmt_prod_formats_ip_metadata() {
    use std::net::IpAddr;
    let ip: IpAddr = "192.168.1.1".parse().unwrap();
    let error = AppError::internal("Error").with_field(field::ip("client_ip", ip));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""client_ip":"192.168.1.1""#));
}

#[test]
fn fmt_prod_formats_uuid_metadata() {
    use uuid::Uuid;
    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let error = AppError::internal("Error").with_field(field::uuid("request_id", uuid));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""request_id":"550e8400-e29b-41d4-a716-446655440000""#));
}

#[cfg(feature = "serde_json")]
#[test]
fn fmt_prod_formats_json_metadata() {
    let json = serde_json::json!({"nested": "value"});
    let error = AppError::internal("Error").with_field(field::json("data", json));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""data":"#));
}

#[test]
fn fmt_prod_without_message() {
    let error = AppError::bare(crate::AppErrorKind::Internal);
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""kind":"Internal""#));
    assert!(!output.contains(r#""message""#));
}

#[test]
fn fmt_prod_with_multiple_metadata_fields() {
    let error = AppError::not_found("test")
        .with_field(field::str("first", "value1"))
        .with_field(field::u64("second", 42))
        .with_field(field::bool("third", true));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""first":"value1""#));
    assert!(output.contains(r#""second":42"#));
    assert!(output.contains(r#""third":true"#));
}

#[test]
fn fmt_prod_escapes_backslash() {
    let error = AppError::internal("path\\to\\file");
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#"path\\to\\file"#));
}

#[test]
fn fmt_prod_with_i64_metadata() {
    let error = AppError::internal("test").with_field(field::i64("count", -100));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""count":-100"#));
}

#[test]
fn fmt_prod_with_string_metadata() {
    let error = AppError::internal("test").with_field(field::str("name", "value"));
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#""name":"value""#));
}

#[test]
fn fmt_prod_escapes_control_chars() {
    let error = AppError::internal("test\x00\x1F");
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#"\u0000"#));
    assert!(output.contains(r#"\u001f"#));
}

#[test]
fn fmt_prod_escapes_tab_and_carriage_return() {
    let error = AppError::internal("line\ttab\rreturn");
    let output = format!("{}", error.fmt_prod_wrapper());
    assert!(output.contains(r#"\t"#));
    assert!(output.contains(r#"\r"#));
}

// ─────────────────────────────────────────────────────────────────────────────
// Local format tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn fmt_local_outputs_human_readable() {
    let error = AppError::not_found("User not found");
    let output = format!("{}", error.fmt_local_wrapper());
    assert!(output.contains("Error:"));
    assert!(output.contains("Code: NOT_FOUND") || output.contains("Code:"));
    assert!(output.contains("Message: User not found") || output.contains("Message:"));
}

#[cfg(feature = "std")]
#[test]
fn fmt_local_includes_source_chain() {
    use std::io::Error as IoError;
    let io_err = IoError::other("connection failed");
    let error = AppError::internal("Database error").with_source(io_err);
    let output = format!("{}", error.fmt_local_wrapper());
    assert!(output.contains("Caused by"));
    assert!(output.contains("connection failed"));
}

#[test]
fn fmt_local_without_message() {
    let error = AppError::bare(crate::AppErrorKind::BadRequest);
    let output = format!("{}", error.fmt_local_wrapper());
    assert!(output.contains("Error:"));
    assert!(!output.contains("Message:"));
}

#[test]
fn fmt_local_with_metadata() {
    let error = AppError::internal("Error")
        .with_field(field::str("key", "value"))
        .with_field(field::i64("count", -42));
    let output = format!("{}", error.fmt_local_wrapper());
    assert!(output.contains("Context:"));
    assert!(output.contains("key"));
    assert!(output.contains("value") || output.contains("-42"));
}

#[cfg(feature = "colored")]
#[test]
fn fmt_local_with_deep_source_chain() {
    use std::io::{Error as IoError, ErrorKind};
    let io1 = IoError::new(ErrorKind::NotFound, "level 1");
    let io2 = IoError::other(io1);
    let error = AppError::internal("top").with_source(io2);
    let output = format!("{}", error.fmt_local_wrapper());
    assert!(output.contains("Caused by"));
    assert!(output.contains("level 1"));
}

// ─────────────────────────────────────────────────────────────────────────────
// Staging format tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn fmt_staging_outputs_json_with_context() {
    let error = AppError::service("Service unavailable");
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""kind":"Service""#));
    assert!(output.contains(r#""code":"SERVICE""#));
}

#[cfg(feature = "std")]
#[test]
fn fmt_staging_includes_source_chain() {
    use std::io::Error as IoError;
    let io_err = IoError::other("timeout");
    let error = AppError::network("Network error").with_source(io_err);
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""source_chain""#));
    assert!(output.contains("timeout"));
}

#[test]
fn fmt_staging_without_message() {
    let error = AppError::bare(crate::AppErrorKind::Timeout);
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""kind":"Timeout""#));
    assert!(!output.contains(r#""message""#));
}

#[test]
fn fmt_staging_with_metadata() {
    let error = AppError::service("Service error").with_field(field::u64("retry_count", 3));
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""metadata""#));
    assert!(output.contains(r#""retry_count":3"#));
}

#[test]
fn fmt_staging_with_redacted_message() {
    let error = AppError::internal("sensitive data").redactable();
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(!output.contains("sensitive data"));
}

#[test]
fn fmt_staging_with_multiple_metadata_fields() {
    let error = AppError::service("error")
        .with_field(field::str("key1", "value1"))
        .with_field(field::u64("key2", 123))
        .with_field(field::bool("key3", false));
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""key1":"value1""#));
    assert!(output.contains(r#""key2":123"#));
    assert!(output.contains(r#""key3":false"#));
}

#[cfg(feature = "std")]
#[test]
fn fmt_staging_with_deep_source_chain() {
    use std::io::{Error as IoError, ErrorKind};
    let io1 = IoError::new(ErrorKind::NotFound, "inner error");
    let io2 = IoError::other(io1);
    let error = AppError::service("outer").with_source(io2);
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""source_chain""#));
    assert!(output.contains("inner error"));
}

#[test]
fn fmt_staging_with_redacted_and_public_metadata() {
    let error = AppError::internal("test")
        .with_field(field::str("public", "visible"))
        .with_field(field::str("password", "secret"));
    let output = format!("{}", error.fmt_staging_wrapper());
    assert!(output.contains(r#""public":"visible""#));
    assert!(!output.contains("secret"));
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper wrapper for testing
// ─────────────────────────────────────────────────────────────────────────────

use crate::app_error::core::error::Error;

impl Error {
    fn fmt_prod_wrapper(&self) -> FormatterWrapper<'_> {
        FormatterWrapper {
            error: self,
            mode:  FormatterMode::Prod
        }
    }

    fn fmt_local_wrapper(&self) -> FormatterWrapper<'_> {
        FormatterWrapper {
            error: self,
            mode:  FormatterMode::Local
        }
    }

    fn fmt_staging_wrapper(&self) -> FormatterWrapper<'_> {
        FormatterWrapper {
            error: self,
            mode:  FormatterMode::Staging
        }
    }
}

enum FormatterMode {
    Prod,
    Local,
    Staging
}

struct FormatterWrapper<'a> {
    error: &'a Error,
    mode:  FormatterMode
}

impl core::fmt::Display for FormatterWrapper<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.mode {
            FormatterMode::Prod => self.error.fmt_prod(f),
            FormatterMode::Local => self.error.fmt_local(f),
            FormatterMode::Staging => self.error.fmt_staging(f)
        }
    }
}
