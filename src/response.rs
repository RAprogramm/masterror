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
//!   `Retry-After` header in HTTP adapters; set via
//!   [`with_retry_after_secs`](ErrorResponse::with_retry_after_secs) or
//!   [`with_retry_after_duration`](ErrorResponse::with_retry_after_duration)
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
//! use std::time::Duration;
//!
//! use masterror::{AppCode, ErrorResponse};
//!
//! let resp = ErrorResponse::new(404, AppCode::NotFound, "User not found")
//!     .expect("status")
//!     .with_retry_after_duration(Duration::from_secs(30));
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
//!     .expect("status")
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

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult},
    time::Duration
};

use http::StatusCode;
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde_json")]
use serde_json::{Value as JsonValue, to_value};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::{
    app_error::{AppError, AppResult},
    code::AppCode
};

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

    /// Serialize and attach structured details from any [`Serialize`] value.
    ///
    /// # Errors
    ///
    /// Returns [`AppError`] if serialization fails.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{AppCode, ErrorResponse};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Extra {
    ///     reason: String
    /// }
    ///
    /// let payload = Extra {
    ///     reason: "missing".into()
    /// };
    /// let resp = ErrorResponse::new(404, AppCode::NotFound, "no user")
    ///     .expect("status")
    ///     .with_details(payload)
    ///     .expect("details");
    /// assert!(resp.details.is_some());
    /// # }
    /// ```
    #[cfg(feature = "serde_json")]
    pub fn with_details<T>(self, payload: T) -> AppResult<Self>
    where
        T: Serialize
    {
        let details = to_value(payload).map_err(|e| AppError::bad_request(e.to_string()))?;
        Ok(self.with_details_json(details))
    }

    /// Attach retry advice (number of seconds).
    ///
    /// See [`with_retry_after_duration`](Self::with_retry_after_duration) for
    /// using a [`Duration`]. When present, integrations set the `Retry-After`
    /// header automatically.
    #[must_use]
    pub fn with_retry_after_secs(mut self, secs: u64) -> Self {
        self.retry = Some(RetryAdvice {
            after_seconds: secs
        });
        self
    }

    /// Attach retry advice as a [`Duration`].
    ///
    /// Equivalent to [`with_retry_after_secs`](Self::with_retry_after_secs).
    /// When present, integrations set the `Retry-After` header automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use masterror::{AppCode, ErrorResponse};
    ///
    /// let resp = ErrorResponse::new(503, AppCode::Internal, "retry later")
    ///     .expect("status")
    ///     .with_retry_after_duration(Duration::from_secs(60));
    /// assert_eq!(resp.retry.expect("retry").after_seconds, 60);
    /// ```
    #[must_use]
    pub fn with_retry_after_duration(self, dur: Duration) -> Self {
        self.with_retry_after_secs(dur.as_secs())
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
        let msg = message.into();
        Self::new(status, AppCode::Internal, msg.clone()).unwrap_or(Self {
            status:           500,
            code:             AppCode::Internal,
            message:          msg,
            details:          None,
            retry:            None,
            www_authenticate: None
        })
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
            .clone()
            .unwrap_or(Cow::Borrowed("An error occurred"))
            .into_owned();

        Self {
            status,
            code,
            message,
            details: None,
            retry: err.retry,
            www_authenticate: err.www_authenticate.clone()
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
            HeaderValue,
            header::{RETRY_AFTER, WWW_AUTHENTICATE}
        },
        response::{IntoResponse, Response}
    };

    use super::ErrorResponse;
    use crate::AppError;

    impl IntoResponse for ErrorResponse {
        fn into_response(self) -> Response {
            let status = self.status_code();

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

            response
        }
    }

    /// Convert `AppError` into the stable wire model and reuse its
    /// `IntoResponse`.
    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            // Use the canonical mapping defined in `From<&AppError> for ErrorResponse`
            let wire: ErrorResponse = (&self).into();
            wire.into_response()
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
        http::header::{RETRY_AFTER, WWW_AUTHENTICATE}
    };

    use super::ErrorResponse;

    impl Responder for ErrorResponse {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
            let mut builder = HttpResponse::build(
                actix_web::http::StatusCode::from_u16(self.status)
                    .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppErrorKind;

    // --- Basic constructors and fields --------------------------------------

    #[test]
    fn new_sets_status_code_and_message() {
        let e = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
        assert_eq!(e.status, 404);
        assert!(matches!(e.code, AppCode::NotFound));
        assert_eq!(e.message, "missing");
        assert!(e.retry.is_none());
        assert!(e.www_authenticate.is_none());
    }

    #[test]
    fn new_rejects_invalid_status() {
        let err = ErrorResponse::new(0, AppCode::Internal, "boom").expect_err("invalid");
        assert!(matches!(err.kind, AppErrorKind::BadRequest));
    }

    #[test]
    fn with_retry_and_www_authenticate_attach_metadata() {
        let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
            .expect("status")
            .with_retry_after_secs(15)
            .with_www_authenticate(r#"Bearer realm="api""#);
        assert_eq!(e.status, 401);
        assert_eq!(e.retry.unwrap().after_seconds, 15);
        assert_eq!(e.www_authenticate.as_deref(), Some(r#"Bearer realm="api""#));
    }

    #[test]
    fn with_retry_after_duration_attaches_advice() {
        use std::time::Duration;

        let e = ErrorResponse::new(429, AppCode::RateLimited, "slow down")
            .expect("status")
            .with_retry_after_duration(Duration::from_secs(42));
        assert_eq!(e.retry.unwrap().after_seconds, 42);
    }

    #[test]
    fn status_code_maps_invalid_to_internal_server_error() {
        use http::StatusCode;

        let valid = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
        assert_eq!(valid.status_code(), StatusCode::NOT_FOUND);

        let invalid = ErrorResponse {
            status:           1000,
            code:             AppCode::Internal,
            message:          "oops".into(),
            details:          None,
            retry:            None,
            www_authenticate: None
        };
        assert_eq!(invalid.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    // --- Details: JSON vs text ----------------------------------------------

    #[cfg(feature = "serde_json")]
    #[test]
    fn details_json_are_attached() {
        let payload = serde_json::json!({"field": "email", "error": "invalid"});
        let e = ErrorResponse::new(422, AppCode::Validation, "invalid")
            .expect("status")
            .with_details_json(payload.clone());
        assert_eq!(e.status, 422);
        assert!(e.details.is_some());
        assert_eq!(e.details.unwrap(), payload);
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn with_details_serializes_custom_struct() {
        use serde::Serialize;
        use serde_json::json;

        #[derive(Serialize)]
        struct Extra {
            value: i32
        }

        let resp = ErrorResponse::new(400, AppCode::BadRequest, "bad")
            .expect("status")
            .with_details(Extra {
                value: 7
            })
            .expect("details");

        assert_eq!(resp.details.unwrap(), json!({"value": 7}));
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn with_details_propagates_serialization_errors() {
        use serde::{Serialize, Serializer};

        struct Failing;

        impl Serialize for Failing {
            fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer
            {
                Err(serde::ser::Error::custom("nope"))
            }
        }

        let err = ErrorResponse::new(400, AppCode::BadRequest, "bad")
            .expect("status")
            .with_details(Failing)
            .expect_err("serialization should fail");
        assert!(matches!(err.kind, AppErrorKind::BadRequest));
    }

    #[cfg(not(feature = "serde_json"))]
    #[test]
    fn details_text_are_attached() {
        let e = ErrorResponse::new(503, AppCode::DependencyUnavailable, "down")
            .expect("status")
            .with_details_text("retry later");
        assert_eq!(e.status, 503);
        assert_eq!(e.details.as_deref(), Some("retry later"));
    }

    // --- From<&AppError> mapping --------------------------------------------

    #[test]
    fn from_app_error_preserves_status_and_sets_code() {
        let app = crate::AppError::new(AppErrorKind::NotFound, "user");
        let e: ErrorResponse = (&app).into();
        assert_eq!(e.status, 404);
        assert!(matches!(e.code, AppCode::NotFound));
        assert_eq!(e.message, "user");
        assert!(e.retry.is_none());
    }

    #[test]
    fn from_app_error_uses_default_message_when_none() {
        let app = crate::AppError::bare(AppErrorKind::Internal);
        let e: ErrorResponse = (&app).into();
        assert_eq!(e.status, 500);
        assert!(matches!(e.code, AppCode::Internal));
        assert_eq!(e.message, "An error occurred");
    }

    // --- Display formatting --------------------------------------------------

    #[test]
    fn display_is_concise_and_does_not_leak_details() {
        let e = ErrorResponse::new(400, AppCode::BadRequest, "bad").expect("status");
        let s = format!("{}", e);
        assert!(s.contains("400"), "status should be present");
        assert!(
            s.to_lowercase().contains("badrequest")
                || s.contains("BAD_REQUEST")
                || s.contains("BadRequest"),
            "code should be present in some form"
        );
        assert!(s.contains("bad"), "message should be present");
    }

    // --- Legacy constructor (migration shim) --------------------------------

    #[allow(deprecated)]
    #[test]
    fn new_legacy_defaults_to_internal_code() {
        let e = ErrorResponse::new_legacy(500, "boom");
        assert_eq!(e.status, 500);
        assert!(matches!(e.code, AppCode::Internal));
        assert_eq!(e.message, "boom");
    }

    // --- Axum adapter: headers and status -----------------------------------

    #[cfg(feature = "axum")]
    #[test]
    fn axum_into_response_sets_headers_and_status() {
        use axum::{
            http::header::{RETRY_AFTER, WWW_AUTHENTICATE},
            response::IntoResponse
        };

        let resp = ErrorResponse::new(401, AppCode::Unauthorized, "no token")
            .expect("status")
            .with_retry_after_secs(7)
            .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#)
            .into_response();

        assert_eq!(resp.status(), 401);
        let headers = resp.headers();
        assert_eq!(headers.get(RETRY_AFTER).unwrap(), "7");
        assert_eq!(
            headers.get(WWW_AUTHENTICATE).unwrap(),
            r#"Bearer realm="api", error="invalid_token""#
        );
    }

    // --- Actix adapter: headers and status ----------------------------------

    #[cfg(feature = "actix")]
    #[test]
    fn actix_responder_sets_headers_and_status() {
        use actix_web::{
            Responder,
            http::{
                StatusCode,
                header::{RETRY_AFTER, WWW_AUTHENTICATE}
            },
            test::TestRequest
        };

        // Build ErrorResponse with both headers
        let resp = ErrorResponse::new(429, AppCode::RateLimited, "slow down")
            .expect("status")
            .with_retry_after_secs(42)
            .with_www_authenticate("Bearer");

        // Build a minimal HttpRequest for Responder::respond_to
        let req = TestRequest::default().to_http_request();

        // `respond_to` builds HttpResponse synchronously; we can inspect it.
        let http = resp.respond_to(&req);
        assert_eq!(http.status(), StatusCode::TOO_MANY_REQUESTS);

        let headers = http.headers();
        assert_eq!(headers.get(RETRY_AFTER).unwrap(), "42");
        assert_eq!(headers.get(WWW_AUTHENTICATE).unwrap(), "Bearer");
    }

    #[cfg(feature = "actix")]
    #[test]
    fn actix_responder_no_optional_headers_by_default() {
        use actix_web::{
            Responder,
            http::header::{RETRY_AFTER, WWW_AUTHENTICATE},
            test::TestRequest
        };

        let resp = ErrorResponse::new(500, AppCode::Internal, "boom").expect("status");
        let req = TestRequest::default().to_http_request();
        let http = resp.respond_to(&req);

        let headers = http.headers();
        assert!(headers.get(RETRY_AFTER).is_none());
        assert!(headers.get(WWW_AUTHENTICATE).is_none());
    }

    // --- Serde snapshot-ish check -------------------------------------------

    #[cfg(feature = "serde_json")]
    #[test]
    fn serialized_json_contains_core_fields() {
        let e = ErrorResponse::new(404, AppCode::NotFound, "nope")
            .expect("status")
            .with_retry_after_secs(1);
        let s = serde_json::to_string(&e).expect("serialize");
        // Fast contract sanity checks without tying to exact field order
        assert!(s.contains("\"status\":404"));
        assert!(s.contains("\"code\":\"NOT_FOUND\""));
        assert!(s.contains("\"message\":\"nope\""));
        // Retry advice is serialized as nested object
        assert!(s.contains("\"retry\""));
        assert!(s.contains("\"after_seconds\":1"));
    }

    #[cfg(feature = "axum")]
    #[test]
    fn app_error_into_response_maps_status() {
        use axum::response::IntoResponse;

        use crate::AppErrorKind;

        let app = crate::AppError::new(AppErrorKind::Unauthorized, "no token");
        let resp = app.into_response();
        assert_eq!(resp.status(), 401);
    }
}
