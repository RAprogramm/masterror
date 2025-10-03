// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::{borrow::Cow, boxed::Box, string::String, sync::Arc};
use core::{
    error::Error as CoreError,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering}
};
#[cfg(feature = "backtrace")]
use std::{
    backtrace::Backtrace,
    env,
    sync::{
        OnceLock,
        atomic::{AtomicU8, Ordering as AtomicOrdering}
    }
};

#[cfg(feature = "serde_json")]
use serde::Serialize;
#[cfg(feature = "serde_json")]
use serde_json::{Value as JsonValue, to_value};
#[cfg(feature = "tracing")]
use tracing::{Level, event};

use super::metadata::{Field, FieldRedaction, Metadata};
use crate::{AppCode, AppErrorKind, RetryAdvice};

/// Attachments accepted by [`Error::with_context`].
#[derive(Debug)]
#[doc(hidden)]
pub enum ContextAttachment {
    Owned(Box<dyn CoreError + Send + Sync + 'static>),
    Shared(Arc<dyn CoreError + Send + Sync + 'static>)
}

impl<E> From<E> for ContextAttachment
where
    E: CoreError + Send + Sync + 'static
{
    fn from(source: E) -> Self {
        Self::Owned(Box::new(source))
    }
}

#[cfg(feature = "std")]
pub type CapturedBacktrace = std::backtrace::Backtrace;

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
#[derive(Debug)]
pub enum CapturedBacktrace {}

/// Controls whether the public message may be redacted before exposure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MessageEditPolicy {
    /// Message must be preserved as-is.
    #[default]
    Preserve,
    /// Message may be redacted or replaced at the transport boundary.
    Redact
}

#[derive(Debug)]
#[doc(hidden)]
pub struct ErrorInner {
    /// Stable machine-readable error code.
    pub code:               AppCode,
    /// Semantic error category.
    pub kind:               AppErrorKind,
    /// Optional, public-friendly message.
    pub message:            Option<Cow<'static, str>>,
    /// Structured metadata for telemetry.
    pub metadata:           Metadata,
    /// Policy describing whether the message can be redacted.
    pub edit_policy:        MessageEditPolicy,
    /// Optional retry advice rendered as `Retry-After`.
    pub retry:              Option<RetryAdvice>,
    /// Optional authentication challenge for `WWW-Authenticate`.
    pub www_authenticate:   Option<String>,
    /// Optional structured details exposed to clients.
    #[cfg(feature = "serde_json")]
    pub details:            Option<JsonValue>,
    /// Optional textual details when JSON is unavailable.
    #[cfg(not(feature = "serde_json"))]
    pub details:            Option<String>,
    pub source:             Option<Arc<dyn CoreError + Send + Sync + 'static>>,
    #[cfg(feature = "backtrace")]
    pub backtrace:          Option<Backtrace>,
    #[cfg(feature = "backtrace")]
    pub captured_backtrace: OnceLock<Option<Backtrace>>,
    telemetry_dirty:        AtomicBool,
    #[cfg(feature = "tracing")]
    tracing_dirty:          AtomicBool
}

#[cfg(feature = "backtrace")]
const BACKTRACE_STATE_UNSET: u8 = 0;
#[cfg(feature = "backtrace")]
const BACKTRACE_STATE_ENABLED: u8 = 1;
#[cfg(feature = "backtrace")]
const BACKTRACE_STATE_DISABLED: u8 = 2;

#[cfg(feature = "backtrace")]
static BACKTRACE_STATE: AtomicU8 = AtomicU8::new(BACKTRACE_STATE_UNSET);

#[cfg(feature = "backtrace")]
fn capture_backtrace_snapshot() -> Option<Backtrace> {
    if should_capture_backtrace() {
        Some(Backtrace::capture())
    } else {
        None
    }
}

#[cfg(feature = "backtrace")]
fn should_capture_backtrace() -> bool {
    match BACKTRACE_STATE.load(AtomicOrdering::Acquire) {
        BACKTRACE_STATE_ENABLED => true,
        BACKTRACE_STATE_DISABLED => false,
        _ => {
            let enabled = detect_backtrace_preference();
            BACKTRACE_STATE.store(
                if enabled {
                    BACKTRACE_STATE_ENABLED
                } else {
                    BACKTRACE_STATE_DISABLED
                },
                AtomicOrdering::Release
            );
            enabled
        }
    }
}

#[cfg(feature = "backtrace")]
fn detect_backtrace_preference() -> bool {
    #[cfg(all(test, feature = "backtrace"))]
    if let Some(value) = test_backtrace_override::get() {
        return value;
    }

    match env::var_os("RUST_BACKTRACE") {
        None => false,
        Some(value) => {
            let value = value.to_string_lossy();
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return false;
            }
            let lowered = trimmed.to_ascii_lowercase();
            !(matches!(lowered.as_str(), "0" | "off" | "false"))
        }
    }
}

