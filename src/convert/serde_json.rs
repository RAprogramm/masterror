//! Conversion from [`serde_json::Error`] into [`AppError`].
//!
//! Enabled with the `serde_json` feature flag.
//!
//! ## Mapping
//!
//! Errors are classified using [`serde_json::Error::classify`]. I/O failures
//! are mapped to [`AppErrorKind::Serialization`]; syntax, data and EOF errors
//! map to [`AppErrorKind::Deserialization`]. The original error string is
//! preserved in `message` for observability.
//!
//! ## Rationale
//!
//! `serde_json::Error` is used both for encoding and decoding JSON. Classifying
//! errors lets callers distinguish between failures while keeping a stable
//! mapping in the API surface.
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
//! assert!(matches!(app_err.kind, AppErrorKind::Deserialization));
//! ```

#[cfg(feature = "serde_json")]
use serde_json::{Error as SjError, error::Category};

#[cfg(feature = "serde_json")]
use crate::AppError;

/// Map a [`serde_json::Error`] into an [`AppError`].
///
/// Errors are classified to `Serialization` or `Deserialization` using
/// [`serde_json::Error::classify`]. The original error string is preserved for
/// logs and optional JSON payloads.
#[cfg(feature = "serde_json")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde_json")))]
impl From<SjError> for AppError {
    fn from(err: SjError) -> Self {
        match err.classify() {
            Category::Io => AppError::serialization(err.to_string()),
            Category::Syntax | Category::Data | Category::Eof => {
                AppError::deserialization(err.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    use serde_json::json;

    use super::*;
    use crate::kind::AppErrorKind;

    #[test]
    fn io_maps_to_serialization() {
        struct FailWriter;

        impl Write for FailWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("fail"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let err = serde_json::to_writer(FailWriter, &json!({"k": "v"})).unwrap_err();
        let app: AppError = err.into();
        assert!(matches!(app.kind, AppErrorKind::Serialization));
    }

    #[test]
    fn syntax_maps_to_deserialization() {
        let err = serde_json::from_str::<serde_json::Value>("not-json").unwrap_err();
        let app: AppError = err.into();
        assert!(matches!(app.kind, AppErrorKind::Deserialization));
    }
}
