use std::borrow::Cow;

use tracing::error;

use crate::{Error, RetryAdvice, code::AppCode, kind::AppErrorKind};

/// Thin error wrapper: kind + optional message.
///
/// `Display` prints only the `kind`. The optional `message` is intended for
/// logs and (when appropriate) public JSON payloads. Keep messages concise and
/// free of sensitive data.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct AppError {
    /// Semantic category of the error.
    pub kind:             AppErrorKind,
    /// Optional, public-friendly message.
    pub message:          Option<Cow<'static, str>>,
    /// Optional retry advice rendered as `Retry-After`.
    pub retry:            Option<RetryAdvice>,
    /// Optional authentication challenge for `WWW-Authenticate`.
    pub www_authenticate: Option<String>
}

/// Conventional result alias for application code.
///
/// The alias defaults to [`AppError`] but accepts a custom error type when the
/// context requires a different domain error.
///
/// # Examples
///
/// ```rust
/// use std::io::Error;
///
/// use masterror::AppResult;
///
/// fn app_logic() -> AppResult<u8> {
///     Ok(7)
/// }
///
/// fn io_logic() -> AppResult<(), Error> {
///     Ok(())
/// }
///
/// assert_eq!(app_logic().unwrap(), 7);
/// assert!(io_logic().is_ok());
/// ```
pub type AppResult<T, E = AppError> = Result<T, E>;

impl AppError {
    /// Create a new [`AppError`] with a kind and message.
    ///
    /// This is equivalent to [`AppError::with`], provided for API symmetry and
    /// to keep doctests readable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid payload");
    /// assert!(err.message.is_some());
    /// ```
    pub fn new(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(kind, msg)
    }

    /// Create an error with the given kind and message.
    ///
    /// Prefer named helpers (e.g. [`AppError::not_found`]) where it clarifies
    /// intent.
    pub fn with(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            message: Some(msg.into()),
            retry: None,
            www_authenticate: None
        }
    }

    /// Create a message-less error with the given kind.
    ///
    /// Useful when the kind alone conveys sufficient information to the client.
    pub fn bare(kind: AppErrorKind) -> Self {
        Self {
            kind,
            message: None,
            retry: None,
            www_authenticate: None
        }
    }

    /// Attach retry advice to the error.
    ///
    /// When mapped to HTTP, this becomes the `Retry-After` header.
    #[must_use]
    pub fn with_retry_after_secs(mut self, secs: u64) -> Self {
        self.retry = Some(RetryAdvice {
            after_seconds: secs
        });
        self
    }

    /// Attach a `WWW-Authenticate` challenge string.
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self
    }

    /// Log the error once at the boundary with stable fields.
    ///
    /// Emits a `tracing::error!` with `kind`, `code` and optional `message`.
    /// No internals or sources are leaked.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::internal("boom");
    /// // In production, call this at the boundary (e.g. HTTP mapping).
    /// err.log();
    /// ```
    pub fn log(&self) {
        let code = AppCode::from(self.kind);
        match &self.message {
            Some(m) => error!(kind = ?self.kind, code = %code, message = %m),
            None => error!(kind = ?self.kind, code = %code)
        }
    }
}
