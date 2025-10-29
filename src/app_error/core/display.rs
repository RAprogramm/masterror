// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::string::ToString;
use core::{
    error::Error as CoreError,
    fmt::{Formatter, Result as FmtResult},
    sync::atomic::{AtomicU8, Ordering}
};

use super::error::Error;

/// Display mode for error output.
///
/// Controls the structure and verbosity of error messages based on
/// the deployment environment. The mode is determined by the
/// `MASTERROR_ENV` environment variable or auto-detected based on
/// build configuration and runtime environment.
///
/// # Examples
///
/// ```
/// use masterror::DisplayMode;
///
/// let mode = DisplayMode::current();
/// match mode {
///     DisplayMode::Prod => println!("Production mode: JSON output"),
///     DisplayMode::Local => println!("Local mode: Human-readable output"),
///     DisplayMode::Staging => println!("Staging mode: JSON with context")
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Production mode: lightweight JSON, minimal fields, no sensitive data.
    ///
    /// Output includes only: `kind`, `code`, `message` (if not redacted).
    /// Metadata is filtered to exclude sensitive fields.
    /// Source chain and backtrace are excluded.
    ///
    /// # Example Output
    ///
    /// ```json
    /// {"kind":"NotFound","code":"NOT_FOUND","message":"User not found"}
    /// ```
    Prod = 0,

    /// Development mode: human-readable, full context.
    ///
    /// Output includes: error details, full source chain, complete metadata,
    /// and backtrace (if enabled). Supports colored output when the `colored`
    /// feature is enabled and output is a TTY.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Error: NotFound
    /// Code: NOT_FOUND
    /// Message: User not found
    ///
    ///   Caused by: database query failed
    ///   Caused by: connection timeout
    ///
    /// Context:
    ///   user_id: 12345
    /// ```
    Local = 1,

    /// Staging mode: JSON with additional context.
    ///
    /// Output includes: `kind`, `code`, `message`, limited `source_chain`,
    /// and filtered metadata. No backtrace.
    ///
    /// # Example Output
    ///
    /// ```json
    /// {"kind":"NotFound","code":"NOT_FOUND","message":"User not found","source_chain":["database error"],"metadata":{"user_id":12345}}
    /// ```
    Staging = 2
}

impl DisplayMode {
    /// Returns the current display mode based on environment configuration.
    ///
    /// The mode is determined by checking (in order):
    /// 1. `MASTERROR_ENV` environment variable (`prod`, `local`, or `staging`)
    /// 2. Kubernetes environment detection (`KUBERNETES_SERVICE_HOST`)
    /// 3. Build configuration (`cfg!(debug_assertions)`)
    ///
    /// The result is cached for performance.
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
        static CACHED_MODE: AtomicU8 = AtomicU8::new(255);

        let cached = CACHED_MODE.load(Ordering::Relaxed);
        if cached != 255 {
            return match cached {
                0 => Self::Prod,
                1 => Self::Local,
                2 => Self::Staging,
                _ => unreachable!()
            };
        }

