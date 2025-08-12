//! Error conversions (`From<...> for AppError`).
//!
//! This module collects conservative mappings from external errors into
//! the crate’s [`AppError`]. Integrations are gated behind feature flags
//! to avoid pulling dependencies you do not use.
//!
//! ## Always-on mappings
//!
//! - [`std::io::Error`] → `AppErrorKind::Internal`
//! - [`String`] → `AppErrorKind::BadRequest`
//!
//! ## Feature-gated mappings
//!
//! These are defined in submodules and compiled only when the feature is
//! enabled:
//!
//! - `axum` + `multipart`: parsing multipart forms
//! - `config`: configuration loader errors
//! - `redis`: Redis client errors
//! - `reqwest`: HTTP client errors
//! - `serde_json`: JSON conversion helpers (if you attach JSON details
//!   yourself)
//! - `sqlx`: database driver errors
//! - `tokio`: timeouts from `tokio::time::error::Elapsed`
//! - `validator`: input DTO validation errors
//!
//! ## Design notes
//!
//! - Mappings prefer stable, public-facing categories (`AppErrorKind`).
//! - No panics, no unwraps; all failures become [`AppError`].
//! - Logging is not performed here. Log once at the transport boundary (e.g. in
//!   the `IntoResponse` implementation).
//!
//! ## Examples
//!
//! `std::io::Error` mapping:
//! ```rust
//! use std::io;
//!
//! use masterror::{AppError, AppErrorKind};
//!
//! fn open() -> Result<(), AppError> {
//!     let _ = io::Error::new(io::ErrorKind::Other, "boom");
//!     Err(io::Error::from(io::ErrorKind::Other).into())
//! }
//!
//! let err = open().unwrap_err();
//! assert!(matches!(err.kind, AppErrorKind::Internal));
//! ```
//!
//! `String` mapping (useful for ad-hoc validation without the `validator`
//! feature): ```rust
//! use masterror::{AppError, AppErrorKind};
//!
//! fn validate(x: i32) -> Result<(), AppError> {
//!     if x < 0 {
//!         return Err(String::from("must be non-negative").into());
//!     }
//!     Ok(())
//! }
//!
//! let err = validate(-1).unwrap_err();
//! assert!(matches!(err.kind, AppErrorKind::BadRequest));
//! ```

use std::io::Error as IoError;

use crate::AppError;

#[cfg(all(feature = "axum", feature = "multipart"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "axum", feature = "multipart"))))]
mod multipart;

#[cfg(feature = "config")]
#[cfg_attr(docsrs, doc(cfg(feature = "config")))]
mod config;

#[cfg(feature = "redis")]
#[cfg_attr(docsrs, doc(cfg(feature = "redis")))]
mod redis;

#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
mod reqwest;

#[cfg(feature = "serde_json")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde_json")))]
mod serde_json;

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
mod sqlx;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
mod tokio;

#[cfg(feature = "validator")]
#[cfg_attr(docsrs, doc(cfg(feature = "validator")))]
mod validator;

/// Map `std::io::Error` to an internal application error.
///
/// Rationale: I/O failures are infrastructure-level and should not leak
/// driver specifics to callers. The message is preserved for observability.
///
/// ```rust
/// use std::io::{self, ErrorKind};
///
/// use masterror::{AppError, AppErrorKind};
///
/// let io_err = io::Error::from(ErrorKind::Other);
/// let app_err: AppError = io_err.into();
/// assert!(matches!(app_err.kind, AppErrorKind::Internal));
/// ```
impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::internal(err.to_string())
    }
}

/// Map a plain `String` to a client error (`BadRequest`).
///
/// This is handy for lightweight validation without adding the `validator`
/// feature. Prefer structured validation where possible.
///
/// ```rust
/// use masterror::{AppError, AppErrorKind};
///
/// fn check(name: &str) -> Result<(), AppError> {
///     if name.is_empty() {
///         return Err(String::from("name must not be empty").into());
///     }
///     Ok(())
/// }
///
/// let err = check("").unwrap_err();
/// assert!(matches!(err.kind, AppErrorKind::BadRequest));
/// ```
impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::bad_request(value)
    }
}
