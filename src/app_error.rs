//! Core application error type: [`AppError`].
//!
//! [`AppError`] is a thin, framework-agnostic wrapper around a canonical
//! error taxonomy [`AppErrorKind`] plus an optional public-facing message.
//! The `Display` for `AppError` prints only the kind, not the message, to keep
//! logs and errors concise by default.
//!
//! ## Design
//!
//! - **Stable taxonomy:** the semantic category is captured by
//!   [`AppErrorKind`].
//! - **Optional message:** human-readable, safe-to-expose text. Do not put
//!   secrets here.
//! - **No panics:** all helpers avoid `unwrap/expect`.
//! - **Transport-agnostic:** mapping to HTTP lives in `kind.rs` and
//!   `convert/*`.
//!
//! ## Common usage
//!
//! Build errors either with generic constructors or named helpers matching
//! taxonomy variants:
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind};
//!
//! // generic
//! let e1 = AppError::with(AppErrorKind::BadRequest, "flag_required");
//! let e2 = AppError::new(AppErrorKind::Forbidden, "access denied");
//!
//! // named helpers
//! let e3 = AppError::not_found("user not found");
//! let e4 = AppError::timeout("operation timed out");
//!
//! assert!(matches!(e1.kind, AppErrorKind::BadRequest));
//! assert!(e3.message.as_deref() == Some("user not found"));
//! ```
//!
//! ## HTTP (Axum) integration
//!
//! With the `axum` feature enabled the crate provides `IntoResponse` for
//! `AppError` (see `convert/axum.rs`). You can return `AppResult<T>` from
//! handlers and the crate will build a JSON error (if `serde_json` is enabled)
//! with status derived from [`AppErrorKind`].
//!
//! ```rust,ignore
//! # #[cfg(feature = "axum")]
//! use masterror::{AppError, AppResult};
//!
//! async fn handler() -> AppResult<&'static str> {
//!     Err(AppError::forbidden("no access"))
//! }
//! ```
//!
//! ## Logging
//!
//! [`AppError::log`] emits a single structured `tracing::error!` event. Prefer
//! calling it at the transport boundary (e.g. in `IntoResponse`) to avoid
//! duplicate logs.

use thiserror::Error;
use tracing::error;

use crate::kind::AppErrorKind;

/// Thin error wrapper: kind + optional message.
///
/// `Display` prints only the `kind`. The optional `message` is intended for
/// logs and (when appropriate) public JSON payloads. Keep messages concise and
/// free of sensitive data.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct AppError {
    /// Semantic category of the error.
    pub kind:    AppErrorKind,
    /// Optional, public-friendly message.
    pub message: Option<String>
}

