// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::string::{String, ToString};
use core::{
    error::Error as CoreError,
    fmt::{Formatter, Result as FmtResult},
    sync::atomic::{AtomicU8, Ordering}
};

use super::error::Error;
use crate::{
    FieldRedaction, FieldValue, MessageEditPolicy, Metadata,
    app_error::redaction::{REDACTED_PLACEHOLDER, hash_field_value, mask_last4_field_value}
};

/// Sentinel stored in [`CACHED_MODE`] while no mode has been detected yet.
const MODE_CACHE_UNSET: u8 = 255;

/// Process-wide cache holding the detected [`DisplayMode`] discriminant.
static CACHED_MODE: AtomicU8 = AtomicU8::new(MODE_CACHE_UNSET);

/// Detected deployment environment driving the `Display` layout of
/// [`struct@crate::Error`].
///
/// [`DisplayMode::current`] identifies the environment the process runs in,
/// based on the `MASTERROR_ENV` environment variable, Kubernetes detection
/// (`KUBERNETES_SERVICE_HOST`) or build configuration, and caches the result.
///
/// The `Display` implementation for [`struct@crate::Error`] dispatches on
/// this mode: `Local` renders a multi-line human-readable report, while
/// `Prod` and `Staging` render compact single-line JSON. Set
/// `MASTERROR_ENV=local` to force the human-readable layout in any
/// environment.
///
/// # Examples
///
/// ```
/// use masterror::DisplayMode;
///
/// let mode = DisplayMode::current();
/// match mode {
///     DisplayMode::Prod => println!("Production environment"),
///     DisplayMode::Local => println!("Local development environment"),
///     DisplayMode::Staging => println!("Staging environment")
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Production environment.
    ///
    /// Selected when `MASTERROR_ENV` is `prod`/`production`, when
    /// `KUBERNETES_SERVICE_HOST` is set, or for release builds by default.
    /// Errors render as compact JSON without a source chain.
    Prod = 0,

    /// Local development environment.
    ///
    /// Selected when `MASTERROR_ENV` is `local`/`dev`/`development`, or for
    /// debug builds by default. Errors render as a multi-line
    /// human-readable report; the `colored` feature adds ANSI styling to
    /// this layout only.
    Local = 1,

    /// Staging environment.
    ///
    /// Selected when `MASTERROR_ENV` is `staging`/`stage`. Errors render as
    /// compact JSON extended with the source chain.
    Staging = 2
}

impl DisplayMode {
    /// Returns the detected environment based on configuration.
    ///
    /// The mode is determined by checking (in order):
    /// 1. `MASTERROR_ENV` environment variable (`prod`, `local`, or `staging`)
    /// 2. Kubernetes environment detection (`KUBERNETES_SERVICE_HOST`)
    /// 3. Build configuration (`cfg!(debug_assertions)`)
    ///
    /// Without the `std` feature only step 3 applies. The result is cached
    /// on first access, so the environment is read once per process and
    /// later changes to the variables have no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::DisplayMode;
    ///
    /// let mode = DisplayMode::current();
    /// assert!(matches!(
    ///     mode,
    ///     DisplayMode::Prod | DisplayMode::Local | DisplayMode::Staging
    /// ));
    /// ```
    #[must_use]
    pub fn current() -> Self {
        #[cfg(test)]
        if let Some(mode) = test_display_mode_override::get() {
            return mode;
        }
        let cached = CACHED_MODE.load(Ordering::Relaxed);
        if cached != MODE_CACHE_UNSET {
            return Self::from_discriminant(cached);
        }
        let mode = Self::detect();
        CACHED_MODE.store(mode as u8, Ordering::Relaxed);
        mode
    }

    /// Converts a cached discriminant back into a mode.
    fn from_discriminant(value: u8) -> Self {
        match value {
            0 => Self::Prod,
            2 => Self::Staging,
            _ => Self::Local
        }
    }

