// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Canonical application error taxonomy and HTTP mappings.
//!
//! This enum defines the **stable**, framework-agnostic set of error categories
//! used throughout the application. Each variant represents a semantic category
//! that can be mapped to transport-specific representations (such as HTTP
//! status codes) at the service boundary.
//!
//! ## Design rules
//!
//! - Keep this set **small and stable** — breaking changes here affect all
//!   services consuming the crate.
//! - Assign HTTP status codes based on the *category* of the error, not the
//!   original source.
//! - Infrastructure and I/O issues default to **5xx** unless explicitly mapped.
//! - Authentication/authorization problems are split into:
//!   - `Unauthorized` (401) — authentication is required or failed.
//!   - `Forbidden` (403) — authentication succeeded but access is denied.
//!
//! ## Mapping methods
//!
//! - [`http_status`](Self::http_status) — always available, returns a numeric
//!   status code (`u16`).
//! - [`status_code`](Self::status_code) — available with the `axum` feature,
//!   returns [`axum::http::StatusCode`].
//!
//! ## Example
//!
//! ```rust
//! use masterror::AppErrorKind;
//!
//! let kind = AppErrorKind::NotFound;
//! assert_eq!(kind.http_status(), 404);
//!
//! #[cfg(feature = "axum")]
//! assert_eq!(kind.status_code().as_u16(), 404);
//! ```

use core::{
    error::Error as CoreError,
    fmt::{self, Display, Formatter}
};

#[cfg(feature = "axum")]
use axum::http::StatusCode;

/// Canonical application error taxonomy.
///
/// Keep it small, stable, and framework-agnostic. Each variant has a clear,
/// documented meaning and a predictable mapping to an HTTP status code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppErrorKind {
    // ── Generic, client-visible failures (4xx/5xx) ────────────────────────────
    /// Resource does not exist or is not visible to the caller.
    ///
    /// Maps to **404 Not Found**.
    NotFound,

    /// Input failed validation (shape, constraints, business rules).
    ///
    /// Prefer this over `BadRequest` when you validate structured input.
    /// Maps to **422 Unprocessable Entity**.
    Validation,

    /// State conflict with an existing resource or concurrent update.
    ///
    /// Typical cases: unique key violation, version mismatch (ETag).
    /// Maps to **409 Conflict**.
    Conflict,

    /// Authentication required or failed (missing/invalid credentials).
    ///
    /// Maps to **401 Unauthorized**.
    Unauthorized,

    /// Authenticated but not allowed to perform the operation.
    ///
    /// Maps to **403 Forbidden**.
    Forbidden,

    /// Operation is not implemented or not supported by this deployment.
    ///
    /// Maps to **501 Not Implemented**.
    NotImplemented,

    /// Unexpected server-side failure not captured by more specific kinds.
    ///
    /// Use sparingly; prefer a more precise category when possible.
    /// Maps to **500 Internal Server Error**.
    Internal,

    /// Malformed request or missing required parameters.
    ///
    /// Prefer `Validation` for structured input with field-level issues.
    /// Maps to **400 Bad Request**.
    BadRequest,

    // ── Domain-specific categories (map conservatively) ───────────────────────
    /// Telegram authentication flow failed (signature, timestamp, or payload).
    ///
    /// Treated as an authentication failure.
    /// Maps to **401 Unauthorized**.
    TelegramAuth,

    /// Provided JWT is invalid (expired, malformed, wrong signature/claims).
    ///
    /// Treated as an authentication failure.
    /// Maps to **401 Unauthorized**.
    InvalidJwt,

    /// Database-related failure (query, connection, migration, etc.).
    ///
    /// Keep driver-specific details out of the public contract.
    /// Maps to **500 Internal Server Error**.
    Database,

    /// Generic service-layer failure (business logic or internal
    /// orchestration).
    ///
    /// Use when no more specific category applies.
    /// Maps to **500 Internal Server Error**.
    Service,

    /// Configuration error (missing/invalid environment or runtime config).
    ///
    /// Maps to **500 Internal Server Error**.
    Config,

    /// Failure in the Turnkey subsystem/integration.
    ///
    /// Maps to **500 Internal Server Error**.
    Turnkey,

    // ── Infrastructure / network ──────────────────────────────────────────────
    /// Operation did not complete within the allotted time.
    ///
    /// Typically returned by timeouts around I/O or remote calls.
    /// Maps to **504 Gateway Timeout**.
    Timeout,

    /// Network-level error (DNS, connect, TLS, request build).
    ///
    /// For upstream HTTP status failures use `ExternalApi` instead.
    /// Maps to **503 Service Unavailable**.
    Network,

    /// Client exceeded rate limits or quota.
    ///
    /// Maps to **429 Too Many Requests**.
    RateLimited,

    /// External dependency is unavailable or degraded.
    ///
    /// Examples: cache down, message broker unreachable, third-party outage.
    /// Maps to **503 Service Unavailable**.
    DependencyUnavailable,

    // ── Serialization / external API / infra subsystems ───────────────────────
    /// Failed to serialize data (encode).
    ///
    /// Maps to **500 Internal Server Error**.
    Serialization,

    /// Failed to deserialize data (decode).
    ///
    /// Maps to **500 Internal Server Error**.
    Deserialization,

    /// Upstream API returned an error or the call failed at protocol level.
    ///
    /// Use `Network` for connect/build failures; use this for HTTP status
    /// errors. Maps to **500 Internal Server Error** by default.
    ExternalApi,

    /// Queue processing failure (publish/consume/ack).
    ///
    /// Maps to **500 Internal Server Error**.
    Queue,

    /// Cache subsystem failure (read/write/encoding).
    ///
    /// Maps to **500 Internal Server Error**.
    Cache
}

