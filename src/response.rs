//! Wire-level error payload and HTTP integration.
//!
//! # Purpose
//!
//! [`ErrorResponse`] is a stable JSON structure intended to be returned
//! directly from HTTP handlers. It represents the **public-facing contract**
//! for error reporting in web APIs.
//!
//! It deliberately contains only *safe-to-expose* fields:
//!
//! - [`status`](ErrorResponse::status): HTTP status code chosen by the service
//! - [`code`](ErrorResponse::code): stable, machine-readable error code
//!   ([`AppCode`])
//! - [`message`](ErrorResponse::message): human-oriented, non-sensitive text
//! - [`details`](ErrorResponse::details): optional structured payload
//!   (`serde_json::Value` if the `serde_json` feature is enabled, otherwise
//!   plain text)
//! - [`retry`](ErrorResponse::retry): optional retry advice, rendered as the
//!   `Retry-After` header in HTTP adapters
//! - [`www_authenticate`](ErrorResponse::www_authenticate): optional
//!   authentication challenge string, rendered as the `WWW-Authenticate` header
//!
//! Internal error sources (the [`std::error::Error`] chain inside [`AppError`])
//! are **never leaked** into this type. They should be logged at the boundary,
//! but not serialized into responses.
//!
//! # Example
//!
//! ```rust
//! use masterror::{AppCode, ErrorResponse};
//!
//! let resp =
//!     ErrorResponse::new(404, AppCode::NotFound, "User not found").with_retry_after_secs(30);
//! ```
//!
//! With `serde_json` enabled:
//!
//! ```rust
//! # #[cfg(feature = "serde_json")]
//! # {
//! use masterror::{AppCode, ErrorResponse};
//! use serde_json::json;
//!
//! let resp = ErrorResponse::new(422, AppCode::Validation, "Invalid input")
//!     .with_details_json(json!({"field": "email", "error": "invalid"}));
//! # }
//! ```
//!
//! # Migration note
//!
//! Prior to version 0.3.0, `ErrorResponse::new` accepted only `(status,
//! message)`. This was replaced with `(status, code, message)` to expose a
//! stable machine-readable code. A temporary [`ErrorResponse::new_legacy`] is
//! provided as a deprecated shim.

use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{app_error::AppError, code::AppCode};

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
    #[must_use]
    pub fn new(status: u16, code: AppCode, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            details: None,
            retry: None,
            www_authenticate: None
        }
    }

    /// Attach plain-text details (available when `serde_json` is disabled).
    #[cfg(not(feature = "serde_json"))]
    #[must_use]
    pub fn with_details_text(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Attach structured JSON details (available when `serde_json` is enabled).
    #[cfg(feature = "serde_json")]
    #[must_use]
    pub fn with_details_json(mut self, details: JsonValue) -> Self {
        self.details = Some(details);
        self
    }

    /// Attach retry advice (number of seconds).
    ///
    /// When present, integrations set the `Retry-After` header automatically.
    #[must_use]
    pub fn with_retry_after_secs(mut self, secs: u64) -> Self {
        self.retry = Some(RetryAdvice {
            after_seconds: secs
        });
        self
    }

    /// Attach an authentication challenge string.
    ///
    /// When present, integrations set the `WWW-Authenticate` header
    /// automatically.
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self
    }
}

/// Legacy constructor retained for migration purposes.
///
/// Deprecated: prefer [`ErrorResponse::new`] with an [`AppCode`] argument.
#[deprecated(note = "Use new(status, code, message) instead")]
impl ErrorResponse {
    /// Construct an error response with only `(status, message)`.
    ///
    /// This defaults the code to [`AppCode::Internal`]. Kept temporarily to
    /// ease migration from versions prior to 0.3.0.
    #[must_use]
    pub fn new_legacy(status: u16, message: impl Into<String>) -> Self {
        Self::new(status, AppCode::Internal, message)
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Concise string form, safe for logs and debugging.
        write!(f, "{} {:?}: {}", self.status, self.code, self.message)
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        let status = err.kind.http_status();
        let code = AppCode::from(err.kind);

        let message = err
            .message
            .as_deref()
            .unwrap_or("An error occurred")
            .to_owned();

        Self {
            status,
            code,
            message,
            details: None,
            retry: None,
            www_authenticate: None
        }
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    //! Axum integration: implements [`IntoResponse`] for [`ErrorResponse`].
    //!
    //! Behavior:
    //! - Serializes the response as JSON with the given status.
    //! - Adds `Retry-After` if [`ErrorResponse::retry`] is present.
    //! - Adds `WWW-Authenticate` if [`ErrorResponse::www_authenticate`] is
    //!   present.

    use axum::{
        Json,
        http::{
            HeaderValue, StatusCode,
            header::{RETRY_AFTER, WWW_AUTHENTICATE}
        },
        response::{IntoResponse, Response}
    };

    use super::ErrorResponse;

    impl IntoResponse for ErrorResponse {
        fn into_response(self) -> Response {
            let status =
                StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            // Serialize JSON body first (borrow self for payload).
            let mut response = (status, Json(&self)).into_response();

            if let Some(retry) = self.retry
                && let Ok(hv) = HeaderValue::from_str(&retry.after_seconds.to_string())
            {
                response.headers_mut().insert(RETRY_AFTER, hv);
            }
            if let Some(ch) = &self.www_authenticate
                && let Ok(hv) = HeaderValue::from_str(ch)
            {
                response.headers_mut().insert(WWW_AUTHENTICATE, hv);
            }

            // Explicitly consume self to satisfy ownership.
            let _ = self;
            response
        }
    }
}

#[cfg(feature = "actix")]
mod actix_impl {
    //! Actix integration: implements [`Responder`] for [`ErrorResponse`].
    //!
    //! Behavior:
    //! - Serializes the response as JSON with the given status.
    //! - Adds `Retry-After` if [`ErrorResponse::retry`] is present.
    //! - Adds `WWW-Authenticate` if [`ErrorResponse::www_authenticate`] is
    //!   present.

    use actix_web::{
        HttpRequest, HttpResponse, Responder,
        body::BoxBody,
        http::{
            StatusCode,
            header::{RETRY_AFTER, WWW_AUTHENTICATE}
        }
    };

    use super::ErrorResponse;

    impl Responder for ErrorResponse {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
            let status =
                StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            let mut builder = HttpResponse::build(status);
            if let Some(retry) = self.retry {
                builder.insert_header((RETRY_AFTER, retry.after_seconds.to_string()));
            }
            if let Some(ref ch) = self.www_authenticate {
                // Pass &str, not &String, to satisfy TryIntoHeaderPair
                builder.insert_header((WWW_AUTHENTICATE, ch.as_str()));
            }
            builder.json(self)
        }
    }
}
