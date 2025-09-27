//! Error conversions (`From<...> for AppError`) and transport-specific
//! adapters.
//!
//! This module collects **conservative mappings** from external errors into
//! the crate’s [`AppError`]. It also conditionally enables transport adapters
//! (Axum/Actix) to turn [`AppError`] into HTTP responses.  
//!
//! ## Always-on mappings
//!
//! - [`std::io::Error`] → `AppErrorKind::Internal`   Infrastructure-level
//!   failure; message preserved for logs only.
//! - [`String`] → `AppErrorKind::BadRequest`   Lightweight validation helper
//!   when you don’t pull in `validator`.
//!
//! ## Feature-gated mappings
//!
//! Each of these is compiled only when the feature is enabled. They live in
//! submodules under `convert/`:
//!
//! - `axum` + `multipart`: Axum multipart parsing errors
//! - `actix`: Actix `ResponseError` integration (not a mapping, but transport)
//! - `config`: configuration loader errors
//! - `redis`: Redis client errors
//! - `reqwest`: HTTP client errors
//! - `serde_json`: JSON conversion helpers (if you attach JSON details
//!   manually)
//! - `sqlx`: database driver errors
//! - `tokio`: timeouts from `tokio::time::error::Elapsed`
//! - `teloxide`: Telegram request errors
//! - `validator`: input DTO validation errors
//!
//! ## Design notes
//!
//! - Mappings prefer **stable, public-facing categories** (`AppErrorKind`).
//! - **No panics, no unwraps**: all failures become [`AppError`].
//! - **Logging is not performed here**. The only place errors are logged is at
//!   the HTTP boundary (e.g. in `IntoResponse` or `ResponseError` impls).
//! - Transport adapters (`axum`, `actix`) are technically not “conversions”,
//!   but are colocated here for discoverability. They never leak internal error
//!   sources; only safe wire payloads are exposed.
//!
//! ## Examples
//!
//! `std::io::Error` mapping:
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! # {
//! use std::io::{self, ErrorKind};
//!
//! use masterror::{AppError, AppErrorKind, AppResult};
//!
//! fn open() -> AppResult<()> {
//!     let _ = io::Error::new(ErrorKind::Other, "boom");
//!     Err(io::Error::from(ErrorKind::Other).into())
//! }
//!
//! let err = open().unwrap_err();
//! assert!(matches!(err.kind, AppErrorKind::Internal));
//! # }
//! ```
//!
//! `String` mapping (useful for ad-hoc validation without the `validator`
//! feature):
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind, AppResult};
//!
//! fn validate(x: i32) -> AppResult<()> {
//!     if x < 0 {
//!         return Err(String::from("must be non-negative").into());
//!     }
//!     Ok(())
//! }
//!
//! let err = validate(-1).unwrap_err();
//! assert!(matches!(err.kind, AppErrorKind::BadRequest));
//! ```

use alloc::string::String;
#[cfg(feature = "std")]
use std::io::Error as IoError;

use crate::AppError;

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
mod axum;

#[cfg(all(feature = "axum", feature = "multipart"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "axum", feature = "multipart"))))]
mod multipart;

#[cfg(feature = "actix")]
#[cfg_attr(docsrs, doc(cfg(feature = "actix")))]
mod actix;

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

#[cfg(feature = "teloxide")]
#[cfg_attr(docsrs, doc(cfg(feature = "teloxide")))]
mod teloxide;

#[cfg(feature = "telegram-webapp-sdk")]
#[cfg_attr(docsrs, doc(cfg(feature = "telegram-webapp-sdk")))]
mod telegram_webapp_sdk;

#[cfg(feature = "tonic")]
#[cfg_attr(docsrs, doc(cfg(feature = "tonic")))]
mod tonic;

#[cfg(feature = "tonic")]
pub use self::tonic::StatusConversionError;

/// Map `std::io::Error` to an internal application error.
///
/// Rationale: I/O failures are infrastructure-level and should not leak
/// driver-specific details to clients. The message is preserved for
/// observability, but the public-facing kind is always `Internal`.
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
#[cfg(feature = "std")]
impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::internal(err.to_string())
    }
}

/// Map a plain `String` to a client error (`BadRequest`).
///
/// Handy for quick validation paths without the `validator` feature.
/// Prefer structured validation for complex DTOs, but this covers simple cases.
///
/// ```rust
/// use masterror::{AppError, AppErrorKind, AppResult};
///
/// fn check(name: &str) -> AppResult<()> {
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

#[cfg(test)]
mod tests {
    use crate::{AppError, AppErrorKind};

    // --- std::io::Error -> AppError -----------------------------------------

    #[test]
    fn io_error_maps_to_internal_and_preserves_message() {
        use std::io::Error;

        let src = Error::other("disk said nope");
        let app: AppError = src.into();

        // kind must be Internal
        assert!(matches!(app.kind, AppErrorKind::Internal));

        // message should be preserved for logs/public payload
        assert_eq!(app.message.as_deref(), Some("disk said nope"));
    }

    // --- String -> AppError --------------------------------------------------

    #[test]
    fn string_maps_to_bad_request_and_preserves_text() {
        let app: AppError = String::from("name must not be empty").into();

        assert!(matches!(app.kind, AppErrorKind::BadRequest));
        assert_eq!(app.message.as_deref(), Some("name must not be empty"));
    }
}
