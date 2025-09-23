//! Axum integration: `IntoResponse` for [`AppError`] and helper status mapping.
//!
//! Enabled with the `axum` feature flag.
//!
//! ## What it does
//! - Adds an inherent `http_status()` on [`AppError`] that returns
//!   `axum::http::StatusCode` based on [`AppErrorKind`].
//! - Implements `IntoResponse` for [`AppError`] so handlers can `return
//!   Err(...)` or directly `return AppError::...(...)` and get an RFC7807
//!   problem+json body.
//! - Flushes [`AppError`] telemetry at the HTTP boundary (tracing event,
//!   metrics counter, lazy backtrace).
//!
//! ## Wire payload
//!
//! The response body is [`ProblemJson`] with fields `{ type, title, status,
//! detail, code, grpc, metadata }`. `detail` is redacted automatically when
//! the error is marked private.
//!
//! ## Example
//!
//! ```rust,ignore
//! use axum::{routing::get, Router};
//! use masterror::{AppError, AppErrorKind, AppResult};
//!
//! async fn handler() -> AppResult<&'static str> {
//!     Err(AppError::forbidden("no access"))
//! }
//!
//! let app = Router::new().route("/demo", get(handler));
//! ```
//!
//! ## Notes
//!
//! - Do not duplicate the `IntoResponse` implementation elsewhere (e.g. in
//!   `response.rs`). There must be exactly one impl in the crate.
//! - This module does not expose internal error sources; only `kind`, `status`,
//!   and optional public `message` are surfaced.

#![cfg(feature = "axum")]
#![cfg_attr(docsrs, doc(cfg(feature = "axum")))]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}
};

use crate::{AppError, response::ProblemJson};

impl AppError {
    /// Map this error to an HTTP status derived from its [`AppErrorKind`].
    ///
    /// This is the transport-specific view over the framework-agnostic
    /// `AppErrorKind::http_status()` mapping.
    #[inline]
    pub fn http_status(&self) -> StatusCode {
        // `kind` is a field, not a method.
        self.kind.status_code()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let err = self;
        let problem = ProblemJson::from_app_error(err);
        problem.into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;
    use crate::{AppCode, AppErrorKind};

    // --- http_status mapping -------------------------------------------------

    #[test]
    fn http_status_maps_from_kind() {
        let e = AppError::forbidden("nope");
        // sanity: kind -> 403
        assert_eq!(e.http_status(), StatusCode::FORBIDDEN);

        let e = AppError::validation("bad");
        assert_eq!(e.http_status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    // --- IntoResponse with JSON body (serde_json enabled) --------------------

    #[tokio::test]
    async fn into_response_builds_problem_json_with_headers() {
        use axum::{
            body::to_bytes,
            http::header::{CONTENT_TYPE, RETRY_AFTER, WWW_AUTHENTICATE},
            response::IntoResponse
        };

        let app_err = AppError::unauthorized("missing token")
            .with_retry_after_secs(7)
            .with_www_authenticate("Bearer realm=\"api\"");
        let mut resp = app_err.into_response();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .expect("content-type header");
        assert_eq!(content_type, "application/problem+json");

        let retry_after = resp
            .headers()
            .get(RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .expect("retry-after header");
        assert_eq!(retry_after, "7");

        let www_authenticate = resp
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|value| value.to_str().ok())
            .expect("www-authenticate header");
        assert_eq!(www_authenticate, "Bearer realm=\"api\"");

        let bytes = to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("read body");
        let body: crate::response::ProblemJson =
            serde_json::from_slice(&bytes).expect("json body");

        assert_eq!(body.status, 401);
        assert!(matches!(body.code, AppCode::Unauthorized));
        assert_eq!(body.detail.as_deref(), Some("missing token"));
        assert!(body.metadata.is_none());
        assert!(body.grpc.is_some());
    }

    #[tokio::test]
    async fn redacted_errors_hide_detail() {
        use axum::{body::to_bytes, response::IntoResponse};

        let app_err = AppError::internal("secret").redactable();
        let mut resp = app_err.into_response();

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let bytes = to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("read body");
        let body: crate::response::ProblemJson =
            serde_json::from_slice(&bytes).expect("json body");

        assert!(body.detail.is_none());
        assert!(body.metadata.is_none());
    }
}
