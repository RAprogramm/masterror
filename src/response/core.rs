// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

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
    /// Stable machine-readable error code.
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

    /// Formatter exposing internals for diagnostic logs.
    #[must_use]
    pub fn internal(&self) -> crate::response::internal::ErrorResponseFormatter<'_> {
        crate::response::internal::ErrorResponseFormatter::new(self)
    }
}
use alloc::{format, string::String};

#[cfg(test)]
mod tests {
    use http::StatusCode;

    use super::*;

    #[test]
    fn new_creates_error_response_with_valid_status() {
        let result = ErrorResponse::new(404, AppCode::NotFound, "missing");
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.status, 404);
        assert_eq!(resp.code, AppCode::NotFound);
        assert_eq!(resp.message, "missing");
        assert!(resp.details.is_none());
        assert!(resp.retry.is_none());
        assert!(resp.www_authenticate.is_none());
    }

    #[test]
    fn new_rejects_invalid_http_status_code() {
        let result = ErrorResponse::new(1000, AppCode::Internal, "bad status");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, crate::AppErrorKind::BadRequest);
    }

    #[test]
    fn new_accepts_string_types() {
        let owned = ErrorResponse::new(200, AppCode::Internal, String::from("owned"));
        assert!(owned.is_ok());

        let borrowed = ErrorResponse::new(201, AppCode::Internal, "borrowed");
        assert!(borrowed.is_ok());
    }

    #[test]
    fn status_code_converts_valid_status() {
        let resp = ErrorResponse::new(404, AppCode::NotFound, "test").unwrap();
        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn status_code_defaults_to_internal_server_error_for_invalid() {
        let mut resp = ErrorResponse::new(200, AppCode::Internal, "test").unwrap();
        resp.status = 1000; // Manually set invalid status (HTTP codes are 100-999)
        assert_eq!(resp.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn retry_advice_fields_are_accessible() {
        let advice = RetryAdvice {
            after_seconds: 60
        };
        assert_eq!(advice.after_seconds, 60);
    }

    #[test]
    fn retry_advice_supports_copy_and_equality() {
        let advice1 = RetryAdvice {
            after_seconds: 30
        };
        let advice2 = advice1; // Copy, not clone
        assert_eq!(advice1, advice2);
    }

    #[test]
    fn error_response_supports_clone() {
        let resp1 = ErrorResponse::new(500, AppCode::Internal, "error").unwrap();
        let resp2 = resp1.clone();
        assert_eq!(resp1.status, resp2.status);
        assert_eq!(resp1.code, resp2.code);
        assert_eq!(resp1.message, resp2.message);
    }

    #[test]
    fn error_response_with_retry_advice() {
        let mut resp = ErrorResponse::new(429, AppCode::RateLimited, "too many").unwrap();
        resp.retry = Some(RetryAdvice {
            after_seconds: 120
        });
        assert!(resp.retry.is_some());
        assert_eq!(resp.retry.unwrap().after_seconds, 120);
    }

    #[test]
    fn error_response_with_www_authenticate() {
        let mut resp = ErrorResponse::new(401, AppCode::Unauthorized, "auth required").unwrap();
        resp.www_authenticate = Some("Bearer realm=\"api\"".to_owned());
        assert!(resp.www_authenticate.is_some());
        assert_eq!(
            resp.www_authenticate.as_deref(),
            Some("Bearer realm=\"api\"")
        );
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn error_response_with_json_details() {
        use serde_json::json;
        let mut resp = ErrorResponse::new(422, AppCode::Validation, "invalid").unwrap();
        resp.details = Some(json!({"field": "email", "error": "invalid format"}));
        assert!(resp.details.is_some());
    }

    #[cfg(not(feature = "serde_json"))]
    #[test]
    fn error_response_with_text_details() {
        let mut resp = ErrorResponse::new(422, AppCode::Validation, "invalid").unwrap();
        resp.details = Some("Field validation failed".to_owned());
        assert!(resp.details.is_some());
        assert_eq!(resp.details.as_deref(), Some("Field validation failed"));
    }

    #[test]
    fn common_http_status_codes_work() {
        let codes = vec![
            (200, StatusCode::OK),
            (201, StatusCode::CREATED),
            (400, StatusCode::BAD_REQUEST),
            (401, StatusCode::UNAUTHORIZED),
            (403, StatusCode::FORBIDDEN),
            (404, StatusCode::NOT_FOUND),
            (422, StatusCode::UNPROCESSABLE_ENTITY),
            (429, StatusCode::TOO_MANY_REQUESTS),
            (500, StatusCode::INTERNAL_SERVER_ERROR),
            (502, StatusCode::BAD_GATEWAY),
            (503, StatusCode::SERVICE_UNAVAILABLE),
            (504, StatusCode::GATEWAY_TIMEOUT),
        ];

        for (num, expected) in codes {
            let resp = ErrorResponse::new(num, AppCode::Internal, "test").unwrap();
            assert_eq!(resp.status_code(), expected);
        }
    }
}
