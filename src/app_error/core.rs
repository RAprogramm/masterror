// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

/// Backtrace capture and configuration.
///
/// Provides backtrace capture functionality with environment-based
/// configuration via `RUST_BACKTRACE`. Handles lazy capture and caching
/// for optimal performance.
#[cfg(feature = "backtrace")]
pub mod backtrace;

/// Builder methods for error construction and mutation.
///
/// Provides fluent API for constructing errors with various properties:
/// - Message and code configuration
/// - Metadata attachment
/// - Source error chaining
/// - Retry advice and authentication challenges
/// - Structured details (JSON or text)
pub mod builder;

/// Core error types and internal representation.
///
/// Defines the main `Error` type and its internal state structure.
/// Provides the foundation for all error handling in the library.
pub mod error;

/// Error introspection and diagnostic methods.
///
/// Provides methods for examining error properties:
/// - Chain traversal
/// - Source inspection
/// - Type downcasting
/// - Message rendering
/// - Metadata access
pub mod introspection;

/// Telemetry integration (metrics and tracing).
///
/// Handles emission of metrics and tracing events when errors are created
/// or modified. Supports conditional compilation for different telemetry
/// backends.
pub mod telemetry;

/// Helper types and utilities.
///
/// Provides supporting types used throughout the error system:
/// - `ContextAttachment` for source error attachment
/// - `MessageEditPolicy` for redaction control
/// - `ErrorChain` iterator for chain traversal
/// - `CapturedBacktrace` type alias
pub mod types;

/// Display formatting and environment-based output modes.
///
/// Provides environment-aware error formatting with three modes:
/// - Production: lightweight JSON, minimal fields
/// - Local: human-readable with full context
/// - Staging: JSON with additional context
pub mod display;

