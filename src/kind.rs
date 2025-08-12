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
/// Keep it small, stable, and framework-agnostic. Each variant should have a
/// clear, documented meaning and a predictable mapping to an HTTP status code.
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum AppErrorKind {
    // Generic, client-visible failures
    #[error("Not found")]
    NotFound,
    #[error("Validation error")]
    Validation,
    #[error("Conflict")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not implemented")]
    NotImplemented,
    #[error("Internal server error")]
    Internal,
    #[error("Bad request")]
    BadRequest,

    // Domain-specific categories (map conservatively)
    #[error("Telegram authentication error")]
    TelegramAuth,
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("Database error")]
    Database,
    #[error("Service error")]
    Service,
    #[error("Configuration error")]
    Config,
    #[error("Turnkey error")]
    Turnkey,

    // Infrastructure / network
    #[error("Operation timed out")]
    Timeout,
    #[error("Network error")]
    Network,
    #[error("Rate limit exceeded")]
    RateLimited,
    #[error("External dependency unavailable")]
    DependencyUnavailable,

    // Serialization / external API / infra subsystems
    #[error("Serialization error")]
    Serialization,
    #[error("Deserialization error")]
    Deserialization,
    #[error("External API error")]
    ExternalApi,
    #[error("Queue processing error")]
    Queue,
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