#[cfg(not(feature = "colored"))]
impl Display for AppErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[cfg(feature = "colored")]
impl Display for AppErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use crate::colored::style;
        let label = self.label();
        let styled = if self.is_critical() {
            style::error_kind_critical(label)
        } else {
            style::error_kind_warning(label)
        };
        f.write_str(&styled)
    }
}

impl CoreError for AppErrorKind {}

impl AppErrorKind {
    /// Human-readable label exposed in HTTP and telemetry payloads.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::NotFound => "Not found",
            Self::Validation => "Validation error",
            Self::Conflict => "Conflict",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotImplemented => "Not implemented",
            Self::Internal => "Internal server error",
            Self::BadRequest => "Bad request",
            Self::TelegramAuth => "Telegram authentication error",
            Self::InvalidJwt => "Invalid JWT",
            Self::Database => "Database error",
            Self::Service => "Service error",
            Self::Config => "Configuration error",
            Self::Turnkey => "Turnkey error",
            Self::Timeout => "Operation timed out",
            Self::Network => "Network error",
            Self::RateLimited => "Rate limit exceeded",
            Self::DependencyUnavailable => "External dependency unavailable",
            Self::Serialization => "Serialization error",
            Self::Deserialization => "Deserialization error",
            Self::ExternalApi => "External API error",
            Self::Queue => "Queue processing error",
            Self::Cache => "Cache error"
        }
    }

    /// Framework-agnostic mapping to an HTTP status code (`u16`).
    ///
    /// This mapping is intentionally conservative and stable. It should **not**
    /// leak environment-specific details (e.g. DB driver error codes or HTTP
    /// library errors).
    pub fn http_status(&self) -> u16 {
        match self {
            AppErrorKind::NotFound => 404,
            AppErrorKind::Validation => 422,
            AppErrorKind::Conflict => 409,
            AppErrorKind::Unauthorized | AppErrorKind::InvalidJwt | AppErrorKind::TelegramAuth => {
                401
            }
            AppErrorKind::Forbidden => 403,
            AppErrorKind::NotImplemented => 501,
            AppErrorKind::BadRequest => 400,
            AppErrorKind::RateLimited => 429,
            AppErrorKind::Timeout => 504,
            AppErrorKind::Network | AppErrorKind::DependencyUnavailable => 503,
            AppErrorKind::Serialization
            | AppErrorKind::Deserialization
            | AppErrorKind::ExternalApi
            | AppErrorKind::Queue
            | AppErrorKind::Cache
            | AppErrorKind::Database
            | AppErrorKind::Service
            | AppErrorKind::Config
            | AppErrorKind::Turnkey
            | AppErrorKind::Internal => 500
        }
    }

    /// Mapping to [`axum::http::StatusCode`] (available with the `axum`
    /// feature).
    #[cfg(feature = "axum")]
    #[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// Check if this error kind represents a critical server-side failure.
    ///
    /// Critical errors are those with HTTP status >= 500, indicating internal
    /// server errors that require immediate attention.
    ///
    /// Used for color-coding in terminal output: critical errors are shown in
    /// red, while client errors are shown in yellow.
    #[cfg(feature = "colored")]
    pub(crate) fn is_critical(&self) -> bool {
        self.http_status() >= 500
    }
}

