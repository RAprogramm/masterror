#[cfg(feature = "backtrace")]
use std::{
    backtrace::Backtrace,
    env,
    sync::{
        OnceLock,
        atomic::{AtomicU8, Ordering as AtomicOrdering}
    }
};
use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    }
};

#[cfg(feature = "tracing")]
use tracing::{Level, event};

use super::metadata::{Field, FieldRedaction, Metadata};
use crate::{AppCode, AppErrorKind, RetryAdvice};

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
    pub source:             Option<Arc<dyn StdError + Send + Sync + 'static>>,
    #[cfg(feature = "backtrace")]
    pub backtrace:          Option<Backtrace>,
    #[cfg(feature = "backtrace")]
    pub captured_backtrace: OnceLock<Option<Backtrace>>,
    telemetry_dirty:        AtomicBool
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

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
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
                source: None,
                #[cfg(feature = "backtrace")]
                backtrace: None,
                #[cfg(feature = "backtrace")]
                captured_backtrace: OnceLock::new(),
                telemetry_dirty: AtomicBool::new(true)
            })
        }
    }

    fn mark_dirty(&self) {
        self.telemetry_dirty.store(true, Ordering::Release);
    }

    fn take_dirty(&self) -> bool {
        self.telemetry_dirty.swap(false, Ordering::AcqRel)
    }

    #[cfg(feature = "backtrace")]
    fn capture_backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        if let Some(backtrace) = self.backtrace.as_ref() {
            return Some(backtrace);
        }

        self.captured_backtrace
            .get_or_init(capture_backtrace_snapshot)
            .as_ref()
    }

    #[cfg(not(feature = "backtrace"))]
    fn capture_backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        None
    }

    #[cfg(feature = "backtrace")]
    fn set_backtrace_slot(&mut self, backtrace: std::backtrace::Backtrace) {
        self.backtrace = Some(backtrace);
        self.captured_backtrace = OnceLock::new();
    }

    #[cfg(not(feature = "backtrace"))]
    fn set_backtrace_slot(&mut self, _backtrace: std::backtrace::Backtrace) {}

    pub(crate) fn emit_telemetry(&self) {
        if self.take_dirty() {
            #[cfg(feature = "backtrace")]
            let _ = self.capture_backtrace();

            #[cfg(feature = "metrics")]
            {
                metrics::counter!(
                    "error_total",
                    "code" => self.code.as_str(),
                    "category" => kind_label(self.kind)
                )
                .increment(1);
            }

            #[cfg(feature = "tracing")]
            {
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
        }
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

    /// Attach a source error for diagnostics.
    #[must_use]
    pub fn with_source(mut self, source: impl StdError + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self.mark_dirty();
        self
    }

    /// Attach a shared source error without cloning the underlying `Arc`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    ///
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let source = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
    /// let err = AppError::internal("boom").with_source_arc(source.clone());
    /// assert!(err.source_ref().is_some());
    /// assert_eq!(Arc::strong_count(&source), 2);
    /// ```
    #[must_use]
    pub fn with_source_arc(mut self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self {
        self.source = Some(source);
        self.mark_dirty();
        self
    }

    /// Attach a captured backtrace.
    #[must_use]
    pub fn with_backtrace(mut self, backtrace: std::backtrace::Backtrace) -> Self {
        self.set_backtrace_slot(backtrace);
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
    pub fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.capture_backtrace()
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
