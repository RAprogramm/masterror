//! Conversion from [`serde_json::Error`] into [`AppError`].
//!
//! Enabled with the `serde_json` feature flag.
//!
//! ## Mapping
//!
//! All JSON (de)serialization errors are currently mapped to
//! `AppErrorKind::Internal` with the original error string preserved
//! in `message` for observability.
//!
//! If you want finer granularity, you can inspect the
//! [`serde_json::Error::classify`] result and map separately to `Serialization`
//! or `Deserialization` kinds.
//!
//! ## Rationale
//!
//! `serde_json::Error` is used both for encoding and decoding JSON. In many
//! service backends, a failure here means either an unexpected input format
//! or an internal bug in serialization code. To avoid leaking specifics to
//! clients, the default mapping treats it as an internal server error.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppError, AppErrorKind};
//! use serde_json::Error as SjError;
//!
//! fn handle_json_error(e: SjError) -> AppError {
//!     e.into()
//! }
//!
//! let err = serde_json::from_str::<serde_json::Value>("not-json").unwrap_err();
//! let app_err = handle_json_error(err);
//! assert!(matches!(app_err.kind, AppErrorKind::Internal));
//! ```

#[cfg(feature = "serde_json")]
use serde_json::Error as SjError;

#[cfg(feature = "serde_json")]
use crate::AppError;

/// Map a [`serde_json::Error`] into an [`AppError`].
///
/// By default, all JSON errors are considered internal failures.
/// The original error string is preserved for logs and optional JSON payloads.
#[cfg(feature = "serde_json")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde_json")))]
impl From<SjError> for AppError {
    fn from(err: SjError) -> Self {
        // If needed, you could inspect err.classify() to separate serialization
        // from deserialization, but here we keep it simple and stable.
        AppError::internal(format!("Serialization error: {err}"))
    }
}
