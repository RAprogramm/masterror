// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Tests for builder module.

use crate::{AppCode, AppError, AppErrorKind, FieldRedaction, MessageEditPolicy, Metadata, field};

// ─────────────────────────────────────────────────────────────────────────────
// Constructors tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn new_creates_error_with_message() {
    let err = AppError::new(AppErrorKind::BadRequest, "test message");
    assert_eq!(err.kind, AppErrorKind::BadRequest);
    assert_eq!(err.message.as_deref(), Some("test message"));
}

#[test]
fn new_with_owned_string() {
    let msg = String::from("owned message");
    let err = AppError::new(AppErrorKind::Internal, msg);
    assert_eq!(err.message.as_deref(), Some("owned message"));
}

#[test]
fn with_creates_error_with_message() {
    let err = AppError::with(AppErrorKind::Validation, "validation failed");
    assert_eq!(err.kind, AppErrorKind::Validation);
    assert_eq!(err.message.as_deref(), Some("validation failed"));
}

#[test]
fn bare_creates_error_without_message() {
    let err = AppError::bare(AppErrorKind::NotFound);
    assert_eq!(err.kind, AppErrorKind::NotFound);
    assert!(err.message.is_none());
}

#[test]
fn bare_sets_correct_code() {
    let err = AppError::bare(AppErrorKind::Unauthorized);
    assert_eq!(err.code, AppCode::Unauthorized);
}

// ─────────────────────────────────────────────────────────────────────────────
// Modifiers tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn with_code_overrides_default_code() {
    let err = AppError::new(AppErrorKind::BadRequest, "test").with_code(AppCode::NotFound);
    assert_eq!(err.code, AppCode::NotFound);
}

#[test]
fn with_retry_after_secs_sets_retry_advice() {
    let err = AppError::new(AppErrorKind::RateLimited, "slow down").with_retry_after_secs(60);
    assert!(err.retry.is_some());
    assert_eq!(err.retry.unwrap().after_seconds, 60);
}

#[test]
fn with_retry_after_secs_zero() {
    let err = AppError::new(AppErrorKind::RateLimited, "test").with_retry_after_secs(0);
    assert_eq!(err.retry.unwrap().after_seconds, 0);
}

#[test]
fn with_www_authenticate_sets_header() {
    let err = AppError::new(AppErrorKind::Unauthorized, "auth required")
        .with_www_authenticate("Bearer realm=\"api\"");
    assert_eq!(
        err.www_authenticate.as_deref(),
        Some("Bearer realm=\"api\"")
    );
}

#[test]
fn with_www_authenticate_owned_string() {
    let challenge = String::from("Basic realm=\"test\"");
    let err = AppError::unauthorized("test").with_www_authenticate(challenge);
    assert_eq!(
        err.www_authenticate.as_deref(),
        Some("Basic realm=\"test\"")
    );
}

#[test]
fn redactable_sets_edit_policy() {
    let err = AppError::new(AppErrorKind::Internal, "secret").redactable();
    assert_eq!(err.edit_policy, MessageEditPolicy::Redact);
}

#[test]
fn redactable_can_be_chained() {
    let err = AppError::internal("secret")
        .redactable()
        .with_code(AppCode::Internal);
    assert_eq!(err.edit_policy, MessageEditPolicy::Redact);
    assert_eq!(err.code, AppCode::Internal);
}

// ─────────────────────────────────────────────────────────────────────────────
// Metadata tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn with_field_adds_metadata() {
    let err = AppError::new(AppErrorKind::Validation, "bad field")
        .with_field(field::str("name", "email"));
    assert!(err.metadata().get("name").is_some());
}

#[test]
fn with_field_multiple_fields() {
    let err = AppError::internal("test")
        .with_field(field::str("key1", "value1"))
        .with_field(field::u64("key2", 42));
    assert!(err.metadata().get("key1").is_some());
    assert!(err.metadata().get("key2").is_some());
}

#[test]
fn with_fields_adds_multiple() {
    let fields = vec![
        field::str("a", "1"),
        field::str("b", "2"),
        field::str("c", "3"),
    ];
    let err = AppError::new(AppErrorKind::BadRequest, "test").with_fields(fields);
    assert!(err.metadata().get("a").is_some());
    assert!(err.metadata().get("b").is_some());
    assert!(err.metadata().get("c").is_some());
}