        let mode = Self::detect();
        CACHED_MODE.store(mode as u8, Ordering::Relaxed);
        mode
    }

    /// Detects the appropriate display mode from environment.
    ///
    /// This is an internal helper called by [`current()`](Self::current).
    fn detect() -> Self {
        #[cfg(feature = "std")]
        {
            if let Ok(env) = std::env::var("MASTERROR_ENV") {
                return match env.as_str() {
                    "prod" | "production" => Self::Prod,
                    "local" | "dev" | "development" => Self::Local,
                    "staging" | "stage" => Self::Staging,
                    _ => Self::detect_auto()
                };
            }

            if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
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

#[allow(dead_code)]
impl Error {
    /// Formats error in production mode (compact JSON).
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::AppError;
    ///
    /// let error = AppError::not_found("User not found");
    /// let output = format!("{}", error);
    /// // In prod mode: {"kind":"NotFound","code":"NOT_FOUND","message":"User not found"}
    /// ```
    #[cfg(not(test))]
    pub(crate) fn fmt_prod(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_prod_impl(f)
    }

    #[cfg(test)]
    #[allow(missing_docs)]
    pub fn fmt_prod(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_prod_impl(f)
    }

    fn fmt_prod_impl(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, r#"{{"kind":"{:?}","code":"{}""#, self.kind, self.code)?;

        if !matches!(self.edit_policy, super::types::MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            write!(f, ",\"message\":\"")?;
            write_json_escaped(f, msg.as_ref())?;
            write!(f, "\"")?;
        }

        if !self.metadata.is_empty() {
            let has_public_fields =
                self.metadata
                    .iter_with_redaction()
                    .any(|(_, _, redaction)| {
                        !matches!(
                            redaction,
                            crate::app_error::metadata::FieldRedaction::Redact
                        )
                    });

            if has_public_fields {
                write!(f, r#","metadata":{{"#)?;
                let mut first = true;

                for (name, value, redaction) in self.metadata.iter_with_redaction() {
                    if matches!(
                        redaction,
                        crate::app_error::metadata::FieldRedaction::Redact
                    ) {
                        continue;
                    }

                    if !first {
                        write!(f, ",")?;
                    }
                    first = false;

                    write!(f, r#""{}":"#, name)?;
                    write_metadata_value(f, value)?;
                }

                write!(f, "}}")?;
            }
        }

        write!(f, "}}")
    }

    /// Formats error in local/development mode (human-readable).
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::AppError;
    ///
    /// let error = AppError::internal("Database error");
    /// let output = format!("{}", error);
    /// // In local mode: multi-line human-readable format with full context
    /// ```
    #[cfg(not(test))]
    pub(crate) fn fmt_local(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_local_impl(f)
    }

    #[cfg(test)]
    #[allow(missing_docs)]
    pub fn fmt_local(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_local_impl(f)
    }

    fn fmt_local_impl(&self, f: &mut Formatter<'_>) -> FmtResult {
        #[cfg(feature = "colored")]
        {
            use crate::colored::style;

            writeln!(f, "Error: {}", self.kind)?;
            writeln!(f, "Code: {}", style::error_code(self.code.to_string()))?;

            if let Some(msg) = &self.message {
                writeln!(f, "Message: {}", style::error_message(msg))?;
            }

            if let Some(source) = &self.source {
                writeln!(f)?;
                let mut current: &dyn CoreError = source.as_ref();
                let mut depth = 0;

                while depth < 10 {
                    writeln!(
                        f,
                        "  {}: {}",
                        style::source_context("Caused by"),
                        style::source_context(current.to_string())
                    )?;

                    if let Some(next) = current.source() {
                        current = next;
                        depth += 1;
                    } else {
                        break;
                    }
                }
            }

            if !self.metadata.is_empty() {
                writeln!(f)?;
                writeln!(f, "Context:")?;
                for (key, value) in self.metadata.iter() {
                    writeln!(f, "  {}: {}", style::metadata_key(key), value)?;
                }
            }

            Ok(())
        }

        #[cfg(not(feature = "colored"))]
        {
            writeln!(f, "Error: {}", self.kind)?;
            writeln!(f, "Code: {}", self.code)?;

            if let Some(msg) = &self.message {
                writeln!(f, "Message: {}", msg)?;
            }

            if let Some(source) = &self.source {
                writeln!(f)?;
                let mut current: &dyn CoreError = source.as_ref();
                let mut depth = 0;

                while depth < 10 {
                    writeln!(f, "  Caused by: {}", current)?;

                    if let Some(next) = current.source() {
                        current = next;
                        depth += 1;
                    } else {
                        break;
                    }
                }
            }

            if !self.metadata.is_empty() {
                writeln!(f)?;
                writeln!(f, "Context:")?;
                for (key, value) in self.metadata.iter() {
                    writeln!(f, "  {}: {}", key, value)?;
                }
            }

            Ok(())
        }
    }

    /// Formats error in staging mode (JSON with context).
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::AppError;
    ///
    /// let error = AppError::service("Service unavailable");
    /// let output = format!("{}", error);
    /// // In staging mode: JSON with source_chain and metadata
    /// ```
    #[cfg(not(test))]
    pub(crate) fn fmt_staging(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_staging_impl(f)
    }

    #[cfg(test)]
    #[allow(missing_docs)]
    pub fn fmt_staging(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_staging_impl(f)
    }

    fn fmt_staging_impl(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, r#"{{"kind":"{:?}","code":"{}""#, self.kind, self.code)?;

        if !matches!(self.edit_policy, super::types::MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            write!(f, ",\"message\":\"")?;
            write_json_escaped(f, msg.as_ref())?;
            write!(f, "\"")?;
        }

        if let Some(source) = &self.source {
            write!(f, r#","source_chain":["#)?;
            let mut current: &dyn CoreError = source.as_ref();
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

        if !self.metadata.is_empty() {
            let has_public_fields =
                self.metadata
                    .iter_with_redaction()
                    .any(|(_, _, redaction)| {
                        !matches!(
                            redaction,
                            crate::app_error::metadata::FieldRedaction::Redact
                        )
                    });

            if has_public_fields {
                write!(f, r#","metadata":{{"#)?;
                let mut first = true;

                for (name, value, redaction) in self.metadata.iter_with_redaction() {
                    if matches!(
                        redaction,
                        crate::app_error::metadata::FieldRedaction::Redact
                    ) {
                        continue;
                    }

                    if !first {
                        write!(f, ",")?;
                    }
                    first = false;

                    write!(f, r#""{}":"#, name)?;
                    write_metadata_value(f, value)?;
                }

                write!(f, "}}")?;
            }
        }

        write!(f, "}}")
    }
}

/// Writes a string with JSON escaping.
#[allow(dead_code)]
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
#[allow(dead_code)]
fn write_metadata_value(
    f: &mut Formatter<'_>,
    value: &crate::app_error::metadata::FieldValue
) -> FmtResult {
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
    use super::*;
    use crate::{AppError, field};

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
            .with_field(field::str("key", "value"))
            .with_field(field::i64("count", -42));
        let output = format!("{}", error.fmt_local_wrapper());

        assert!(output.contains("Context:"));
        assert!(output.contains("key: value"));
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
        let first = DisplayMode::current();
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

    #[cfg(feature = "colored")]
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