    /// Detects the appropriate display mode from environment.
    ///
    /// This is an internal helper called by [`current()`](Self::current).
    fn detect() -> Self {
        #[cfg(feature = "std")]
        {
            use std::env::var;
            if let Ok(env) = var("MASTERROR_ENV") {
                return match env.as_str() {
                    "prod" | "production" => Self::Prod,
                    "local" | "dev" | "development" => Self::Local,
                    "staging" | "stage" => Self::Staging,
                    _ => Self::detect_auto()
                };
            }
            if var("KUBERNETES_SERVICE_HOST").is_ok() {
                return Self::Prod;
            }
        }
        Self::detect_auto()
    }

    /// Auto-detects mode based on build configuration.
    fn detect_auto() -> Self {
        if cfg!(debug_assertions) {
            Self::Local
        } else {
            Self::Prod
        }
    }
}

/// Overrides the detected display mode for testing purposes.
///
/// Setting an override clears the process-wide cache so the next
/// [`DisplayMode::current`] call observes the new value. Overridden modes are
/// consulted before the cache and never stored in it, keeping detection
/// deterministic for tests that do not override.
///
/// # Arguments
///
/// * `mode` - `Some(mode)` to force a mode, `None` to clear the override
#[cfg(test)]
pub(crate) fn set_display_mode_override(mode: Option<DisplayMode>) {
    test_display_mode_override::set(mode);
    CACHED_MODE.store(MODE_CACHE_UNSET, Ordering::Relaxed);
}

/// Resets the display mode cache and override to the uninitialized state.
///
/// Forces the next [`DisplayMode::current`] call to re-run detection. Tests
/// call it after overriding the mode, mirroring
/// `reset_backtrace_preference`.
#[cfg(test)]
pub(crate) fn reset_display_mode() {
    test_display_mode_override::set(None);
    CACHED_MODE.store(MODE_CACHE_UNSET, Ordering::Relaxed);
}

#[cfg(test)]
mod test_display_mode_override {
    use core::sync::atomic::{AtomicU8, Ordering};

    use super::DisplayMode;

    const OVERRIDE_UNSET: u8 = 255;

    static OVERRIDE_STATE: AtomicU8 = AtomicU8::new(OVERRIDE_UNSET);

    pub(super) fn set(mode: Option<DisplayMode>) {
        let state = match mode {
            Some(mode) => mode as u8,
            None => OVERRIDE_UNSET
        };
        OVERRIDE_STATE.store(state, Ordering::Release);
    }

    pub(super) fn get() -> Option<DisplayMode> {
        match OVERRIDE_STATE.load(Ordering::Acquire) {
            OVERRIDE_UNSET => None,
            value => Some(DisplayMode::from_discriminant(value))
        }
    }
}

#[cfg(test)]
pub(crate) use test_support::force_display_mode;

#[cfg(test)]
mod test_support {
    use std::sync::{Mutex, MutexGuard, PoisonError};

    use super::{DisplayMode, reset_display_mode, set_display_mode_override};

    static DISPLAY_MODE_LOCK: Mutex<()> = Mutex::new(());

    /// Guard restoring the default display mode detection on drop.
    pub(crate) struct DisplayModeGuard {
        _lock: MutexGuard<'static, ()>
    }

    impl Drop for DisplayModeGuard {
        fn drop(&mut self) {
            reset_display_mode();
        }
    }

    /// Forces the display mode for the lifetime of the returned guard.
    ///
    /// Serializes tests that override the mode so concurrent overrides do
    /// not observe each other.
    pub(crate) fn force_display_mode(mode: DisplayMode) -> DisplayModeGuard {
        let lock = DISPLAY_MODE_LOCK
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        set_display_mode_override(Some(mode));
        DisplayModeGuard {
            _lock: lock
        }
    }
}

