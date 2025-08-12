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