#[test]
fn with_fields_empty_iterator() {
    let err = AppError::internal("test").with_fields(Vec::new());
    assert!(err.metadata().is_empty());
}

#[test]
fn redact_field_sets_redaction() {
    let err = AppError::new(AppErrorKind::Internal, "test")
        .with_field(field::str("password", "secret"))
        .redact_field("password", FieldRedaction::Redact);
    // Field exists but is marked for redaction
    assert!(err.metadata().get("password").is_some());
}

#[test]
fn redact_field_nonexistent_field() {
    let err = AppError::internal("test").redact_field("nonexistent", FieldRedaction::Redact);
    // Should not panic, just no-op
    assert!(err.metadata().get("nonexistent").is_none());
}

#[test]
fn with_metadata_replaces_all() {
    let mut metadata = Metadata::new();
    metadata.insert(field::str("new_key", "new_value"));
    let err = AppError::internal("test")
        .with_field(field::str("old_key", "old_value"))
        .with_metadata(metadata);
    assert!(err.metadata().get("old_key").is_none());
    assert!(err.metadata().get("new_key").is_some());
}

#[test]
fn with_metadata_empty() {
    let err = AppError::internal("test")
        .with_field(field::str("key", "value"))
        .with_metadata(Metadata::new());
    assert!(err.metadata().is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// Context tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "std")]
#[test]
fn with_source_attaches_error() {
    use std::io::{Error as IoError, ErrorKind};
    let io_err = IoError::new(ErrorKind::NotFound, "file not found");
    let err = AppError::internal("failed").with_source(io_err);
    assert!(err.source_ref().is_some());
}

#[cfg(feature = "std")]
#[test]
fn with_source_arc_shares_error() {
    use std::{io::Error as IoError, sync::Arc};
    let source = Arc::new(IoError::other("shared error"));
    let err = AppError::internal("test").with_source_arc(source.clone());
    assert!(err.source_ref().is_some());
    assert_eq!(Arc::strong_count(&source), 2);
}

#[cfg(feature = "std")]
#[test]
fn with_context_accepts_owned_error() {
    use std::io::Error as IoError;
    let io_err = IoError::other("context error");
    let err = AppError::service("degraded").with_context(io_err);
    assert!(err.source_ref().is_some());
}

#[cfg(feature = "std")]
#[test]
fn with_context_accepts_arc() {
    use std::{io::Error as IoError, sync::Arc};
    let source: Arc<dyn core::error::Error + Send + Sync> = Arc::new(IoError::other("arc error"));
    let err = AppError::internal("test").with_context(source);
    assert!(err.source_ref().is_some());
}

#[cfg(feature = "backtrace")]
#[test]
fn with_backtrace_attaches_backtrace() {
    use std::backtrace::Backtrace;
    let bt = Backtrace::capture();
    let err = AppError::internal("test").with_backtrace(bt);
    // Just ensure it doesn't panic
    let _ = err;
}

// ─────────────────────────────────────────────────────────────────────────────
// Details tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde_json")]
#[test]
fn with_details_json_attaches_value() {
    use serde_json::json;
    let err = AppError::new(AppErrorKind::Validation, "invalid")
        .with_details_json(json!({"field": "email", "reason": "invalid format"}));
    assert!(err.details.is_some());
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_json_null() {
    use serde_json::Value;
    let err = AppError::internal("test").with_details_json(Value::Null);
    assert!(err.details.is_some());
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_serializes_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Details {
        code:   u32,
        reason: &'static str
    }

    let err = AppError::bad_request("invalid")
        .with_details(Details {
            code:   100,
            reason: "missing field"
        })
        .expect("serialization should succeed");
    assert!(err.details.is_some());
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_handles_serialization_error() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct BadStruct {
        #[serde(serialize_with = "fail_serialize")]
        value: u32
    }

    fn fail_serialize<S>(_: &u32, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        Err(serde::ser::Error::custom("intentional failure"))
    }

    let result = AppError::internal("test").with_details(BadStruct {
        value: 0
    });
    assert!(result.is_err());
}

#[cfg(not(feature = "serde_json"))]
#[test]
fn with_details_text_attaches_string() {
    let err = AppError::internal("test").with_details_text("additional info");
    assert!(err.details.is_some());
}

// ─────────────────────────────────────────────────────────────────────────────
// Diagnostics tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn with_hint_adds_hint() {
    let err = AppError::not_found("User not found").with_hint("Check user ID");
    let diag = err.diagnostics().expect("diagnostics should exist");
    assert_eq!(diag.hints.len(), 1);
}

#[test]
fn with_hint_multiple() {
    let err = AppError::not_found("test")
        .with_hint("hint 1")
        .with_hint("hint 2")
        .with_hint("hint 3");
    let diag = err.diagnostics().unwrap();
    assert_eq!(diag.hints.len(), 3);
}

#[test]
fn with_hint_visible_sets_visibility() {
    use crate::DiagnosticVisibility;
    let err = AppError::unauthorized("test")
        .with_hint_visible("public hint", DiagnosticVisibility::Public);
    let diag = err.diagnostics().unwrap();
    assert_eq!(diag.hints[0].visibility, DiagnosticVisibility::Public);
}

#[test]
fn with_suggestion_adds_suggestion() {
    let err =
        AppError::database_with_message("Connection failed").with_suggestion("Check DB server");
    let diag = err.diagnostics().unwrap();
    assert_eq!(diag.suggestions.len(), 1);
}

#[test]
fn with_suggestion_cmd_includes_command() {
    let err = AppError::database_with_message("test")
        .with_suggestion_cmd("Check status", "systemctl status postgresql");
    let diag = err.diagnostics().unwrap();
    assert!(diag.suggestions[0].command.is_some());
    assert_eq!(
        diag.suggestions[0].command.as_deref(),
        Some("systemctl status postgresql")
    );
}

#[test]
fn with_docs_sets_doc_link() {
    let err = AppError::not_found("test").with_docs("https://docs.example.com/errors/NOT_FOUND");
    let diag = err.diagnostics().unwrap();
    assert!(diag.doc_link.is_some());
    assert_eq!(
        diag.doc_link.as_ref().unwrap().url.as_ref(),
        "https://docs.example.com/errors/NOT_FOUND"
    );
}

#[test]
fn with_docs_titled_includes_title() {
    let err = AppError::unauthorized("test")
        .with_docs_titled("https://docs.example.com/auth", "Authentication Guide");
    let diag = err.diagnostics().unwrap();
    let doc = diag.doc_link.as_ref().unwrap();
    assert_eq!(doc.title.as_deref(), Some("Authentication Guide"));
}

#[test]
fn with_related_code_adds_code() {
    let err = AppError::database_with_message("test")
        .with_related_code("DB_POOL_EXHAUSTED")
        .with_related_code("DB_AUTH_FAILED");
    let diag = err.diagnostics().unwrap();
    assert_eq!(diag.related_codes.len(), 2);
}

#[test]
fn diagnostics_returns_none_when_empty() {
    let err = AppError::internal("no diagnostics");
    assert!(err.diagnostics().is_none());
}

#[test]
fn diagnostics_returns_some_when_present() {
    let err = AppError::internal("test").with_hint("a hint");
    assert!(err.diagnostics().is_some());
}

// ─────────────────────────────────────────────────────────────────────────────
// Chaining tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_builders_can_be_chained() {
    let err = AppError::new(AppErrorKind::Service, "service error")
        .with_code(AppCode::Service)
        .with_retry_after_secs(30)
        .with_field(field::str("service", "payment"))
        .with_hint("Check service health")
        .with_suggestion("Retry in 30 seconds")
        .with_docs("https://docs.example.com/errors");

    assert_eq!(err.code, AppCode::Service);
    assert!(err.retry.is_some());
    assert!(err.metadata().get("service").is_some());
    assert!(err.diagnostics().is_some());
}

#[cfg(feature = "std")]
#[test]
fn chaining_with_source_preserves_all() {
    use std::io::Error as IoError;

    let err = AppError::internal("test")
        .with_field(field::u64("request_id", 12345))
        .with_source(IoError::other("underlying error"))
        .with_hint("debug hint")
        .redactable();

    assert!(err.metadata().get("request_id").is_some());
    assert!(err.source_ref().is_some());
    assert!(err.diagnostics().is_some());
    assert_eq!(err.edit_policy, MessageEditPolicy::Redact);
}
