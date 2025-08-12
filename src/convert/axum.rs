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
    response::{IntoResponse, Response},
    Json
};
use tracing::error;

#[cfg(feature = "serde_json")]
use crate::response::ErrorResponse;
use crate::AppError;

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

        // JSON payload when `serde_json` is enabled.
        #[cfg(feature = "serde_json")]
        {
            let body = ErrorResponse {
                status:  status.as_u16(),
                // Prefer specific message if present, otherwise kind text.
                message: self
                    .message
                    .clone()
                    .unwrap_or_else(|| self.kind.to_string()),
                // You can wire structured details later if you add them to AppError.
                details: None
            };
            return (status, Json(body)).into_response();
        }

        // Fallback: status-only, empty body (content-type left to Axum).
        #[allow(unreachable_code)]
        (status, ()).into_response()
    }
}
