//! Axum integration: `IntoResponse` for [`AppError`] and helper status mapping.
//!
//! Enabled with the `axum` feature flag.
//!
//! ## What it does
//! - Adds an inherent `http_status()` on [`AppError`] that returns
//!   `axum::http::StatusCode` based on [`AppErrorKind`].
//! - Implements `IntoResponse` for [`AppError`] so handlers can `return
//!   Err(...)` or directly `return AppError::...(...)` and get a JSON error
//!   body (when the `serde_json` feature is enabled) or an empty body
//!   otherwise.
//! - Logs each error once at the HTTP boundary using `tracing::error`.
//!
//! ## Wire payload
//!
//! When the `serde_json` feature is enabled, the response body is
//! [`ErrorResponse`] with fields `{ status, message }`. `message` prefers the
//! explicit application message and falls back to the `AppErrorKind`’s display.
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
use tracing::error;

use crate::AppError;
#[cfg(feature = "serde_json")]
use crate::response::ErrorResponse;

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
        let status = self.http_status();

        // Log once at the boundary with stable fields.
        error!(
            status = status.as_u16(),
            kind = ?self.kind,
            msg = self.message.as_deref().unwrap_or(""),
            "AppError -> HTTP response"
        );

        #[cfg(feature = "serde_json")]
        {
            // Build the stable wire contract (includes `code`).
            let body: ErrorResponse = self.into();
            return body.into_response();
        }

        #[allow(unreachable_code)]
        (status, ()).into_response()
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

    #[cfg(feature = "serde_json")]
    #[tokio::test]
    async fn into_response_builds_json_error_with_code_and_message() {
        use axum::{body::to_bytes, response::IntoResponse};

        let app_err = AppError::unauthorized("missing token");
        let resp = app_err.into_response();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let bytes = to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("read body");
        // Deserialize via our own type to ensure wire contract matches
        let body: crate::response::ErrorResponse =
            serde_json::from_slice(&bytes).expect("json body");

        assert_eq!(body.status, 401);
        assert!(matches!(body.code, AppCode::Unauthorized));
        assert_eq!(body.message, "missing token");

        // Optional fields are absent by default
        #[cfg(feature = "serde_json")]
        {
            assert!(body.details.is_none());
        }
        assert!(body.retry.is_none());
        assert!(body.www_authenticate.is_none());
    }

    // --- IntoResponse without JSON body (serde_json disabled) ----------------

    #[cfg(not(feature = "serde_json"))]
    #[tokio::test]
    async fn into_response_without_json_has_empty_body() {
        use axum::{body::to_bytes, response::IntoResponse};

        let app_err = AppError::not_found("nope");
        let resp = app_err.into_response();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let bytes = to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("read body");
        assert_eq!(bytes.len(), 0, "body should be empty without serde_json");
    }
}
