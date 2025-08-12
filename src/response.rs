//! Wire-level error payload and HTTP integration.
//!
//! `ErrorResponse` is a small, stable structure intended to be returned from
//! HTTP handlers. It intentionally carries only public information:
//! - `status`: HTTP status code chosen by the application
//! - `message`: human-oriented, non-sensitive message
//! - `details`: optional structured payload (JSON if the `serde_json` feature
//!   is enabled, otherwise a plain string)
//!
//! The crate keeps internal error sources for logs only. Do not leak internals
//! into `message` or `details`.
//!
//! ## Construction
//!
//! ```rust
//! use masterror::ErrorResponse;
//!
//! let resp = ErrorResponse::new(404, "User not found");
//! ```
//!
//! With `serde_json` feature you can attach structured details:
//!
//! ```rust
//! # #[cfg(feature = "serde_json")]
//! # {
//! use masterror::ErrorResponse;
//! use serde_json::json;
//!
//! let resp = ErrorResponse::new(422, "Validation failed")
//!     .with_details_json(json!({"field": "email", "error": "invalid"}));
//! # }
//! ```
//!
//! Without `serde_json`, attach a plain text detail:
//!
//! ```rust
//! #[cfg(not(feature = "serde_json"))]
//! {
//!     use masterror::ErrorResponse;
//!     let resp = ErrorResponse::new(503, "Service unavailable").with_details_text("retry later");
//! }
//! ```
//!
//! ## Mapping from `AppError`
//!
//! If you use this crate’s [`AppError`], you can convert it to `ErrorResponse`:
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind, ErrorResponse};
//!
//! let app_err = AppError::new(AppErrorKind::NotFound, "user_not_found");
//! let resp: ErrorResponse = (&app_err).into();
//! assert_eq!(resp.status, 404);
//! ```
//!
//! ## Axum integration
//!
//! With the `axum` feature enabled, `AppError` implements `IntoResponse`.
//! If you need to return a pre-built `ErrorResponse`, you can do it too:
//!
//! ```rust,ignore
//! # #[cfg(feature = "axum")]
//! use axum::response::IntoResponse;
//! use masterror::ErrorResponse;
//!
//! fn handler() -> impl IntoResponse {
//!     ErrorResponse::new(403, "Forbidden")
//! }
//! ```

use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Public, wire-level error payload for HTTP APIs.
///
/// `details` is an optional, consumer-facing payload:
/// - when `serde_json` is enabled it is JSON
/// - otherwise it is a plain string
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    /// HTTP status code (e.g. 404, 422, 500)
    pub status:  u16,
    /// Human-oriented, non-sensitive message
    pub message: String,

    /// Optional structured details (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "serde_json")]
    pub details: Option<JsonValue>,

    /// Optional textual details (no JSON feature)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(not(feature = "serde_json"))]
    pub details: Option<String>
}

impl ErrorResponse {
    /// Create a new error payload with a status and message.
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            details: None
        }
    }

    /// Attach textual details (available when `serde_json` is disabled).
    #[cfg(not(feature = "serde_json"))]
    pub fn with_details_text(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Attach JSON details (available when `serde_json` is enabled).
    #[cfg(feature = "serde_json")]
    pub fn with_details_json(mut self, details: JsonValue) -> Self {
        self.details = Some(details);
        self
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl From<&crate::app_error::AppError> for ErrorResponse {
    fn from(err: &crate::app_error::AppError) -> Self {
        let status = err.kind.http_status();

        // err.message: Option<String> → String with safe default
        let message: String = match &err.message {
            Some(m) => m.clone(),
            None => "An error occurred".to_string()
        };

        #[cfg(feature = "serde_json")]
        {
            // AppError.details в твоём типе нет → всегда None
            Self {
                status,
                message,
                details: None
            }
        }

        #[cfg(not(feature = "serde_json"))]
        {
            // Нет serde_json → и пытаться сериализовать нечего
            Self {
                status,
                message,
                details: None
            }
        }
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    //! Axum `IntoResponse` implementations.
    //!
    //! `AppError` maps to an `ErrorResponse` with JSON body and appropriate
    //! status code derived from `AppErrorKind`.

    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json
    };

    use super::ErrorResponse;
    use crate::app_error::AppError;

    impl IntoResponse for ErrorResponse {
        fn into_response(self) -> Response {
            // Build response with the provided status code.
            let status =
                StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            (status, Json(self)).into_response()
        }
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            // Log once at the boundary.
            self.log();

            let status = StatusCode::from_u16(self.kind.http_status())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            let body = ErrorResponse::from(&self);
            (status, Json(body)).into_response()
        }
    }
}