impl Error {
    /// Formats the error as compact JSON (`kind`, `code`, optional `message`,
    /// redaction-aware metadata).
    ///
    /// Selected by the `Display` implementation when [`DisplayMode::current`]
    /// returns [`DisplayMode::Prod`]. The output never contains ANSI escape
    /// sequences.
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    pub(crate) fn fmt_prod(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, r#"{{"kind":"{:?}","code":"{}""#, self.kind, self.code)?;
        if !matches!(self.edit_policy, MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            write!(f, ",\"message\":\"")?;
            write_json_escaped(f, msg.as_ref())?;
            write!(f, "\"")?;
        }
        write_json_metadata_section(f, &self.metadata)?;
        write!(f, "}}")
    }

    /// Formats the error as a multi-line human-readable report (kind, code,
    /// message, source chain, redaction-aware metadata).
    ///
    /// Selected by the `Display` implementation when [`DisplayMode::current`]
    /// returns [`DisplayMode::Local`]. The `colored` feature applies ANSI
    /// styling to this layout when the terminal supports it.
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    pub(crate) fn fmt_local(&self, f: &mut Formatter<'_>) -> FmtResult {
        #[cfg(feature = "colored")]
        use crate::colored::style;

        writeln!(f, "Error: {}", self.kind)?;
        #[cfg(feature = "colored")]
        writeln!(f, "Code: {}", style::error_code(self.code.to_string()))?;
        #[cfg(not(feature = "colored"))]
        writeln!(f, "Code: {}", self.code)?;
        if !matches!(self.edit_policy, MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            #[cfg(feature = "colored")]
            writeln!(f, "Message: {}", style::error_message(msg))?;
            #[cfg(not(feature = "colored"))]
            writeln!(f, "Message: {}", msg)?;
        }
        if let Some(source) = &self.source {
            writeln!(f)?;
            let mut current: &dyn CoreError = source.as_dyn();
            let mut depth = 0;
            while depth < 10 {
                #[cfg(feature = "colored")]
                writeln!(
                    f,
                    "  {}: {}",
                    style::source_context("Caused by"),
                    style::source_context(current.to_string())
                )?;
                #[cfg(not(feature = "colored"))]
                writeln!(f, "  Caused by: {}", current)?;
                if let Some(next) = current.source() {
                    current = next;
                    depth += 1;
                } else {
                    break;
                }
            }
        }
        write_local_metadata_section(f, &self.metadata)
    }

