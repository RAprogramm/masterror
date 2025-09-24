use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::kind::AppErrorKind;

/// Stable machine-readable error code exposed to clients.
///
/// Values are serialized as **SCREAMING_SNAKE_CASE** strings (e.g.,
/// `"NOT_FOUND"`). This type is part of the public wire contract.
///
/// Design rules:
/// - Keep the set small and meaningful.
/// - Prefer adding new variants over overloading existing ones.
/// - Do not encode private/internal details in codes.
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppCode {
    // ───────────── 4xx family (client-visible categories) ─────────────
    /// Resource does not exist or is not visible to the caller.
    ///
    /// Typically mapped to HTTP **404 Not Found**.
    NotFound,

    /// Input failed validation (shape, constraints, business rules).
    ///
    /// Typically mapped to HTTP **422 Unprocessable Entity**.
    Validation,

    /// State conflict with an existing resource or concurrent update.
    ///
    /// Typically mapped to HTTP **409 Conflict**.
    Conflict,

    /// Attempted to create a user that already exists (unique constraint).
    ///
    /// Typically mapped to HTTP **409 Conflict**.
    UserAlreadyExists,

    /// Authentication required or failed (missing/invalid credentials).
    ///
    /// Typically mapped to HTTP **401 Unauthorized**.
    Unauthorized,

    /// Authenticated but not allowed to perform the operation.
    ///
    /// Typically mapped to HTTP **403 Forbidden**.
    Forbidden,

    /// Operation is not implemented or not supported by this deployment.
    ///
    /// Typically mapped to HTTP **501 Not Implemented**.
    NotImplemented,

    /// Malformed request or missing required parameters.
    ///
    /// Typically mapped to HTTP **400 Bad Request**.
    BadRequest,

    /// Client exceeded rate limits or quota.
    ///
    /// Typically mapped to HTTP **429 Too Many Requests**.
    RateLimited,

    /// Telegram authentication flow failed (signature, timestamp, or payload).
    ///
    /// Typically mapped to HTTP **401 Unauthorized**.
    TelegramAuth,

    /// Provided JWT is invalid (expired, malformed, wrong signature/claims).
    ///
    /// Typically mapped to HTTP **401 Unauthorized**.
    InvalidJwt,

    // ───────────── 5xx family (server/infra categories) ─────────────
    /// Unexpected server-side failure not captured by more specific kinds.
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Internal,

    /// Database-related failure (query, connection, migration, etc.).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Database,

    /// Generic service-layer failure (business logic or orchestration).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Service,

    /// Configuration error (missing/invalid environment or runtime config).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Config,

    /// Failure in the Turnkey subsystem/integration.
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Turnkey,

    /// Operation did not complete within the allotted time.
    ///
    /// Typically mapped to HTTP **504 Gateway Timeout**.
    Timeout,

    /// Network-level error (DNS, connect, TLS, request build).
    ///
    /// Typically mapped to HTTP **503 Service Unavailable**.
    Network,

    /// External dependency is unavailable or degraded (cache, broker,
    /// third-party).
    ///
    /// Typically mapped to HTTP **503 Service Unavailable**.
    DependencyUnavailable,

    /// Failed to serialize data (encode).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Serialization,

    /// Failed to deserialize data (decode).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Deserialization,

    /// Upstream API returned an error or protocol-level failure.
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    ExternalApi,

    /// Queue processing failure (publish/consume/ack).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Queue,

    /// Cache subsystem failure (read/write/encoding).
    ///
    /// Typically mapped to HTTP **500 Internal Server Error**.
    Cache
}

