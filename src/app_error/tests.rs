use std::{borrow::Cow, error::Error as StdError, fmt::Display, sync::Arc};

use super::{AppError, FieldValue, MessageEditPolicy, field};
use crate::{AppCode, AppErrorKind};

// --- Helpers -------------------------------------------------------------

/// Assert helper: kind matches and message is Some(s).
fn assert_err_with_msg(err: AppError, expected: AppErrorKind, msg: &str) {
    assert!(
        matches!(err.kind, k if k == expected),
        "expected kind {:?}, got {:?}",
        expected,
        err.kind
    );
    assert_eq!(err.message.as_deref(), Some(msg));
}

/// Assert helper: kind matches and message is None.
fn assert_err_bare(err: AppError, expected: AppErrorKind) {
    assert!(
        matches!(err.kind, k if k == expected),
        "expected kind {:?}, got {:?}",
        expected,
        err.kind
    );
    assert!(err.message.is_none());
}

#[test]
fn constructors_match_kinds() {
    assert_err_with_msg(
        AppError::not_found("missing"),
        AppErrorKind::NotFound,
        "missing"
    );
    assert_err_with_msg(
        AppError::validation("invalid"),
        AppErrorKind::Validation,
        "invalid"
    );
    assert_err_with_msg(
        AppError::unauthorized("need token"),
        AppErrorKind::Unauthorized,
        "need token"
    );
    assert_err_with_msg(
        AppError::forbidden("no access"),
        AppErrorKind::Forbidden,
        "no access"
    );
    assert_err_with_msg(AppError::conflict("dup"), AppErrorKind::Conflict, "dup");
    assert_err_with_msg(
        AppError::bad_request("bad"),
        AppErrorKind::BadRequest,
        "bad"
    );
    assert_err_with_msg(
        AppError::rate_limited("slow"),
        AppErrorKind::RateLimited,
        "slow"
    );
    assert_err_with_msg(
        AppError::telegram_auth("fail"),
        AppErrorKind::TelegramAuth,
        "fail"
    );
    assert_err_with_msg(AppError::internal("oops"), AppErrorKind::Internal, "oops");
    assert_err_with_msg(AppError::service("down"), AppErrorKind::Service, "down");
    assert_err_with_msg(AppError::config("bad cfg"), AppErrorKind::Config, "bad cfg");
    assert_err_with_msg(
        AppError::turnkey("turnkey"),
        AppErrorKind::Turnkey,
        "turnkey"
    );
    assert_err_with_msg(
        AppError::timeout("timeout"),
        AppErrorKind::Timeout,
        "timeout"
    );
    assert_err_with_msg(AppError::network("net"), AppErrorKind::Network, "net");
    assert_err_with_msg(
        AppError::dependency_unavailable("dep"),
        AppErrorKind::DependencyUnavailable,
        "dep"
    );
    assert_err_with_msg(
        AppError::service_unavailable("dep"),
        AppErrorKind::DependencyUnavailable,
        "dep"
    );
    assert_err_with_msg(
        AppError::serialization("ser"),
        AppErrorKind::Serialization,
        "ser"
    );
    assert_err_with_msg(
        AppError::deserialization("deser"),
        AppErrorKind::Deserialization,
        "deser"
    );
    assert_err_with_msg(
        AppError::external_api("external"),
        AppErrorKind::ExternalApi,
        "external"
    );
    assert_err_with_msg(AppError::queue("queue"), AppErrorKind::Queue, "queue");
    assert_err_with_msg(AppError::cache("cache"), AppErrorKind::Cache, "cache");
}

#[test]
fn database_accepts_optional_message() {
    let with_msg = AppError::database_with_message("db down");
    assert_err_with_msg(with_msg, AppErrorKind::Database, "db down");

    let via_option = AppError::database(Some(Cow::Borrowed("db down")));
    assert_err_with_msg(via_option, AppErrorKind::Database, "db down");

    let without = AppError::database(None);
    assert_err_bare(without, AppErrorKind::Database);
}

#[test]
fn bare_sets_kind_without_message() {
    assert_err_bare(
        AppError::bare(AppErrorKind::Internal),
        AppErrorKind::Internal
    );
}

#[test]
fn retry_and_www_authenticate_are_attached() {
    let err = AppError::internal("boom")
        .with_retry_after_secs(30)
        .with_www_authenticate("Bearer");
    assert_eq!(err.retry.unwrap().after_seconds, 30);
    assert_eq!(err.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn render_message_does_not_allocate_for_borrowed_str() {
    let err = AppError::new(AppErrorKind::BadRequest, "borrowed");
    let rendered = err.render_message();
    assert!(matches!(rendered, Cow::Borrowed("borrowed")));
    assert!(std::ptr::eq(rendered.as_ref(), "borrowed"));
}

#[test]
fn metadata_and_code_are_preserved() {
    let err = AppError::service("downstream")
        .with_field(field::str("request_id", "abc-123"))
        .with_field(field::i64("attempt", 2))
        .with_code(AppCode::Service);

    assert_eq!(err.code, AppCode::Service);
    let metadata = err.metadata();
    assert_eq!(metadata.len(), 2);
    assert_eq!(
        metadata.get("request_id"),
        Some(&FieldValue::Str(Cow::Borrowed("abc-123")))
    );
    assert_eq!(metadata.get("attempt"), Some(&FieldValue::I64(2)));
}

#[derive(Debug)]
struct DummyError;

impl Display for DummyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dummy")
    }
}

impl StdError for DummyError {}

#[test]
fn source_is_preserved_without_extra_allocation() {
    let source = Arc::new(DummyError);
    let err = AppError::internal("boom").with_source(source.clone());

    assert_eq!(Arc::strong_count(&source), 2);

    let stored = err.source_ref().expect("source");
    let stored_arc = stored
        .downcast_ref::<Arc<DummyError>>()
        .expect("arc should be preserved");
    assert!(Arc::ptr_eq(stored_arc, &source));
}

#[test]
fn redactable_policy_is_exposed() {
    let err = AppError::internal("boom").redactable();
    assert!(matches!(err.edit_policy, MessageEditPolicy::Redact));
}

#[test]
fn log_uses_kind_and_code() {
    // Smoke test to ensure the method is callable; tracing output isn't asserted
    // here.
    let err = AppError::internal("boom");
    err.log();
}

#[test]
fn result_alias_is_generic() {
    fn app() -> super::AppResult<u8> {
        Ok(1)
    }

    fn other() -> super::AppResult<u8, &'static str> {
        Ok(2)
    }

    assert_eq!(app().unwrap(), 1);
    assert_eq!(other().unwrap(), 2);
}
