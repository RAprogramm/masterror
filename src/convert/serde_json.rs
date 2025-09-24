//! Conversion from [`serde_json::Error`] into [`Error`].
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
//! use masterror::{AppErrorKind, Error};
//! use serde_json::Error as SjError;
//!
//! fn handle_json_error(e: SjError) -> Error {
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
use crate::{
    AppErrorKind,
    app_error::{Context, Error, field}
};

/// Map a [`serde_json::Error`] into an [`AppError`].
///
/// Errors are classified to `Serialization` or `Deserialization` using
/// [`serde_json::Error::classify`]. The original error string is preserved for
/// logs and optional JSON payloads.
#[cfg(feature = "serde_json")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde_json")))]
impl From<SjError> for Error {
    fn from(err: SjError) -> Self {
        build_context(&err).into_error(err)
    }
}

#[cfg(feature = "serde_json")]
fn build_context(err: &SjError) -> Context {
    let category = err.classify();
    let mut context = match category {
        Category::Io => Context::new(AppErrorKind::Serialization),
        Category::Syntax | Category::Data | Category::Eof => {
            Context::new(AppErrorKind::Deserialization)
        }
    }
    .with(field::str("serde_json.category", format!("{:?}", category)));

    let line = err.line();
    if line != 0 {
        context = context.with(field::u64("serde_json.line", u64::from(line)));
    }
    let column = err.column();
    if column != 0 {
        context = context.with(field::u64("serde_json.column", u64::from(column)));
    }

    context
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    use serde_json::json;

    use super::*;
    use crate::{AppErrorKind, FieldValue};

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
        let app: Error = err.into();
        assert!(matches!(app.kind, AppErrorKind::Serialization));
        assert_eq!(
            app.metadata().get("serde_json.category"),
            Some(&FieldValue::Str("Io".into()))
        );
    }

    #[test]
    fn syntax_maps_to_deserialization() {
        let err = serde_json::from_str::<serde_json::Value>("not-json").unwrap_err();
        let app: Error = err.into();
        assert!(matches!(app.kind, AppErrorKind::Deserialization));
        let metadata = app.metadata();
        assert_eq!(
            metadata.get("serde_json.category"),
            Some(&FieldValue::Str("Syntax".into()))
        );
    }
}