/// Conventional result alias for application code.
pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// Create a new [`AppError`] with a kind and message.
    ///
    /// This is equivalent to [`AppError::with`], provided for API symmetry and
    /// to keep doctests readable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid payload");
    /// assert!(err.message.is_some());
    /// ```
    pub fn new(kind: AppErrorKind, msg: impl Into<String>) -> Self {
        Self::with(kind, msg)
    }

    /// Create an error with the given kind and message.
    ///
    /// Prefer named helpers (e.g. [`AppError::not_found`]) where it clarifies
    /// intent.
    pub fn with(kind: AppErrorKind, msg: impl Into<String>) -> Self {
        Self {
            kind,
            message: Some(msg.into())
        }
    }

    /// Create a message-less error with the given kind.
    ///
    /// Useful when the kind alone conveys sufficient information to the client.
    pub fn bare(kind: AppErrorKind) -> Self {
        Self {
            kind,
            message: None
        }
    }

    /// Log the error once at the boundary with stable fields.
    ///
    /// Emits a `tracing::error!` with `kind` and optional `message`.
    /// No internals or sources are leaked.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::internal("boom");
    /// // In production, call this at the boundary (e.g. HTTP mapping).
    /// err.log();
    /// ```
    pub fn log(&self) {
        match &self.message {
            Some(m) => error!(kind = ?self.kind, message = %m),
            None => error!(kind = ?self.kind)
        }
    }

    // --- Canonical constructors (keep in sync with AppErrorKind) -------------

    // 4xx-ish
    /// Build a `NotFound` error.
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::NotFound, msg)
    }
    /// Build a `Validation` error.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Validation, msg)
    }
    /// Build an `Unauthorized` error.
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Unauthorized, msg)
    }
    /// Build a `Forbidden` error.
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Forbidden, msg)
    }
    /// Build a `Conflict` error.
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Conflict, msg)
    }
    /// Build a `BadRequest` error.
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::BadRequest, msg)
    }
    /// Build a `RateLimited` error.
    pub fn rate_limited(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::RateLimited, msg)
    }
    /// Build a `TelegramAuth` error.
    pub fn telegram_auth(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::TelegramAuth, msg)
    }

    // 5xx-ish
    /// Build an `Internal` error.
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Internal, msg)
    }
    /// Build a `Service` error (generic server-side service failure).
    pub fn service(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Service, msg)
    }
    /// Build a `Database` error with an optional message.
    ///
    /// Accepts `Option` to avoid gratuitous `.map(|...| ...)` at call sites
    /// when you may or may not have a safe-to-print string at hand.
    pub fn database(msg: Option<impl Into<String>>) -> Self {
        Self {
            kind:    AppErrorKind::Database,
            message: msg.map(|m| m.into())
        }
    }
    /// Build a `Config` error.
    pub fn config(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Config, msg)
    }
    /// Build a `Turnkey` error.
    pub fn turnkey(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Turnkey, msg)
    }

    // Infra / network
    /// Build a `Timeout` error.
    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Timeout, msg)
    }
    /// Build a `Network` error.
    pub fn network(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Network, msg)
    }
    /// Build a `DependencyUnavailable` error.
    pub fn dependency_unavailable(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::DependencyUnavailable, msg)
    }
    /// Backward-compatible alias; routes to `DependencyUnavailable`.
    pub fn service_unavailable(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::DependencyUnavailable, msg)
    }

    // Serialization / external API / subsystems
    /// Build a `Serialization` error.
    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Serialization, msg)
    }
    /// Build a `Deserialization` error.
    pub fn deserialization(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Deserialization, msg)
    }
    /// Build an `ExternalApi` error.
    pub fn external_api(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::ExternalApi, msg)
    }
    /// Build a `Queue` error.
    pub fn queue(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Queue, msg)
    }
    /// Build a `Cache` error.
    pub fn cache(msg: impl Into<String>) -> Self {
        Self::with(AppErrorKind::Cache, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::{AppError, AppErrorKind, AppResult};

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
        assert!(err.message.is_none(), "expected no message");
    }

    // --- Constructors: generic ----------------------------------------------

    #[test]
    fn new_and_with_attach_message() {
        let e1 = AppError::new(AppErrorKind::BadRequest, "invalid payload");
        assert_err_with_msg(e1, AppErrorKind::BadRequest, "invalid payload");

        let e2 = AppError::with(AppErrorKind::Forbidden, "no access");
        assert_err_with_msg(e2, AppErrorKind::Forbidden, "no access");
    }

    #[test]
    fn bare_sets_only_kind() {
        let e = AppError::bare(AppErrorKind::NotFound);
        assert_err_bare(e, AppErrorKind::NotFound);
    }

    // --- Display formatting --------------------------------------------------

    #[test]
    fn display_prints_kind_only() {
        // AppError's Display is "{kind}", message must not appear.
        let e = AppError::new(AppErrorKind::Validation, "email invalid");
        let shown = format!("{}", e);
        // AppErrorKind::Validation Display text is defined on the enum via
        // `thiserror::Error`. We only assert that message is not leaked.
        assert!(
            !shown.contains("email invalid"),
            "Display must not include the public message"
        );

        // Spot-check kind text presence: should include "Validation".
        assert!(
            shown.to_lowercase().contains("validation"),
            "Display should include kind name text"
        );
    }

    // --- Named helpers: 4xx --------------------------------------------------

    #[test]
    fn not_found() {
        assert_err_with_msg(
            AppError::not_found("missing"),
            AppErrorKind::NotFound,
            "missing"
        );
    }

    #[test]
    fn validation() {
        assert_err_with_msg(
            AppError::validation("bad email"),
            AppErrorKind::Validation,
            "bad email"
        );
    }

    #[test]
    fn unauthorized() {
        assert_err_with_msg(
            AppError::unauthorized("no token"),
            AppErrorKind::Unauthorized,
            "no token"
        );
    }

    #[test]
    fn forbidden() {
        assert_err_with_msg(
            AppError::forbidden("no access"),
            AppErrorKind::Forbidden,
            "no access"
        );
    }

    #[test]
    fn conflict() {
        assert_err_with_msg(
            AppError::conflict("version mismatch"),
            AppErrorKind::Conflict,
            "version mismatch"
        );
    }

    #[test]
    fn bad_request() {
        assert_err_with_msg(
            AppError::bad_request("malformed"),
            AppErrorKind::BadRequest,
            "malformed"
        );
    }

    #[test]
    fn rate_limited() {
        assert_err_with_msg(
            AppError::rate_limited("slow down"),
            AppErrorKind::RateLimited,
            "slow down"
        );
    }

    #[test]
    fn telegram_auth() {
        assert_err_with_msg(
            AppError::telegram_auth("bad hash"),
            AppErrorKind::TelegramAuth,
            "bad hash"
        );
    }

    // --- Named helpers: 5xx and infra ---------------------------------------

    #[test]
    fn internal() {
        assert_err_with_msg(AppError::internal("boom"), AppErrorKind::Internal, "boom");
    }

    #[test]
    fn service() {
        assert_err_with_msg(
            AppError::service("failed pipeline"),
            AppErrorKind::Service,
            "failed pipeline"
        );
    }

    #[test]
    fn database_some_message() {
        let e = AppError::database(Some("unique violation"));
        assert_err_with_msg(e, AppErrorKind::Database, "unique violation");
    }

    #[test]
    fn database_no_message() {
        let e = AppError::database(None::<String>);
        assert_err_bare(e, AppErrorKind::Database);
    }

    #[test]
    fn config() {
        assert_err_with_msg(AppError::config("bad env"), AppErrorKind::Config, "bad env");
    }

    #[test]
    fn turnkey() {
        assert_err_with_msg(
            AppError::turnkey("provider down"),
            AppErrorKind::Turnkey,
            "provider down"
        );
    }

    #[test]
    fn timeout() {
        assert_err_with_msg(
            AppError::timeout("deadline exceeded"),
            AppErrorKind::Timeout,
            "deadline exceeded"
        );
    }

    #[test]
    fn network() {
        assert_err_with_msg(AppError::network("dns"), AppErrorKind::Network, "dns");
    }

    #[test]
    fn dependency_unavailable_and_alias() {
        let e = AppError::dependency_unavailable("cache down");
        assert_err_with_msg(e, AppErrorKind::DependencyUnavailable, "cache down");

        // Alias must map to the same kind.
        let alias = AppError::service_unavailable("cache down");
        assert_err_with_msg(alias, AppErrorKind::DependencyUnavailable, "cache down");
    }

    #[test]
    fn serialization() {
        assert_err_with_msg(
            AppError::serialization("encode fail"),
            AppErrorKind::Serialization,
            "encode fail"
        );
    }

    #[test]
    fn deserialization() {
        assert_err_with_msg(
            AppError::deserialization("decode fail"),
            AppErrorKind::Deserialization,
            "decode fail"
        );
    }

    #[test]
    fn external_api() {
        assert_err_with_msg(
            AppError::external_api("upstream 502"),
            AppErrorKind::ExternalApi,
            "upstream 502"
        );
    }

    #[test]
    fn queue() {
        assert_err_with_msg(AppError::queue("nack"), AppErrorKind::Queue, "nack");
    }

    #[test]
    fn cache() {
        assert_err_with_msg(AppError::cache("miss"), AppErrorKind::Cache, "miss");
    }

    // --- AppResult alias -----------------------------------------------------

    #[test]
    fn app_result_alias_compiles_and_matches() {
        fn ok() -> AppResult<u8> {
            Ok(1)
        }
        fn err() -> AppResult<u8> {
            Err(AppError::internal("x"))
        }

        let a: AppResult<u8> = ok();
        let b: AppResult<u8> = err();

        assert_eq!(a.unwrap(), 1);
        assert!(b.is_err());
        if let Err(e) = b {
            assert!(matches!(e.kind, AppErrorKind::Internal));
        }
    }

    // --- Logging path sanity check -------------------------------------------

    #[test]
    fn log_does_not_panic() {
        // We cannot assert on tracing output here, but we can ensure no panics happen.
        let e1 = AppError::internal("boom");
        e1.log();
        let e2 = AppError::bare(AppErrorKind::BadRequest);
        e2.log();
    }
}
