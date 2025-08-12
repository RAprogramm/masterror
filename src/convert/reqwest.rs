//! Conversion from [`reqwest::Error`] into [`AppError`].
//!
//! Enabled with the `reqwest` feature flag.
//!
//! ## Mapping
//!
//! - [`reqwest::Error::is_timeout`] → `AppErrorKind::Timeout`
//! - [`reqwest::Error::is_connect`] or [`reqwest::Error::is_request`] →
//!   `AppErrorKind::Network`
//! - [`reqwest::Error::is_status`] → `AppErrorKind::ExternalApi` (with upstream
//!   status info)
//! - All other cases → `AppErrorKind::ExternalApi`
//!
//! The original error string is preserved in the `message` for observability.
//!
//! ## Rationale
//!
//! This mapping treats `reqwest` as a client to an external HTTP API.
//! Timeout and network/connectivity issues are separated from upstream
//! HTTP status failures so they can be handled differently by clients.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppError, AppErrorKind};
//! use reqwest::Error as ReqwestError;
//!
//! fn handle_http_error(e: ReqwestError) -> AppError {
//!     e.into()
//! }
//!
//! // Simulate: in reality, you'd get the error from a `reqwest` request.
//! let err = reqwest::get("http://invalid-domain").await.unwrap_err();
//! let app_err = handle_http_error(err);
//!
//! match app_err.kind {
//!     AppErrorKind::Network | AppErrorKind::Timeout | AppErrorKind::ExternalApi => {}
//!     _ => panic!("unexpected kind"),
//! }
//! ```

#[cfg(feature = "reqwest")]
use reqwest::Error as ReqwestError;

#[cfg(feature = "reqwest")]
use crate::AppError;

/// Map a [`reqwest::Error`] into an [`AppError`] according to its category.
///
/// - Timeout → `Timeout`
/// - Connect or request build error → `Network`
/// - Upstream returned HTTP error status → `ExternalApi`
/// - Fallback for other cases → `ExternalApi`
#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
impl From<ReqwestError> for AppError {
    fn from(err: ReqwestError) -> Self {
        if err.is_timeout() {
            AppError::timeout("Request timeout")
        } else if err.is_connect() || err.is_request() {
            AppError::network(format!("Network error: {err}"))
        } else if err.is_status() {
            AppError::external_api(format!("Upstream status error: {err}"))
        } else {
            AppError::external_api(format!("Upstream error: {err}"))
        }
    }
}