#[cfg(all(test, feature = "backtrace"))]
pub use backtrace::{reset_backtrace_preference, set_backtrace_preference_override};
pub use display::DisplayMode;
pub use error::{AppError, AppResult, Error};
pub use types::{ErrorChain, MessageEditPolicy};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppErrorKind;

    #[test]
    fn error_new_creates_error_with_message() {
        let err = Error::new(AppErrorKind::BadRequest, "invalid input");
        assert_eq!(err.kind, AppErrorKind::BadRequest);
        assert_eq!(err.message.as_deref(), Some("invalid input"));
    }

    #[test]
    fn error_with_creates_error_with_message() {
        let err = Error::with(AppErrorKind::NotFound, "not found");
        assert_eq!(err.kind, AppErrorKind::NotFound);
        assert_eq!(err.message.as_deref(), Some("not found"));
    }

    #[test]
    fn error_bare_creates_error_without_message() {
        let err = Error::bare(AppErrorKind::Internal);
        assert_eq!(err.kind, AppErrorKind::Internal);
        assert!(err.message.is_none());
    }

    #[test]
    fn error_with_code_overrides_code() {
        use crate::AppCode;

        let err = Error::new(AppErrorKind::BadRequest, "test").with_code(AppCode::NotFound);
        assert_eq!(err.code, AppCode::NotFound);
    }

    #[test]
    fn error_with_retry_after_secs_sets_retry() {
        let err = Error::new(AppErrorKind::RateLimited, "slow down").with_retry_after_secs(60);
        assert_eq!(err.retry.map(|r| r.after_seconds), Some(60));
    }

    #[test]
    fn error_with_www_authenticate_sets_header() {
        let err = Error::new(AppErrorKind::Unauthorized, "auth required")
            .with_www_authenticate("Bearer realm=\"api\"");
        assert_eq!(
            err.www_authenticate.as_deref(),
            Some("Bearer realm=\"api\"")
        );
    }

    #[test]
    fn error_with_field_adds_metadata() {
        use crate::field;

        let err = Error::new(AppErrorKind::Validation, "bad field")
            .with_field(field::str("field_name", "email"));
        assert_eq!(
            err.metadata().get("field_name"),
            Some(&crate::app_error::metadata::FieldValue::Str("email".into()))
        );
    }

    #[test]
    fn error_with_fields_adds_multiple_metadata() {
        use crate::field;

        let fields = vec![field::str("key1", "value1"), field::str("key2", "value2")];
        let err = Error::new(AppErrorKind::BadRequest, "test").with_fields(fields);
        assert!(err.metadata().get("key1").is_some());
        assert!(err.metadata().get("key2").is_some());
    }

    #[test]
    fn error_redactable_sets_edit_policy() {
        let err = Error::new(AppErrorKind::Internal, "secret").redactable();
        assert_eq!(err.edit_policy, MessageEditPolicy::Redact);
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_with_source_attaches_source() {
        use std::io::Error as IoError;

        let io_err = IoError::other("disk error");
        let err = Error::new(AppErrorKind::Internal, "fail").with_source(io_err);
        assert!(err.source_ref().is_some());
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_with_context_attaches_source() {
        use std::io::Error as IoError;

        let io_err = IoError::other("network error");
        let err = Error::new(AppErrorKind::Network, "fail").with_context(io_err);
        assert!(err.source_ref().is_some());
    }

    #[test]
    fn error_metadata_returns_metadata() {
        use crate::field;

        let err =
            Error::new(AppErrorKind::Internal, "test").with_field(field::str("test", "value"));
        let metadata = err.metadata();
        assert!(!metadata.is_empty());
    }

    #[test]
    fn error_render_message_returns_message_when_present() {
        let err = Error::new(AppErrorKind::BadRequest, "custom message");
        assert_eq!(err.render_message(), "custom message");
    }

    #[test]
    fn error_render_message_returns_kind_label_when_no_message() {
        let err = Error::bare(AppErrorKind::NotFound);
        assert!(!err.render_message().is_empty());
    }

    #[test]
    fn error_display_shows_kind() {
        let err = Error::new(AppErrorKind::Internal, "test");
        let display = format!("{}", err);
        assert!(!display.is_empty());
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_chain_returns_iterator() {
        use std::io::Error as IoError;

        let io_err = IoError::other("root cause");
        let err = Error::new(AppErrorKind::Internal, "wrapper").with_context(io_err);
        let chain: Vec<_> = err.chain().collect();
        assert_eq!(chain.len(), 2);
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_root_cause_returns_lowest_error() {
        use std::io::Error as IoError;

        let io_err = IoError::other("disk offline");
        let err = Error::new(AppErrorKind::Internal, "db down").with_context(io_err);
        let root = err.root_cause();
        assert_eq!(root.to_string(), "disk offline");
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_is_checks_source_type() {
        use std::io::Error as IoError;

        let io_err = IoError::other("test");
        let err = Error::new(AppErrorKind::Network, "fail").with_context(io_err);
        assert!(err.is::<IoError>());
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_downcast_ref_returns_concrete_type() {
        use std::io::Error as IoError;

        let io_err = IoError::other("disk error");
        let err = Error::new(AppErrorKind::Internal, "fail").with_context(io_err);
        assert!(err.downcast_ref::<IoError>().is_some());
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_downcast_mut_returns_none() {
        use std::io::Error as IoError;

        let io_err = IoError::other("test");
        let mut err = Error::new(AppErrorKind::Internal, "fail").with_context(io_err);
        assert!(err.downcast_mut::<IoError>().is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_downcast_returns_err() {
        use std::io::Error as IoError;

        let io_err = IoError::other("test");
        let err = Error::new(AppErrorKind::Internal, "fail").with_context(io_err);
        assert!(err.downcast::<IoError>().is_err());
    }

    #[test]
    fn app_result_type_alias_works() {
        let result: AppResult<u8> = Ok(42);
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, 42);
        }
    }

    #[test]
    fn error_chain_single_error() {
        let err = Error::new(AppErrorKind::NotFound, "not found");
        let chain: Vec<_> = err.chain().collect();
        assert_eq!(chain.len(), 1);
    }

    #[test]
    fn error_root_cause_self_when_no_source() {
        let err = Error::new(AppErrorKind::Internal, "root");
        let root = err.root_cause();
        assert!(!root.to_string().is_empty());
    }

    #[test]
    fn message_edit_policy_default_is_preserve() {
        assert_eq!(MessageEditPolicy::default(), MessageEditPolicy::Preserve);
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn error_with_details_json_attaches_details() {
        use serde_json::json;

        let err = Error::new(AppErrorKind::Validation, "invalid")
            .with_details_json(json!({"field": "email"}));
        assert!(err.details.is_some());
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn error_with_details_serializes_payload() {
        use serde::Serialize;

        #[derive(Serialize)]
        struct Extra {
            reason: &'static str
        }

        let err = Error::new(AppErrorKind::BadRequest, "invalid")
            .with_details(Extra {
                reason: "missing"
            })
            .expect("should serialize");
        assert!(err.details.is_some());
    }

    #[cfg(all(feature = "std", feature = "backtrace"))]
    #[test]
    fn error_with_backtrace_attaches_backtrace() {
        use std::backtrace::Backtrace;

        let bt = Backtrace::capture();
        let err = Error::new(AppErrorKind::Internal, "test").with_backtrace(bt);
        assert!(err.backtrace.is_some());
    }

    #[cfg(all(feature = "std", feature = "backtrace"))]
    #[test]
    fn error_with_shared_backtrace_reuses_arc() {
        use std::{backtrace::Backtrace, sync::Arc};

        let bt = Arc::new(Backtrace::capture());
        let bt_clone = Arc::clone(&bt);
        let err = Error::new(AppErrorKind::Internal, "test").with_shared_backtrace(bt);

        assert!(err.backtrace.is_some());
        assert_eq!(Arc::strong_count(&bt_clone), 2);
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_with_context_shared_attachment() {
        use std::{io::Error as IoError, sync::Arc};

        use crate::app_error::core::types::ContextAttachment;

        let io_err = Arc::new(IoError::other("shared error"));
        let err = Error::new(AppErrorKind::Internal, "test")
            .with_context(ContextAttachment::Shared(io_err.clone()));

        assert!(err.source_ref().is_some());
        assert_eq!(Arc::strong_count(&io_err), 2);
    }
}
