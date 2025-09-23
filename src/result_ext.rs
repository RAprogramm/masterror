use std::error::Error as StdError;

use crate::app_error::{Context, Error};

/// Extension trait for enriching `Result` errors with [`Context`].
///
/// The [`ctx`](ResultExt::ctx) method converts the error side of a `Result`
/// into [`Error`] while attaching metadata, category and edit policy captured
/// by [`Context`].
///
/// # Examples
///
/// ```rust
/// use std::io::{Error as IoError, ErrorKind};
///
/// use masterror::{AppErrorKind, Context, ResultExt, field};
///
/// fn validate() -> Result<(), IoError> {
///     Err(IoError::from(ErrorKind::Other))
/// }
///
/// let err = validate()
///     .ctx(|| Context::new(AppErrorKind::Validation).with(field::str("phase", "validate")))
///     .unwrap_err();
///
/// assert_eq!(err.kind, AppErrorKind::Validation);
/// assert!(err.metadata().get("phase").is_some());
/// ```
pub trait ResultExt<T, E> {
    /// Convert an error into [`Error`] using [`Context`] supplied by `build`.
    #[allow(clippy::result_large_err)]
    fn ctx(self, build: impl FnOnce() -> Context) -> Result<T, Error>
    where
        E: StdError + Send + Sync + 'static;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn ctx(self, build: impl FnOnce() -> Context) -> Result<T, Error>
    where
        E: StdError + Send + Sync + 'static
    {
        self.map_err(|err| build().into_error(err))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        error::Error as StdError,
        fmt::{Display, Formatter, Result as FmtResult},
        sync::Arc
    };

    use super::ResultExt;
    use crate::{
        AppCode, AppErrorKind,
        app_error::{Context, FieldValue, MessageEditPolicy},
        field
    };

    #[derive(Debug)]
    struct DummyError;

    impl Display for DummyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("dummy")
        }
    }

    impl StdError for DummyError {}

    #[test]
    fn ctx_preserves_ok() {
        let res: Result<u8, DummyError> = Ok(5);
        let value = res
            .ctx(|| Context::new(AppErrorKind::Internal))
            .expect("ok");
        assert_eq!(value, 5);
    }

    #[test]
    fn ctx_wraps_err_with_context() {
        let result: Result<(), DummyError> = Err(DummyError);
        let err = result
            .ctx(|| {
                Context::new(AppErrorKind::Service)
                    .with(field::str("operation", "sync"))
                    .redact(true)
                    .track_caller()
            })
            .expect_err("err");

        assert_eq!(err.kind, AppErrorKind::Service);
        assert_eq!(err.code, AppCode::Service);
        assert!(matches!(err.edit_policy, MessageEditPolicy::Redact));

        let metadata = err.metadata();
        assert_eq!(
            metadata.get("operation"),
            Some(&FieldValue::Str(Cow::Borrowed("sync")))
        );
        let caller_file = metadata.get("caller.file").expect("caller file field");
        assert_eq!(caller_file, &FieldValue::Str(Cow::Borrowed(file!())));
        assert!(metadata.get("caller.line").is_some());
        assert!(metadata.get("caller.column").is_some());
    }

    #[derive(Debug, Clone)]
    struct SharedError(Arc<InnerError>);

    #[derive(Debug)]
    struct InnerError;

    impl Display for InnerError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("inner")
        }
    }

    impl StdError for InnerError {}

    impl Display for SharedError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            Display::fmt(&*self.0, f)
        }
    }

    impl StdError for SharedError {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            Some(&*self.0)
        }
    }

    #[test]
    fn ctx_preserves_source_without_extra_arc_clone() {
        let inner = Arc::new(InnerError);
        let shared = SharedError(inner.clone());
        let err = Result::<(), SharedError>::Err(shared.clone())
            .ctx(|| Context::new(AppErrorKind::Internal))
            .expect_err("err");

        drop(shared);
        assert_eq!(Arc::strong_count(&inner), 2);

        let stored = err
            .source_ref()
            .and_then(|src| src.downcast_ref::<SharedError>())
            .expect("shared source");
        assert!(Arc::ptr_eq(&stored.0, &inner));
    }
}
