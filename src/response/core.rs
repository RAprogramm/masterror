use http::StatusCode;
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{AppCode, AppError, AppResult};

/// Retry advice intended for API clients.
///
/// When present, HTTP adapters set the `Retry-After` header with the number of
/// seconds.
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct RetryAdvice {
    /// Number of seconds the client should wait before retrying.
    pub after_seconds: u64
}

/// Public, wire-level error payload for HTTP APIs.
///
/// This type is serialized to JSON (or another transport format) and forms part
/// of the stable wire contract between services and clients.
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    /// HTTP status code (e.g. 404, 422, 500).
    pub status:  u16,
    /// Stable machine-readable error code (enum).
    pub code:    AppCode,
    /// Human-oriented, non-sensitive message.
    pub message: String,

    /// Optional structured details (JSON if `serde_json` is enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "serde_json")]
    pub details: Option<JsonValue>,

    /// Optional textual details (if `serde_json` is *not* enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(not(feature = "serde_json"))]
    pub details: Option<String>,

    /// Optional retry advice. If present, integrations set the `Retry-After`
    /// header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryAdvice>,

    /// Optional authentication challenge. If present, integrations set the
    /// `WWW-Authenticate` header.
    ///
    /// Example value: `Bearer realm="api", error="invalid_token"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub www_authenticate: Option<String>
}

impl ErrorResponse {
    /// Construct a new [`ErrorResponse`] with a status code, a stable
    /// [`AppCode`], and a public message.
    ///
    /// # Errors
    ///
    /// Returns [`AppError`] if `status` is not a valid HTTP status code.
    #[allow(clippy::result_large_err)]
    pub fn new(status: u16, code: AppCode, message: impl Into<String>) -> AppResult<Self> {
        StatusCode::from_u16(status)
            .map_err(|_| AppError::bad_request(format!("invalid HTTP status: {status}")))?;
        Ok(Self {
            status,
            code,
            message: message.into(),
            details: None,
            retry: None,
            www_authenticate: None
        })
    }

    /// Convert numeric [`status`](ErrorResponse::status) into [`StatusCode`].
    ///
    /// Invalid codes default to `StatusCode::INTERNAL_SERVER_ERROR`.
    ///
    /// # Examples
    /// ```
    /// use http::StatusCode;
    /// use masterror::{AppCode, ErrorResponse};
    ///
    /// let resp = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
    /// assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    /// ```
    #[must_use]
    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