#[cfg(all(test, feature = "backtrace"))]
pub(crate) fn reset_backtrace_preference() {
    BACKTRACE_STATE.store(BACKTRACE_STATE_UNSET, AtomicOrdering::Release);
    test_backtrace_override::set(None);
}

#[cfg(all(test, feature = "backtrace"))]
pub(crate) fn set_backtrace_preference_override(value: Option<bool>) {
    test_backtrace_override::set(value);
}

#[cfg(all(test, feature = "backtrace"))]
mod test_backtrace_override {
    use std::sync::atomic::{AtomicI8, Ordering};

    const OVERRIDE_UNSET: i8 = -1;
    const OVERRIDE_DISABLED: i8 = 0;
    const OVERRIDE_ENABLED: i8 = 1;

    static OVERRIDE_STATE: AtomicI8 = AtomicI8::new(OVERRIDE_UNSET);

    pub(super) fn set(value: Option<bool>) {
        let state = match value {
            Some(true) => OVERRIDE_ENABLED,
            Some(false) => OVERRIDE_DISABLED,
            None => OVERRIDE_UNSET
        };
        OVERRIDE_STATE.store(state, Ordering::Release);
    }

    pub(super) fn get() -> Option<bool> {
        match OVERRIDE_STATE.load(Ordering::Acquire) {
            OVERRIDE_ENABLED => Some(true),
            OVERRIDE_DISABLED => Some(false),
            _ => None
        }
    }
}