#[cfg(test)]
mod tests {
    use super::AppErrorKind::*;

    #[test]
    fn http_status_is_stable() {
        assert_eq!(NotFound.http_status(), 404);
        assert_eq!(Validation.http_status(), 422);
        assert_eq!(Unauthorized.http_status(), 401);
        assert_eq!(Forbidden.http_status(), 403);
        assert_eq!(Conflict.http_status(), 409);
        assert_eq!(BadRequest.http_status(), 400);
        assert_eq!(RateLimited.http_status(), 429);
        assert_eq!(Timeout.http_status(), 504);
        assert_eq!(DependencyUnavailable.http_status(), 503);
        assert_eq!(Internal.http_status(), 500);
    }

    #[test]
    #[cfg(feature = "colored")]
    fn is_critical_identifies_server_errors() {
        assert!(Internal.is_critical());
        assert!(Database.is_critical());
        assert!(Service.is_critical());
        assert!(Config.is_critical());
        assert!(Timeout.is_critical());
        assert!(Network.is_critical());
        assert!(DependencyUnavailable.is_critical());
        assert!(Serialization.is_critical());
        assert!(Deserialization.is_critical());
        assert!(ExternalApi.is_critical());
        assert!(Queue.is_critical());
        assert!(Cache.is_critical());
        assert!(Turnkey.is_critical());
        assert!(NotImplemented.is_critical());
    }

    #[test]
    #[cfg(feature = "colored")]
    fn is_critical_excludes_client_errors() {
        assert!(!NotFound.is_critical());
        assert!(!Validation.is_critical());
        assert!(!Conflict.is_critical());
        assert!(!Unauthorized.is_critical());
        assert!(!Forbidden.is_critical());
        assert!(!BadRequest.is_critical());
        assert!(!TelegramAuth.is_critical());
        assert!(!InvalidJwt.is_critical());
        assert!(!RateLimited.is_critical());
    }

    #[test]
    fn display_shows_label() {
        assert_eq!(NotFound.to_string(), "Not found");
        assert_eq!(Internal.to_string(), "Internal server error");
        assert_eq!(BadRequest.to_string(), "Bad request");
    }

    #[test]
    #[cfg(feature = "colored")]
    fn display_colored_contains_label() {
        let output = Internal.to_string();
        assert!(output.contains("Internal server error"));
        let output = BadRequest.to_string();
        assert!(output.contains("Bad request"));
    }

    #[test]
    fn http_status_all_variants() {
        assert_eq!(NotFound.http_status(), 404);
        assert_eq!(Validation.http_status(), 422);
        assert_eq!(Conflict.http_status(), 409);
        assert_eq!(Unauthorized.http_status(), 401);
        assert_eq!(Forbidden.http_status(), 403);
        assert_eq!(NotImplemented.http_status(), 501);
        assert_eq!(Internal.http_status(), 500);
        assert_eq!(BadRequest.http_status(), 400);
        assert_eq!(TelegramAuth.http_status(), 401);
        assert_eq!(InvalidJwt.http_status(), 401);
        assert_eq!(Database.http_status(), 500);
        assert_eq!(Service.http_status(), 500);
        assert_eq!(Config.http_status(), 500);
        assert_eq!(Turnkey.http_status(), 500);
        assert_eq!(Timeout.http_status(), 504);
        assert_eq!(Network.http_status(), 503);
        assert_eq!(RateLimited.http_status(), 429);
        assert_eq!(DependencyUnavailable.http_status(), 503);
        assert_eq!(Serialization.http_status(), 500);
        assert_eq!(Deserialization.http_status(), 500);
        assert_eq!(ExternalApi.http_status(), 500);
        assert_eq!(Queue.http_status(), 500);
        assert_eq!(Cache.http_status(), 500);
    }

