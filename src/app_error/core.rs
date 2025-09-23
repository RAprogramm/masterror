use std::{
    backtrace::Backtrace,
    borrow::Cow,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult}
};

use tracing::error;

use super::metadata::{Field, Metadata};
use crate::{AppCode, AppErrorKind, RetryAdvice};

/// Controls whether the public message may be redacted before exposure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageEditPolicy {
    /// Message must be preserved as-is.
    Preserve,
    /// Message may be redacted or replaced at the transport boundary.
    Redact
}

impl Default for MessageEditPolicy {
    fn default() -> Self {
        Self::Preserve
    }
}

/// Rich application error preserving domain code, taxonomy and metadata.
#[derive(Debug)]
pub struct Error {
    /// Stable machine-readable error code.
    pub code:             AppCode,
    /// Semantic error category.
    pub kind:             AppErrorKind,
    /// Optional, public-friendly message.
    pub message:          Option<Cow<'static, str>>,
    /// Structured metadata for telemetry.
    pub metadata:         Metadata,
    /// Policy describing whether the message can be redacted.
    pub edit_policy:      MessageEditPolicy,
    /// Optional retry advice rendered as `Retry-After`.
    pub retry:            Option<RetryAdvice>,
    /// Optional authentication challenge for `WWW-Authenticate`.
    pub www_authenticate: Option<String>,
    source:               Option<Box<dyn StdError + Send + Sync + 'static>>,
    backtrace:            Option<Backtrace>
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.kind, f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn StdError + 'static))
    }
}

/// Conventional result alias for application code.
///
/// The alias defaults to [`Error`] but accepts a custom error type when the
/// context requires a different domain error.
///
/// # Examples
///
/// ```rust
/// use std::io::Error as IoError;
///
/// use masterror::AppResult;
///
/// fn app_logic() -> AppResult<u8> {
///     Ok(7)
/// }
///
/// fn io_logic() -> AppResult<(), IoError> {
///     Ok(())
/// }
///
/// assert_eq!(app_logic().unwrap(), 7);
/// assert!(io_logic().is_ok());
/// ```
pub type AppResult<T, E = Error> = Result<T, E>;

impl Error {
    /// Create a new [`Error`] with a kind and message.
    ///
    /// This is equivalent to [`Error::with`], provided for API symmetry and to
    /// keep doctests readable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid payload");
    /// assert!(err.message.is_some());
    /// ```
    #[must_use]
    pub fn new(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(kind, msg)
    }

    /// Create an error with the given kind and message.
    ///
    /// Prefer named helpers (e.g. [`Error::not_found`]) where it clarifies
    /// intent.
    #[must_use]
    pub fn with(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code: AppCode::from(kind),
            kind,
            message: Some(msg.into()),
            metadata: Metadata::new(),
            edit_policy: MessageEditPolicy::Preserve,
            retry: None,
            www_authenticate: None,
            source: None,
            backtrace: None
        }
    }

    /// Create a message-less error with the given kind.
    ///
    /// Useful when the kind alone conveys sufficient information to the client.
    #[must_use]
    pub fn bare(kind: AppErrorKind) -> Self {
        Self {
            code: AppCode::from(kind),
            kind,
            message: None,
            metadata: Metadata::new(),
            edit_policy: MessageEditPolicy::Preserve,
            retry: None,
            www_authenticate: None,
            source: None,
            backtrace: None
        }
    }

    /// Override the machine-readable [`AppCode`].
    #[must_use]
    pub fn with_code(mut self, code: AppCode) -> Self {
        self.code = code;
        self
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

    /// Attach additional metadata to the error.
    #[must_use]
    pub fn with_field(mut self, field: Field) -> Self {
        self.metadata.insert(field);
        self
    }

    /// Extend metadata from an iterator of fields.
    #[must_use]
    pub fn with_fields(mut self, fields: impl IntoIterator<Item = Field>) -> Self {
        self.metadata.extend(fields);
        self
    }

    /// Replace metadata entirely.
    #[must_use]
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Mark the message as redactable.
    #[must_use]
    pub fn redactable(mut self) -> Self {
        self.edit_policy = MessageEditPolicy::Redact;
        self
    }

    /// Attach a source error for diagnostics.
    #[must_use]
    pub fn with_source(mut self, source: impl StdError + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Attach a captured backtrace.
    #[must_use]
    pub fn with_backtrace(mut self, backtrace: Backtrace) -> Self {
        self.backtrace = Some(backtrace);
        self
    }

    /// Borrow the attached metadata.
    #[must_use]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Borrow the backtrace if present.
    #[must_use]
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }

    /// Borrow the source if present.
    #[must_use]
    pub fn source_ref(&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Human-readable message or the kind fallback.
    #[must_use]
    pub fn render_message(&self) -> Cow<'_, str> {
        match &self.message {
            Some(msg) => Cow::Borrowed(msg.as_ref()),
            None => Cow::Owned(self.kind.to_string())
        }
    }

    /// Log the error once at the boundary with stable fields.
    ///
    /// Emits a `tracing::error!` with `kind`, `code`, optional `message` and
    /// metadata length. No internals or sources are leaked.
    pub fn log(&self) {
        match &self.message {
            Some(m) => error!(
                kind = ?self.kind,
                code = %self.code,
                message = %m,
                metadata_len = self.metadata.len()
            ),
            None => error!(
                kind = ?self.kind,
                code = %self.code,
                metadata_len = self.metadata.len()
            )
        }
    }
}

/// Backwards-compatible export using the historical name.
pub use Error as AppError;