    /// Formats the error as JSON with additional context (`source_chain` and
    /// redaction-aware metadata).
    ///
    /// Selected by the `Display` implementation when [`DisplayMode::current`]
    /// returns [`DisplayMode::Staging`]. The output never contains ANSI
    /// escape sequences.
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    pub(crate) fn fmt_staging(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, r#"{{"kind":"{:?}","code":"{}""#, self.kind, self.code)?;
        if !matches!(self.edit_policy, MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            write!(f, ",\"message\":\"")?;
            write_json_escaped(f, msg.as_ref())?;
            write!(f, "\"")?;
        }
        if let Some(source) = &self.source {
            write!(f, r#","source_chain":["#)?;
            let mut current: &dyn CoreError = source.as_dyn();
            let mut depth = 0;
            let mut first = true;
            while depth < 5 {
                if !first {
                    write!(f, ",")?;
                }
                first = false;
                write!(f, "\"")?;
                write_json_escaped(f, &current.to_string())?;
                write!(f, "\"")?;
                if let Some(next) = current.source() {
                    current = next;
                    depth += 1;
                } else {
                    break;
                }
            }
            write!(f, "]")?;
        }
        write_json_metadata_section(f, &self.metadata)?;
        write!(f, "}}")
    }
}

/// Metadata value prepared for the local layout after applying redaction.
enum LocalFieldValue<'a> {
    /// Public value rendered as-is.
    Raw(&'a FieldValue),
    /// Value replaced by the redaction placeholder.
    Placeholder,
    /// Hashed or masked textual representation.
    Owned(String)
}

/// Writes the `,"metadata":{...}` JSON section applying field redaction.
///
/// Fields marked [`FieldRedaction::Redact`] render as the placeholder,
/// [`FieldRedaction::Hash`] as a SHA-256 hex digest and
/// [`FieldRedaction::Last4`] as a masked value. Fields whose masking yields
/// no value are omitted; if nothing remains, the section is skipped
/// entirely.
fn write_json_metadata_section(f: &mut Formatter<'_>, metadata: &Metadata) -> FmtResult {
    let mut wrote_any = false;
    for (name, value, redaction) in metadata.iter_with_redaction() {
        let masked = match redaction {
            FieldRedaction::Last4 => match mask_last4_field_value(value) {
                Some(masked) => Some(masked),
                None => continue
            },
            _ => None
        };
        if wrote_any {
            write!(f, ",")?;
        } else {
            write!(f, r#","metadata":{{"#)?;
            wrote_any = true;
        }
        write!(f, r#""{}":"#, name)?;
        match redaction {
            FieldRedaction::None => write_metadata_value(f, value)?,
            FieldRedaction::Redact => write!(f, "\"{}\"", REDACTED_PLACEHOLDER)?,
            FieldRedaction::Hash => write!(f, "\"{}\"", hash_field_value(value))?,
            FieldRedaction::Last4 => {
                write!(f, "\"")?;
                write_json_escaped(f, masked.as_deref().unwrap_or_default())?;
                write!(f, "\"")?;
            }
        }
    }
    if wrote_any {
        write!(f, "}}")?;
    }
    Ok(())
}

/// Writes the `Context:` block of the local layout applying field redaction.
///
/// Applies the same policies as [`write_json_metadata_section`]; the header
/// is skipped when every field is omitted.
fn write_local_metadata_section(f: &mut Formatter<'_>, metadata: &Metadata) -> FmtResult {
    let mut wrote_header = false;
    for (name, value, redaction) in metadata.iter_with_redaction() {
        let rendered = match redaction {
            FieldRedaction::None => LocalFieldValue::Raw(value),
            FieldRedaction::Redact => LocalFieldValue::Placeholder,
            FieldRedaction::Hash => LocalFieldValue::Owned(hash_field_value(value)),
            FieldRedaction::Last4 => match mask_last4_field_value(value) {
                Some(masked) => LocalFieldValue::Owned(masked),
                None => continue
            }
        };
        if !wrote_header {
            writeln!(f)?;
            writeln!(f, "Context:")?;
            wrote_header = true;
        }
        #[cfg(feature = "colored")]
        write!(f, "  {}: ", crate::colored::style::metadata_key(name))?;
        #[cfg(not(feature = "colored"))]
        write!(f, "  {}: ", name)?;
        match rendered {
            LocalFieldValue::Raw(value) => writeln!(f, "{}", value)?,
            LocalFieldValue::Placeholder => writeln!(f, "{}", REDACTED_PLACEHOLDER)?,
            LocalFieldValue::Owned(text) => writeln!(f, "{}", text)?
        }
    }
    Ok(())
}

/// Writes a string with JSON escaping.
fn write_json_escaped(f: &mut Formatter<'_>, s: &str) -> FmtResult {
    for ch in s.chars() {
        match ch {
            '"' => write!(f, "\\\"")?,
            '\\' => write!(f, "\\\\")?,
            '\n' => write!(f, "\\n")?,
            '\r' => write!(f, "\\r")?,
            '\t' => write!(f, "\\t")?,
            ch if ch.is_control() => write!(f, "\\u{:04x}", ch as u32)?,
            ch => write!(f, "{}", ch)?
        }
    }
    Ok(())
}

/// Writes a metadata field value in JSON format.
fn write_metadata_value(f: &mut Formatter<'_>, value: &FieldValue) -> FmtResult {
    use crate::app_error::metadata::FieldValue;
    match value {
        FieldValue::Str(s) => {
            write!(f, "\"")?;
            write_json_escaped(f, s.as_ref())?;
            write!(f, "\"")
        }
        FieldValue::I64(v) => write!(f, "{}", v),
        FieldValue::U64(v) => write!(f, "{}", v),
        FieldValue::F64(v) => {
            if v.is_finite() {
                write!(f, "{}", v)
            } else {
                write!(f, "null")
            }
        }
        FieldValue::Bool(v) => write!(f, "{}", v),
        FieldValue::Uuid(v) => write!(f, "\"{}\"", v),
        FieldValue::Duration(v) => {
            write!(
                f,
                r#"{{"secs":{},"nanos":{}}}"#,
                v.as_secs(),
                v.subsec_nanos()
            )
        }
        FieldValue::Ip(v) => write!(f, "\"{}\"", v),
        #[cfg(feature = "serde_json")]
        FieldValue::Json(v) => write!(f, "{}", v)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use sha2::{Digest, Sha256};

    use super::*;
    use crate::{AppError, field};

    fn sha256_hex(input: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher
            .finalize()
            .iter()
            .fold(String::with_capacity(64), |mut acc, byte| {
                let _ = write!(&mut acc, "{:02x}", byte);
                acc
            })
    }

    #[test]
    fn display_mode_current_returns_valid_mode() {
        let mode = DisplayMode::current();
        assert!(matches!(
            mode,
            DisplayMode::Prod | DisplayMode::Local | DisplayMode::Staging
        ));
    }

    #[test]
    fn display_mode_detect_auto_returns_local_in_debug() {
        if cfg!(debug_assertions) {
            assert_eq!(DisplayMode::detect_auto(), DisplayMode::Local);
        } else {
            assert_eq!(DisplayMode::detect_auto(), DisplayMode::Prod);
        }
    }

    #[test]
    fn display_mode_override_takes_priority() {
        let _guard = force_display_mode(DisplayMode::Staging);
        assert_eq!(DisplayMode::current(), DisplayMode::Staging);
    }

    #[test]
    fn display_dispatches_prod_layout() {
        let _guard = force_display_mode(DisplayMode::Prod);
        let error = AppError::not_found("missing user");
        let output = format!("{}", error);
        assert!(output.starts_with(r#"{"kind":"NotFound""#), "{output}");
        assert!(output.contains(r#""code":"NOT_FOUND""#));
        assert!(output.contains(r#""message":"missing user""#));
        assert!(!output.contains('\u{1b}'));
    }

    #[cfg(feature = "std")]
    #[test]
    fn display_dispatches_staging_layout() {
        use std::io::Error as IoError;
        let _guard = force_display_mode(DisplayMode::Staging);
        let error = AppError::network("upstream down").with_source(IoError::other("timeout"));
        let output = format!("{}", error);
        assert!(output.starts_with(r#"{"kind":"Network""#), "{output}");
        assert!(output.contains(r#""source_chain":["timeout"]"#));
        assert!(!output.contains('\u{1b}'));
    }

    #[test]
    fn display_dispatches_local_layout() {
        let _guard = force_display_mode(DisplayMode::Local);
        let error = AppError::not_found("missing user");
        let output = format!("{}", error);
        assert!(output.contains("Error:"));
        assert!(output.contains("Code: NOT_FOUND"));
        assert!(output.contains("missing user"));
    }

    #[test]
    fn fmt_prod_outputs_json() {
        let error = AppError::not_found("User not found");
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""kind":"NotFound""#));
        assert!(output.contains(r#""code":"NOT_FOUND""#));
        assert!(output.contains(r#""message":"User not found""#));
    }

    #[test]
    fn fmt_prod_excludes_redacted_message() {
        let error = AppError::internal("secret").redactable();
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(!output.contains("secret"));
    }

    #[test]
    fn fmt_prod_includes_metadata() {
        let error = AppError::not_found("User not found").with_field(field::u64("user_id", 12345));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""metadata""#));
        assert!(output.contains(r#""user_id":12345"#));
    }

    #[test]
    fn fmt_prod_excludes_sensitive_metadata() {
        let error = AppError::internal("Error").with_field(field::str("password", "secret"));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(!output.contains("secret"));
        assert!(output.contains(r#""password":"[REDACTED]""#));
    }

    #[test]
    fn hash_redaction_renders_hex_digest_in_all_modes() {
        let expected = sha256_hex(b"super-secret");
        let error = AppError::internal("Error").with_field(
            field::str("fingerprint", "super-secret").with_redaction(FieldRedaction::Hash)
        );
        let prod = format!("{}", error.fmt_prod_wrapper());
        let staging = format!("{}", error.fmt_staging_wrapper());
        let local = format!("{}", error.fmt_local_wrapper());
        for output in [&prod, &staging, &local] {
            assert!(output.contains(&expected), "{output}");
            assert!(!output.contains("super-secret"), "{output}");
        }
        assert!(prod.contains(&format!(r#""fingerprint":"{}""#, expected)));
        assert!(staging.contains(&format!(r#""fingerprint":"{}""#, expected)));
        assert!(local.contains(&format!("fingerprint: {}", expected)));
    }

    #[test]
    fn last4_redaction_masks_value_in_all_modes() {
        let error = AppError::internal("Error").with_field(
            field::str("card_number", "4111111111111111").with_redaction(FieldRedaction::Last4)
        );
        let prod = format!("{}", error.fmt_prod_wrapper());
        let staging = format!("{}", error.fmt_staging_wrapper());
        let local = format!("{}", error.fmt_local_wrapper());
        for output in [&prod, &staging, &local] {
            assert!(output.contains("************1111"), "{output}");
            assert!(!output.contains("4111111111111111"), "{output}");
        }
    }

    #[test]
    fn last4_redaction_omits_unmaskable_field() {
        let error = AppError::internal("Error")
            .with_field(field::bool("consent", true).with_redaction(FieldRedaction::Last4));
        let prod = format!("{}", error.fmt_prod_wrapper());
        let staging = format!("{}", error.fmt_staging_wrapper());
        let local = format!("{}", error.fmt_local_wrapper());
        assert!(!prod.contains("metadata"), "{prod}");
        assert!(!staging.contains("metadata"), "{staging}");
        assert!(!local.contains("Context:"), "{local}");
        for output in [&prod, &staging, &local] {
            assert!(!output.contains("consent"), "{output}");
        }
    }

    #[test]
    fn redact_policy_uses_placeholder_in_local_layout() {
        let error = AppError::internal("Error").with_field(field::str("password", "hunter2"));
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(output.contains("password: [REDACTED]"), "{output}");
        assert!(!output.contains("hunter2"));
    }

    #[test]
    fn fmt_local_hides_redacted_message() {
        let error = AppError::internal("sensitive data").redactable();
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(!output.contains("sensitive data"));
        assert!(!output.contains("Message:"));
    }

    #[test]
    fn fmt_local_outputs_human_readable() {
        let error = AppError::not_found("User not found");
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(output.contains("Error:"));
        assert!(output.contains("Code: NOT_FOUND"));
        assert!(output.contains("Message: User not found"));
    }

    #[cfg(feature = "std")]
    #[test]
    fn fmt_local_includes_source_chain() {
        use std::io::Error as IoError;
        let io_err = IoError::other("connection failed");
        let error = AppError::internal("Database error").with_source(io_err);
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(output.contains("Caused by"));
        assert!(output.contains("connection failed"));
    }

    #[test]
    fn fmt_staging_outputs_json_with_context() {
        let error = AppError::service("Service unavailable");
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""kind":"Service""#));
        assert!(output.contains(r#""code":"SERVICE""#));
    }

    #[cfg(feature = "std")]
    #[test]
    fn fmt_staging_includes_source_chain() {
        use std::io::Error as IoError;
        let io_err = IoError::other("timeout");
        let error = AppError::network("Network error").with_source(io_err);
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""source_chain""#));
        assert!(output.contains("timeout"));
    }

    #[test]
    fn fmt_prod_escapes_special_chars() {
        let error = AppError::internal("Line\nwith\"quotes\"");
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#"\n"#));
        assert!(output.contains(r#"\""#));
    }

    #[test]
    fn fmt_prod_handles_infinity_in_metadata() {
        let error = AppError::internal("Error").with_field(field::f64("ratio", f64::INFINITY));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains("null"));
    }

    #[test]
    fn fmt_prod_formats_duration_metadata() {
        use core::time::Duration;
        let error = AppError::internal("Error")
            .with_field(field::duration("elapsed", Duration::from_millis(1500)));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""secs":1"#));
        assert!(output.contains(r#""nanos":500000000"#));
    }

    #[test]
    fn fmt_prod_formats_bool_metadata() {
        let error = AppError::internal("Error").with_field(field::bool("active", true));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""active":true"#));
    }

    #[cfg(feature = "std")]
    #[test]
    fn fmt_prod_formats_ip_metadata() {
        use std::net::IpAddr;
        let ip: IpAddr = "192.168.1.1".parse().unwrap();
        let error = AppError::internal("Error").with_field(field::ip("client_ip", ip));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""client_ip":"192.168.1.1""#));
    }

    #[test]
    fn fmt_prod_formats_uuid_metadata() {
        use uuid::Uuid;
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let error = AppError::internal("Error").with_field(field::uuid("request_id", uuid));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""request_id":"550e8400-e29b-41d4-a716-446655440000""#));
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn fmt_prod_formats_json_metadata() {
        let json = serde_json::json!({"nested": "value"});
        let error = AppError::internal("Error").with_field(field::json("data", json));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""data":"#));
    }

    #[test]
    fn fmt_prod_without_message() {
        let error = AppError::bare(crate::AppErrorKind::Internal);
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""kind":"Internal""#));
        assert!(!output.contains(r#""message""#));
    }

    #[test]
    fn fmt_local_without_message() {
        let error = AppError::bare(crate::AppErrorKind::BadRequest);
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(output.contains("Error:"));
        assert!(!output.contains("Message:"));
    }

    #[test]
    fn fmt_local_with_metadata() {
        let error = AppError::internal("Error")
            .with_field(field::str("region", "value"))
            .with_field(field::i64("count", -42));
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(output.contains("Context:"));
        assert!(output.contains("region: value"));
        assert!(output.contains("count: -42"));
    }

    #[test]
    fn fmt_staging_without_message() {
        let error = AppError::bare(crate::AppErrorKind::Timeout);
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""kind":"Timeout""#));
        assert!(!output.contains(r#""message""#));
    }

    #[test]
    fn fmt_staging_with_metadata() {
        let error = AppError::service("Service error").with_field(field::u64("retry_count", 3));
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""metadata""#));
        assert!(output.contains(r#""retry_count":3"#));
    }

    #[test]
    fn fmt_staging_with_redacted_message() {
        let error = AppError::internal("sensitive data").redactable();
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(!output.contains("sensitive data"));
    }

    #[test]
    fn fmt_prod_escapes_control_chars() {
        let error = AppError::internal("test\x00\x1F");
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#"\u0000"#));
        assert!(output.contains(r#"\u001f"#));
    }

    #[test]
    fn fmt_prod_escapes_tab_and_carriage_return() {
        let error = AppError::internal("line\ttab\rreturn");
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#"\t"#));
        assert!(output.contains(r#"\r"#));
    }

    #[test]
    fn display_mode_current_caches_result() {
        let _guard = force_display_mode(DisplayMode::Staging);
        reset_display_mode();
        assert_eq!(CACHED_MODE.load(Ordering::Relaxed), MODE_CACHE_UNSET);
        let first = DisplayMode::current();
        assert_eq!(CACHED_MODE.load(Ordering::Relaxed), first as u8);
        let second = DisplayMode::current();
        assert_eq!(first, second);
    }

    #[test]
    fn display_mode_detect_auto_returns_prod_in_release() {
        if !cfg!(debug_assertions) {
            assert_eq!(DisplayMode::detect_auto(), DisplayMode::Prod);
        }
    }

    #[test]
    fn fmt_prod_with_multiple_metadata_fields() {
        let error = AppError::not_found("test")
            .with_field(field::str("first", "value1"))
            .with_field(field::u64("second", 42))
            .with_field(field::bool("third", true));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""first":"value1""#));
        assert!(output.contains(r#""second":42"#));
        assert!(output.contains(r#""third":true"#));
    }

    #[test]
    fn fmt_prod_escapes_backslash() {
        let error = AppError::internal("path\\to\\file");
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#"path\\to\\file"#));
    }

    #[test]
    fn fmt_prod_with_i64_metadata() {
        let error = AppError::internal("test").with_field(field::i64("count", -100));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""count":-100"#));
    }

    #[test]
    fn fmt_prod_with_string_metadata() {
        let error = AppError::internal("test").with_field(field::str("name", "value"));
        let output = format!("{}", error.fmt_prod_wrapper());
        assert!(output.contains(r#""name":"value""#));
    }

    #[cfg(feature = "std")]
    #[test]
    fn fmt_local_with_deep_source_chain() {
        use std::io::{Error as IoError, ErrorKind};
        let io1 = IoError::new(ErrorKind::NotFound, "level 1");
        let io2 = IoError::other(io1);
        let error = AppError::internal("top").with_source(io2);
        let output = format!("{}", error.fmt_local_wrapper());
        assert!(output.contains("Caused by"));
        assert!(output.contains("level 1"));
    }

    #[test]
    fn fmt_staging_with_multiple_metadata_fields() {
        let error = AppError::service("error")
            .with_field(field::str("key1", "value1"))
            .with_field(field::u64("key2", 123))
            .with_field(field::bool("key3", false));
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""key1":"value1""#));
        assert!(output.contains(r#""key2":123"#));
        assert!(output.contains(r#""key3":false"#));
    }

    #[test]
    fn fmt_staging_with_deep_source_chain() {
        use std::io::{Error as IoError, ErrorKind};
        let io1 = IoError::new(ErrorKind::NotFound, "inner error");
        let io2 = IoError::other(io1);
        let error = AppError::service("outer").with_source(io2);
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""source_chain""#));
        assert!(output.contains("inner error"));
    }

    #[test]
    fn fmt_staging_with_redacted_and_public_metadata() {
        let error = AppError::internal("test")
            .with_field(field::str("public", "visible"))
            .with_field(field::str("password", "secret"));
        let output = format!("{}", error.fmt_staging_wrapper());
        assert!(output.contains(r#""public":"visible""#));
        assert!(output.contains(r#""password":"[REDACTED]""#));
        assert!(!output.contains("secret"));
    }

    impl Error {
        fn fmt_prod_wrapper(&self) -> FormatterWrapper<'_> {
            FormatterWrapper {
                error: self,
                mode:  FormatterMode::Prod
            }
        }

        fn fmt_local_wrapper(&self) -> FormatterWrapper<'_> {
            FormatterWrapper {
                error: self,
                mode:  FormatterMode::Local
            }
        }

        fn fmt_staging_wrapper(&self) -> FormatterWrapper<'_> {
            FormatterWrapper {
                error: self,
                mode:  FormatterMode::Staging
            }
        }
    }

    enum FormatterMode {
        Prod,
        Local,
        Staging
    }

    struct FormatterWrapper<'a> {
        error: &'a Error,
        mode:  FormatterMode
    }

    impl core::fmt::Display for FormatterWrapper<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self.mode {
                FormatterMode::Prod => self.error.fmt_prod(f),
                FormatterMode::Local => self.error.fmt_local(f),
                FormatterMode::Staging => self.error.fmt_staging(f)
            }
        }
    }
}
