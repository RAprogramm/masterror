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

#[cfg(feature = "axum")]
use axum::http::StatusCode;

/// Canonical application error taxonomy.
///
/// Keep it small, stable, and framework-agnostic. Each variant has a clear,
/// documented meaning and a predictable mapping to an HTTP status code.
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum AppErrorKind {
    // ── Generic, client-visible failures (4xx/5xx) ────────────────────────────
    /// Resource does not exist or is not visible to the caller.
    ///
    /// Maps to **404 Not Found**.
    #[error("Not found")]
    NotFound,

    /// Input failed validation (shape, constraints, business rules).
    ///
    /// Prefer this over `BadRequest` when you validate structured input.
    /// Maps to **422 Unprocessable Entity**.
    #[error("Validation error")]
    Validation,

    /// State conflict with an existing resource or concurrent update.
    ///
    /// Typical cases: unique key violation, version mismatch (ETag).
    /// Maps to **409 Conflict**.
    #[error("Conflict")]
    Conflict,

    /// Authentication required or failed (missing/invalid credentials).
    ///
    /// Maps to **401 Unauthorized**.
    #[error("Unauthorized")]
    Unauthorized,

    /// Authenticated but not allowed to perform the operation.
    ///
    /// Maps to **403 Forbidden**.
    #[error("Forbidden")]
    Forbidden,

    /// Operation is not implemented or not supported by this deployment.
    ///
    /// Maps to **501 Not Implemented**.
    #[error("Not implemented")]
    NotImplemented,

    /// Unexpected server-side failure not captured by more specific kinds.
    ///
    /// Use sparingly; prefer a more precise category when possible.
    /// Maps to **500 Internal Server Error**.
    #[error("Internal server error")]
    Internal,

    /// Malformed request or missing required parameters.
    ///
    /// Prefer `Validation` for structured input with field-level issues.
    /// Maps to **400 Bad Request**.
    #[error("Bad request")]
    BadRequest,

    // ── Domain-specific categories (map conservatively) ───────────────────────
    /// Telegram authentication flow failed (signature, timestamp, or payload).
    ///
    /// Treated as an authentication failure.
    /// Maps to **401 Unauthorized**.
    #[error("Telegram authentication error")]
    TelegramAuth,

    /// Provided JWT is invalid (expired, malformed, wrong signature/claims).
    ///
    /// Treated as an authentication failure.
    /// Maps to **401 Unauthorized**.
    #[error("Invalid JWT")]
    InvalidJwt,

    /// Database-related failure (query, connection, migration, etc.).
    ///
    /// Keep driver-specific details out of the public contract.
    /// Maps to **500 Internal Server Error**.
    #[error("Database error")]
    Database,

    /// Generic service-layer failure (business logic or internal
    /// orchestration).
    ///
    /// Use when no more specific category applies.
    /// Maps to **500 Internal Server Error**.
    #[error("Service error")]
    Service,

    /// Configuration error (missing/invalid environment or runtime config).
    ///
    /// Maps to **500 Internal Server Error**.
    #[error("Configuration error")]
    Config,

    /// Failure in the Turnkey subsystem/integration.
    ///
    /// Maps to **500 Internal Server Error**.
    #[error("Turnkey error")]
    Turnkey,

    // ── Infrastructure / network ──────────────────────────────────────────────
    /// Operation did not complete within the allotted time.
    ///
    /// Typically returned by timeouts around I/O or remote calls.
    /// Maps to **504 Gateway Timeout**.
    #[error("Operation timed out")]
    Timeout,

    /// Network-level error (DNS, connect, TLS, request build).
    ///
    /// For upstream HTTP status failures use `ExternalApi` instead.
    /// Maps to **503 Service Unavailable**.
    #[error("Network error")]
    Network,

    /// Client exceeded rate limits or quota.
    ///
    /// Maps to **429 Too Many Requests**.
    #[error("Rate limit exceeded")]
    RateLimited,

    /// External dependency is unavailable or degraded.
    ///
    /// Examples: cache down, message broker unreachable, third-party outage.
    /// Maps to **503 Service Unavailable**.
    #[error("External dependency unavailable")]
    DependencyUnavailable,

    // ── Serialization / external API / infra subsystems ───────────────────────
    /// Failed to serialize data (encode).
    ///
    /// Maps to **500 Internal Server Error**.
    #[error("Serialization error")]
    Serialization,

    /// Failed to deserialize data (decode).
    ///
    /// Maps to **500 Internal Server Error**.
    #[error("Deserialization error")]
    Deserialization,

    /// Upstream API returned an error or the call failed at protocol level.
    ///
    /// Use `Network` for connect/build failures; use this for HTTP status
    /// errors. Maps to **500 Internal Server Error** by default.
    #[error("External API error")]
    ExternalApi,

    /// Queue processing failure (publish/consume/ack).
    ///
    /// Maps to **500 Internal Server Error**.
    #[error("Queue processing error")]
    Queue,

    /// Cache subsystem failure (read/write/encoding).
    ///
    /// Maps to **500 Internal Server Error**.
    #[error("Cache error")]
    Cache
}

impl AppErrorKind {
    /// Framework-agnostic mapping to an HTTP status code (`u16`).
    ///
    /// This mapping is intentionally conservative and stable. It should **not**
    /// leak environment-specific details (e.g. DB driver error codes or HTTP
    /// library errors).
    pub fn http_status(&self) -> u16 {
        match self {
            // 4xx — client errors
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

            // 5xx — server/infrastructure errors
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
}

#[cfg(test)]
mod tests {
    use super::AppErrorKind::*;

    #[test]
    fn http_status_is_stable() {
        // Simple spot checks to guard against accidental remaps.
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
}