/// Rich application error preserving domain code, taxonomy and metadata.
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>
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
        Display::fmt(&self.kind, f)
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

    fn mark_dirty(&self) {
        self.telemetry_dirty.store(true, Ordering::Release);
        #[cfg(feature = "tracing")]
        self.mark_tracing_dirty();
    }

    fn take_dirty(&self) -> bool {
        self.telemetry_dirty.swap(false, Ordering::AcqRel)
    }

    #[cfg(feature = "tracing")]
    fn mark_tracing_dirty(&self) {
        self.tracing_dirty.store(true, Ordering::Release);
    }

    #[cfg(feature = "tracing")]
    fn take_tracing_dirty(&self) -> bool {
        self.tracing_dirty.swap(false, Ordering::AcqRel)
    }

    #[cfg(feature = "backtrace")]
    fn capture_backtrace(&self) -> Option<&CapturedBacktrace> {
        if let Some(backtrace) = self.backtrace.as_ref() {
            return Some(backtrace);
        }

        self.captured_backtrace
            .get_or_init(capture_backtrace_snapshot)
            .as_ref()
    }

    #[cfg(not(feature = "backtrace"))]
    fn capture_backtrace(&self) -> Option<&CapturedBacktrace> {
        None
    }

    #[cfg(feature = "backtrace")]
    fn set_backtrace_slot(&mut self, backtrace: CapturedBacktrace) {
        self.backtrace = Some(backtrace);
        self.captured_backtrace = OnceLock::new();
    }

    #[cfg(not(feature = "backtrace"))]
    fn set_backtrace_slot(&mut self, _backtrace: CapturedBacktrace) {}

    pub(crate) fn emit_telemetry(&self) {
        if self.take_dirty() {
            #[cfg(feature = "backtrace")]
            let _ = self.capture_backtrace();

            #[cfg(feature = "metrics")]
            {
                let code_label = self.code.as_str().to_owned();
                let category_label = kind_label(self.kind).to_owned();
                metrics::counter!(
                    "error_total",
                    "code" => code_label,
                    "category" => category_label
                )
                .increment(1);
            }
        }

        #[cfg(feature = "tracing")]
        self.flush_tracing();
    }

    #[cfg(feature = "tracing")]
    fn flush_tracing(&self) {
        if !self.take_tracing_dirty() {
            return;
        }

        if !tracing::event_enabled!(target: "masterror::error", Level::ERROR) {
            self.mark_tracing_dirty();
            return;
        }

        let message = self.message.as_deref();
        let retry_seconds = self.retry.map(|value| value.after_seconds);
        let trace_id = log_mdc::get("trace_id", |value| value.map(str::to_owned));
        event!(
            target: "masterror::error",
            Level::ERROR,
            code = self.code.as_str(),
            category = kind_label(self.kind),
            message = message,
            retry_seconds,
            redactable = matches!(self.edit_policy, MessageEditPolicy::Redact),
            metadata_len = self.metadata.len() as u64,
            www_authenticate = self.www_authenticate.as_deref(),
            trace_id = trace_id.as_deref(),
            "app error constructed"
        );
    }

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
        let err = Self::new_raw(kind, Some(msg.into()));
        err.emit_telemetry();
        err
    }

    /// Create a message-less error with the given kind.
    ///
    /// Useful when the kind alone conveys sufficient information to the client.
    #[must_use]
    pub fn bare(kind: AppErrorKind) -> Self {
        let err = Self::new_raw(kind, None);
        err.emit_telemetry();
        err
    }

    /// Override the machine-readable [`AppCode`].
    #[must_use]
    pub fn with_code(mut self, code: AppCode) -> Self {
        self.code = code;
        self.mark_dirty();
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
        self.mark_dirty();
        self
    }

    /// Attach a `WWW-Authenticate` challenge string.
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self.mark_dirty();
        self
    }

    /// Attach additional metadata to the error.
    #[must_use]
    pub fn with_field(mut self, field: Field) -> Self {
        self.metadata.insert(field);
        self.mark_dirty();
        self
    }

    /// Extend metadata from an iterator of fields.
    #[must_use]
    pub fn with_fields(mut self, fields: impl IntoIterator<Item = Field>) -> Self {
        self.metadata.extend(fields);
        self.mark_dirty();
        self
    }

    /// Override the redaction policy for a stored metadata field.
    #[must_use]
    pub fn redact_field(mut self, name: &'static str, redaction: FieldRedaction) -> Self {
        self.metadata.set_redaction(name, redaction);
        self.mark_dirty();
        self
    }

    /// Replace metadata entirely.
    #[must_use]
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self.mark_dirty();
        self
    }

    /// Mark the message as redactable.
    #[must_use]
    pub fn redactable(mut self) -> Self {
        self.edit_policy = MessageEditPolicy::Redact;
        self.mark_dirty();
        self
    }

    /// Attach upstream diagnostics using [`with_source`](Self::with_source) or
    /// an existing [`Arc`].
    ///
    /// This is the preferred alias for capturing upstream errors. It accepts
    /// either an owned error implementing [`core::error::Error`] or a
    /// shared [`Arc`] produced by other APIs, reusing the allocation when
    /// possible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::AppError;
    ///
    /// let err = AppError::service("downstream degraded")
    ///     .with_context(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
    /// assert!(err.source_ref().is_some());
    /// # }
    /// ```
    #[must_use]
    pub fn with_context(self, context: impl Into<ContextAttachment>) -> Self {
        match context.into() {
            ContextAttachment::Owned(source) => {
                match source.downcast::<Arc<dyn CoreError + Send + Sync + 'static>>() {
                    Ok(shared) => self.with_source_arc(*shared),
                    Err(source) => self.with_source_arc(Arc::from(source))
                }
            }
            ContextAttachment::Shared(source) => self.with_source_arc(source)
        }
    }

    /// Attach a source error for diagnostics.
    ///
    /// Prefer [`with_context`](Self::with_context) when capturing upstream
    /// diagnostics without additional `Arc` allocations.
    #[must_use]
    pub fn with_source(mut self, source: impl CoreError + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self.mark_dirty();
        self
    }

    /// Attach a shared source error without cloning the underlying `Arc`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::sync::Arc;
    ///
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let source = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
    /// let err = AppError::internal("boom").with_source_arc(source.clone());
    /// assert!(err.source_ref().is_some());
    /// assert_eq!(Arc::strong_count(&source), 2);
    /// # }
    /// ```
    #[must_use]
    pub fn with_source_arc(mut self, source: Arc<dyn CoreError + Send + Sync + 'static>) -> Self {
        self.source = Some(source);
        self.mark_dirty();
        self
    }

    /// Attach a captured backtrace.
    #[must_use]
    pub fn with_backtrace(mut self, backtrace: CapturedBacktrace) -> Self {
        self.set_backtrace_slot(backtrace);
        self.mark_dirty();
        self
    }

    /// Attach structured JSON details for the client payload.
    ///
    /// The details are omitted from responses when the error has been marked as
    /// [`redactable`](Self::redactable).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{AppError, AppErrorKind};
    /// use serde_json::json;
    ///
    /// let err = AppError::new(AppErrorKind::Validation, "invalid input")
    ///     .with_details_json(json!({"field": "email"}));
    /// assert!(err.details.is_some());
    /// # }
    /// ```
    #[must_use]
    #[cfg(feature = "serde_json")]
    pub fn with_details_json(mut self, details: JsonValue) -> Self {
        self.details = Some(details);
        self.mark_dirty();
        self
    }

    /// Serialize and attach structured details.
    ///
    /// Returns [`AppError`] with [`AppErrorKind::BadRequest`] if serialization
    /// fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{AppError, AppErrorKind};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Extra {
    ///     reason: &'static str
    /// }
    ///
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid")
    ///     .with_details(Extra {
    ///         reason: "missing"
    ///     })
    ///     .expect("details should serialize");
    /// assert!(err.details.is_some());
    /// # }
    /// ```
    #[cfg(feature = "serde_json")]
    #[allow(clippy::result_large_err)]
    pub fn with_details<T>(self, payload: T) -> crate::AppResult<Self>
    where
        T: Serialize
    {
        let details = to_value(payload).map_err(|err| Self::bad_request(err.to_string()))?;
        Ok(self.with_details_json(details))
    }

    /// Attach plain-text details for client payloads.
    ///
    /// The text is omitted from responses when the error is
    /// [`redactable`](Self::redactable).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(not(feature = "serde_json"))]
    /// # {
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let err = AppError::new(AppErrorKind::Internal, "boom").with_details_text("retry later");
    /// assert!(err.details.is_some());
    /// # }
    /// ```
    #[must_use]
    #[cfg(not(feature = "serde_json"))]
    pub fn with_details_text(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self.mark_dirty();
        self
    }

    /// Borrow the attached metadata.
    #[must_use]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Borrow the backtrace, capturing it lazily when the `backtrace` feature
    /// is enabled.
    #[must_use]
    pub fn backtrace(&self) -> Option<&CapturedBacktrace> {
        self.capture_backtrace()
    }

    /// Borrow the source if present.
    #[must_use]
    pub fn source_ref(&self) -> Option<&(dyn CoreError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Human-readable message or the kind fallback.
    #[must_use]
    pub fn render_message(&self) -> Cow<'_, str> {
        match &self.message {
            Some(msg) => Cow::Borrowed(msg.as_ref()),
            None => Cow::Borrowed(self.kind.label())
        }
    }

    /// Emit telemetry (`tracing` event, metrics counter, backtrace capture).
    ///
    /// Downstream code can call this to guarantee telemetry after mutating the
    /// error. It is automatically invoked by constructors and conversions.
    pub fn log(&self) {
        self.emit_telemetry();
    }
}