    #[test]
    fn label_all_variants() {
        assert_eq!(NotFound.label(), "Not found");
        assert_eq!(Validation.label(), "Validation error");
        assert_eq!(Conflict.label(), "Conflict");
        assert_eq!(Unauthorized.label(), "Unauthorized");
        assert_eq!(Forbidden.label(), "Forbidden");
        assert_eq!(NotImplemented.label(), "Not implemented");
        assert_eq!(Internal.label(), "Internal server error");
        assert_eq!(BadRequest.label(), "Bad request");
        assert_eq!(TelegramAuth.label(), "Telegram authentication error");
        assert_eq!(InvalidJwt.label(), "Invalid JWT");
        assert_eq!(Database.label(), "Database error");
        assert_eq!(Service.label(), "Service error");
        assert_eq!(Config.label(), "Configuration error");
        assert_eq!(Turnkey.label(), "Turnkey error");
        assert_eq!(Timeout.label(), "Operation timed out");
        assert_eq!(Network.label(), "Network error");
        assert_eq!(RateLimited.label(), "Rate limit exceeded");
        assert_eq!(
            DependencyUnavailable.label(),
            "External dependency unavailable"
        );
        assert_eq!(Serialization.label(), "Serialization error");
        assert_eq!(Deserialization.label(), "Deserialization error");
        assert_eq!(ExternalApi.label(), "External API error");
        assert_eq!(Queue.label(), "Queue processing error");
        assert_eq!(Cache.label(), "Cache error");
    }

    #[test]
    fn display_all_variants() {
        assert_eq!(NotFound.to_string(), NotFound.label());
        assert_eq!(Validation.to_string(), Validation.label());
        assert_eq!(Conflict.to_string(), Conflict.label());
        assert_eq!(Unauthorized.to_string(), Unauthorized.label());
        assert_eq!(Forbidden.to_string(), Forbidden.label());
        assert_eq!(NotImplemented.to_string(), NotImplemented.label());
        assert_eq!(Internal.to_string(), Internal.label());
        assert_eq!(BadRequest.to_string(), BadRequest.label());
        assert_eq!(TelegramAuth.to_string(), TelegramAuth.label());
        assert_eq!(InvalidJwt.to_string(), InvalidJwt.label());
        assert_eq!(Database.to_string(), Database.label());
        assert_eq!(Service.to_string(), Service.label());
        assert_eq!(Config.to_string(), Config.label());
        assert_eq!(Turnkey.to_string(), Turnkey.label());
        assert_eq!(Timeout.to_string(), Timeout.label());
        assert_eq!(Network.to_string(), Network.label());
        assert_eq!(RateLimited.to_string(), RateLimited.label());
        assert_eq!(
            DependencyUnavailable.to_string(),
            DependencyUnavailable.label()
        );
        assert_eq!(Serialization.to_string(), Serialization.label());
        assert_eq!(Deserialization.to_string(), Deserialization.label());
        assert_eq!(ExternalApi.to_string(), ExternalApi.label());
        assert_eq!(Queue.to_string(), Queue.label());
        assert_eq!(Cache.to_string(), Cache.label());
    }

    #[test]
    fn error_trait_impl() {
        use core::error::Error;
        let kind = Internal;
        let err: &dyn Error = &kind;
        assert!(err.source().is_none());
    }

    #[test]
    fn clone_and_copy() {
        let kind1 = Internal;
        let kind2 = kind1;
        let kind3 = kind1;
        assert_eq!(kind1, kind2);
        assert_eq!(kind2, kind3);
    }

    #[test]
    fn debug_format() {
        let debug_str = format!("{:?}", Internal);
        assert_eq!(debug_str, "Internal");
        let debug_str = format!("{:?}", NotFound);
        assert_eq!(debug_str, "NotFound");
    }
}