impl AppCode {
    /// Get the canonical string form of this code (SCREAMING_SNAKE_CASE).
    ///
    /// This is equivalent to how the code is serialized to JSON.
    pub const fn as_str(&self) -> &'static str {
        match self {
            // 4xx
            AppCode::NotFound => "NOT_FOUND",
            AppCode::Validation => "VALIDATION",
            AppCode::Conflict => "CONFLICT",
            AppCode::UserAlreadyExists => "USER_ALREADY_EXISTS",
            AppCode::Unauthorized => "UNAUTHORIZED",
            AppCode::Forbidden => "FORBIDDEN",
            AppCode::NotImplemented => "NOT_IMPLEMENTED",
            AppCode::BadRequest => "BAD_REQUEST",
            AppCode::RateLimited => "RATE_LIMITED",
            AppCode::TelegramAuth => "TELEGRAM_AUTH",
            AppCode::InvalidJwt => "INVALID_JWT",

            // 5xx
            AppCode::Internal => "INTERNAL",
            AppCode::Database => "DATABASE",
            AppCode::Service => "SERVICE",
            AppCode::Config => "CONFIG",
            AppCode::Turnkey => "TURNKEY",
            AppCode::Timeout => "TIMEOUT",
            AppCode::Network => "NETWORK",
            AppCode::DependencyUnavailable => "DEPENDENCY_UNAVAILABLE",
            AppCode::Serialization => "SERIALIZATION",
            AppCode::Deserialization => "DESERIALIZATION",
            AppCode::ExternalApi => "EXTERNAL_API",
            AppCode::Queue => "QUEUE",
            AppCode::Cache => "CACHE"
        }
    }
}

impl Display for AppCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Stable human/machine readable form matching JSON representation.
        f.write_str(self.as_str())
    }
}

impl From<AppErrorKind> for AppCode {
    /// Map internal taxonomy (`AppErrorKind`) to public machine code
    /// (`AppCode`).
    ///
    /// The mapping is 1:1 today and intentionally conservative.
    fn from(kind: AppErrorKind) -> Self {
        match kind {
            // 4xx
            AppErrorKind::NotFound => Self::NotFound,
            AppErrorKind::Validation => Self::Validation,
            AppErrorKind::Conflict => Self::Conflict,
            AppErrorKind::Unauthorized => Self::Unauthorized,
            AppErrorKind::Forbidden => Self::Forbidden,
            AppErrorKind::NotImplemented => Self::NotImplemented,
            AppErrorKind::BadRequest => Self::BadRequest,
            AppErrorKind::RateLimited => Self::RateLimited,
            AppErrorKind::TelegramAuth => Self::TelegramAuth,
            AppErrorKind::InvalidJwt => Self::InvalidJwt,

            // 5xx
            AppErrorKind::Internal => Self::Internal,
            AppErrorKind::Database => Self::Database,
            AppErrorKind::Service => Self::Service,
            AppErrorKind::Config => Self::Config,
            AppErrorKind::Turnkey => Self::Turnkey,
            AppErrorKind::Timeout => Self::Timeout,
            AppErrorKind::Network => Self::Network,
            AppErrorKind::DependencyUnavailable => Self::DependencyUnavailable,
            AppErrorKind::Serialization => Self::Serialization,
            AppErrorKind::Deserialization => Self::Deserialization,
            AppErrorKind::ExternalApi => Self::ExternalApi,
            AppErrorKind::Queue => Self::Queue,
            AppErrorKind::Cache => Self::Cache
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppCode, AppErrorKind};

    #[test]
    fn as_str_matches_json_serde_names() {
        assert_eq!(AppCode::NotFound.as_str(), "NOT_FOUND");
        assert_eq!(AppCode::RateLimited.as_str(), "RATE_LIMITED");
        assert_eq!(
            AppCode::DependencyUnavailable.as_str(),
            "DEPENDENCY_UNAVAILABLE"
        );
    }

    #[test]
    fn mapping_from_kind_is_stable() {
        // Spot checks to guard against accidental remaps.
        assert!(matches!(
            AppCode::from(AppErrorKind::NotFound),
            AppCode::NotFound
        ));
        assert!(matches!(
            AppCode::from(AppErrorKind::Validation),
            AppCode::Validation
        ));
        assert!(matches!(
            AppCode::from(AppErrorKind::Internal),
            AppCode::Internal
        ));
        assert!(matches!(
            AppCode::from(AppErrorKind::Timeout),
            AppCode::Timeout
        ));
    }

    #[test]
    fn display_uses_screaming_snake_case() {
        assert_eq!(AppCode::BadRequest.to_string(), "BAD_REQUEST");
    }
}