/// Backwards-compatible export using the historical name.
pub use Error as AppError;

#[cfg(any(feature = "metrics", feature = "tracing"))]
fn kind_label(kind: AppErrorKind) -> &'static str {
    match kind {
        AppErrorKind::NotFound => "NotFound",
        AppErrorKind::Validation => "Validation",
        AppErrorKind::Conflict => "Conflict",
        AppErrorKind::Unauthorized => "Unauthorized",
        AppErrorKind::Forbidden => "Forbidden",
        AppErrorKind::NotImplemented => "NotImplemented",
        AppErrorKind::Internal => "Internal",
        AppErrorKind::BadRequest => "BadRequest",
        AppErrorKind::TelegramAuth => "TelegramAuth",
        AppErrorKind::InvalidJwt => "InvalidJwt",
        AppErrorKind::Database => "Database",
        AppErrorKind::Service => "Service",
        AppErrorKind::Config => "Config",
        AppErrorKind::Turnkey => "Turnkey",
        AppErrorKind::Timeout => "Timeout",
        AppErrorKind::Network => "Network",
        AppErrorKind::RateLimited => "RateLimited",
        AppErrorKind::DependencyUnavailable => "DependencyUnavailable",
        AppErrorKind::Serialization => "Serialization",
        AppErrorKind::Deserialization => "Deserialization",
        AppErrorKind::ExternalApi => "ExternalApi",
        AppErrorKind::Queue => "Queue",
        AppErrorKind::Cache => "Cache"
    }
}
