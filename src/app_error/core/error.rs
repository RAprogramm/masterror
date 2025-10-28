// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::{borrow::Cow, boxed::Box, string::String, sync::Arc};
use core::{
    error::Error as CoreError,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool
};
#[cfg(feature = "backtrace")]
use std::{backtrace::Backtrace, sync::OnceLock};

#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;

#[cfg(not(feature = "backtrace"))]
use super::types::CapturedBacktrace;
use super::types::MessageEditPolicy;
use crate::{AppCode, AppErrorKind, RetryAdvice, app_error::metadata::Metadata};

/// Internal representation of error state.
///
/// This structure holds all error data including code, kind, message,
/// metadata, and optional diagnostic information. It is boxed within
/// [`Error`] to minimize stack size.
#[derive(Debug)]
#[doc(hidden)]
pub struct ErrorInner {
    /// Stable machine-readable error code.
    pub code:                   AppCode,
    /// Semantic error category.
    pub kind:                   AppErrorKind,
    /// Optional, public-friendly message.
    pub message:                Option<Cow<'static, str>>,
    /// Structured metadata for telemetry.
    pub metadata:               Metadata,
    /// Policy describing whether the message can be redacted.
    pub edit_policy:            MessageEditPolicy,
    /// Optional retry advice rendered as `Retry-After`.
    pub retry:                  Option<RetryAdvice>,
    /// Optional authentication challenge for `WWW-Authenticate`.
    pub www_authenticate:       Option<String>,
    /// Optional structured details exposed to clients.
    #[cfg(feature = "serde_json")]
    pub details:                Option<JsonValue>,
    /// Optional textual details when JSON is unavailable.
    #[cfg(not(feature = "serde_json"))]
    pub details:                Option<String>,
    pub source:                 Option<Arc<dyn CoreError + Send + Sync + 'static>>,
    #[cfg(feature = "backtrace")]
    pub backtrace:              Option<Arc<Backtrace>>,
    #[cfg(feature = "backtrace")]
    pub captured_backtrace:     OnceLock<Option<Arc<Backtrace>>>,
    pub(super) telemetry_dirty: AtomicBool,
    #[cfg(feature = "tracing")]
    pub(super) tracing_dirty:   AtomicBool
}

/// Rich application error preserving domain code, taxonomy and metadata.
///
/// This is the main error type for application-level errors. It provides
/// structured error information, telemetry integration, and diagnostic
/// capabilities.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppError, AppErrorKind};
///
/// let err = AppError::new(AppErrorKind::BadRequest, "invalid payload");
/// assert_eq!(err.kind, AppErrorKind::BadRequest);
/// ```
#[derive(Debug)]
pub struct Error {
    pub(super) inner: Box<ErrorInner>
}

impl Deref for Error {
    type Target = ErrorInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use super::display::DisplayMode;

        match DisplayMode::current() {
            DisplayMode::Prod => self.fmt_prod(f),
            DisplayMode::Local => self.fmt_local(f),
            DisplayMode::Staging => self.fmt_staging(f)
        }
    }
}

impl CoreError for Error {
    fn source(&self) -> Option<&(dyn CoreError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn CoreError + 'static))
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
    /// Creates a new error with raw initialization.
    ///
    /// This is an internal constructor that initializes all fields to their
    /// default values. Public constructors should use this and call
    /// `emit_telemetry()`.
    ///
    /// # Arguments
    ///
    /// * `kind` - The error category
    /// * `message` - Optional human-readable message
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let err = Error::new_raw(AppErrorKind::Internal, Some(Cow::Borrowed("test")));
    /// ```
    pub(crate) fn new_raw(kind: AppErrorKind, message: Option<Cow<'static, str>>) -> Self {
        Self {
            inner: Box::new(ErrorInner {
                code: AppCode::from(kind),
                kind,
                message,
                metadata: Metadata::new(),
                edit_policy: MessageEditPolicy::Preserve,
                retry: None,
                www_authenticate: None,
                details: None,
                source: None,
                #[cfg(feature = "backtrace")]
                backtrace: None,
                #[cfg(feature = "backtrace")]
                captured_backtrace: OnceLock::new(),
                telemetry_dirty: AtomicBool::new(true),
                #[cfg(feature = "tracing")]
                tracing_dirty: AtomicBool::new(true)
            })
        }
    }

    /// Sets the backtrace slot, replacing any existing backtrace.
    ///
    /// This is an internal method used by builder methods to attach
    /// backtraces.
    ///
    /// # Arguments
    ///
    /// * `backtrace` - The backtrace to attach
    #[cfg(feature = "backtrace")]
    pub(super) fn set_backtrace_slot(&mut self, backtrace: Arc<Backtrace>) {
        self.backtrace = Some(backtrace);
        self.captured_backtrace = OnceLock::new();
    }

    #[cfg(not(feature = "backtrace"))]
    pub(super) fn set_backtrace_slot(&mut self, _backtrace: CapturedBacktrace) {}
}

/// Backwards-compatible export using the historical name.
pub use Error as AppError;
