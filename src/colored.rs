// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Terminal color styling for error output with automatic detection.
//!
//! This module provides zero-cost color styling that automatically detects
//! terminal capabilities and respects environment-based color preferences.
//!
//! # Automatic Color Detection
//!
//! Colors are applied only when all of the following conditions are met:
//! - stderr is connected to a TTY
//! - `NO_COLOR` environment variable is not set
//! - `TERM` is not set to `dumb`
//! - Terminal supports ANSI colors
//!
//! # Platform Support
//!
//! - Linux/Unix: Full ANSI color support
//! - macOS: Full ANSI color support
//! - Windows 10+: Native ANSI support via Windows Terminal
//! - Older Windows: Graceful fallback to monochrome
//!
//! # Color Scheme
//!
//! The color scheme is designed for professional CLI tools:
//! - Critical errors: Red
//! - Warnings: Yellow
//! - Error codes: Cyan (easy to scan)
//! - Messages: Bright white (emphasis)
//! - Source context: Dimmed (secondary information)
//! - Metadata keys: Green (structured data)
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "colored")] {
//! use masterror::colored::style;
//!
//! let error_text = style::error_kind_critical("ServiceUnavailable");
//! let code_text = style::error_code("ERR_DB_001");
//! let msg_text = style::error_message("Database connection failed");
//!
//! eprintln!("Error: {}", error_text);
//! eprintln!("Code: {}", code_text);
//! eprintln!("{}", msg_text);
//! # }
//! ```
//!
//! # Integration with Display
//!
//! These functions are designed for use in `Display` implementations:
//!
//! ```rust
//! # #[cfg(feature = "colored")] {
//! use std::fmt;
//!
//! use masterror::colored::style;
//!
//! struct MyError {
//!     code:    String,
//!     message: String
//! }
//!
//! impl fmt::Display for MyError {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(
//!             f,
//!             "{}: {}",
//!             style::error_code(&self.code),
//!             style::error_message(&self.message)
//!         )
//!     }
//! }
//! # }
//! ```
//!
//! # Performance
//!
//! Terminal detection is cached per-process, resulting in negligible overhead.
//! Color styling only allocates when colors are actually applied.

/// Color styling functions with automatic TTY detection.
///
/// Each function applies appropriate ANSI color codes when stderr supports
/// colors. When colors are not supported, the original text is returned
/// unchanged.
#[cfg(feature = "std")]
pub mod style {
    use owo_colors::{OwoColorize, Stream};

    /// Style critical error kind text in red.
    ///
    /// Use this for error kinds that indicate critical failures requiring
    /// immediate attention.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "colored")] {
    /// use masterror::colored::style;
    ///
    /// let styled = style::error_kind_critical("ServiceUnavailable");
    /// eprintln!("Kind: {}", styled);
    /// # }
    /// ```
    ///
    /// # Color Behavior
    ///
    /// - TTY: Red text
    /// - Non-TTY: Plain text
    /// - NO_COLOR=1: Plain text
    pub fn error_kind_critical(text: impl AsRef<str>) -> String {
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.red())
            .to_string()
    }

    /// Style warning-level error kind text in yellow.
    ///
    /// Use this for error kinds that indicate recoverable issues or warnings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "colored")] {
    /// use masterror::colored::style;
    ///
    /// let styled = style::error_kind_warning("BadRequest");
    /// eprintln!("Kind: {}", styled);
    /// # }
    /// ```
    ///
    /// # Color Behavior
    ///
    /// - TTY: Yellow text
    /// - Non-TTY: Plain text
    /// - NO_COLOR=1: Plain text
    pub fn error_kind_warning(text: impl AsRef<str>) -> String {
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.yellow())
            .to_string()
    }

    /// Style error code text in cyan for easy visual scanning.
    ///
    /// Use this for machine-readable error codes that users need to reference
    /// in documentation or support requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "colored")] {
    /// use masterror::colored::style;
    ///
    /// let styled = style::error_code("ERR_DATABASE_001");
    /// eprintln!("Code: {}", styled);
    /// # }
    /// ```
    ///
    /// # Color Behavior
    ///
    /// - TTY: Cyan text
    /// - Non-TTY: Plain text
    /// - NO_COLOR=1: Plain text
    pub fn error_code(text: impl AsRef<str>) -> String {
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.cyan())
            .to_string()
    }

    /// Style error message text in bright white for maximum readability.
    ///
    /// Use this for the primary error message that describes what went wrong.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "colored")] {
    /// use masterror::colored::style;
    ///
    /// let styled = style::error_message("Failed to connect to database");
    /// eprintln!("{}", styled);
    /// # }
    /// ```
    ///
    /// # Color Behavior
    ///
    /// - TTY: Bright white text
    /// - Non-TTY: Plain text
    /// - NO_COLOR=1: Plain text
    pub fn error_message(text: impl AsRef<str>) -> String {
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.bright_white())
            .to_string()
    }

    /// Style source context text with dimmed appearance.
    ///
    /// Use this for error source chains and contextual information that is
    /// important but secondary to the main error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "colored")] {
    /// use masterror::colored::style;
    ///
    /// let styled = style::source_context("Caused by: Connection timeout");
    /// eprintln!("{}", styled);
    /// # }
    /// ```
    ///
    /// # Color Behavior
    ///
    /// - TTY: Dimmed text
    /// - Non-TTY: Plain text
    /// - NO_COLOR=1: Plain text
    pub fn source_context(text: impl AsRef<str>) -> String {
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.dimmed())
            .to_string()
    }

    /// Style metadata key text in green.
    ///
    /// Use this for structured metadata keys in error context to visually
    /// separate keys from values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "colored")] {
    /// use masterror::colored::style;
    ///
    /// let key = style::metadata_key("request_id");
    /// eprintln!("{}: abc123", key);
    /// # }
    /// ```
    ///
    /// # Color Behavior
    ///
    /// - TTY: Green text
    /// - Non-TTY: Plain text
    /// - NO_COLOR=1: Plain text
    pub fn metadata_key(text: impl AsRef<str>) -> String {
        text.as_ref()
            .if_supports_color(Stream::Stderr, |t| t.green())
            .to_string()
    }
}

/// No-op styling for no-std builds.
#[cfg(not(feature = "std"))]
pub mod style {
    /// Style critical error kind text in red.
    pub fn error_kind_critical(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    /// Style warning-level error kind text in yellow.
    pub fn error_kind_warning(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    /// Style error code text in cyan for easy visual scanning.
    pub fn error_code(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    /// Style error message text in bright white for maximum readability.
    pub fn error_message(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    /// Style source context text with dimmed appearance.
    pub fn source_context(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    /// Style metadata key text in green.
    pub fn metadata_key(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }
}

#[cfg(all(test, not(feature = "std")))]
mod nostd_tests {
    use super::style;

    #[test]
    fn error_kind_critical_returns_plain_text() {
        assert_eq!(style::error_kind_critical("test"), "test");
    }

    #[test]
    fn error_kind_warning_returns_plain_text() {
        assert_eq!(style::error_kind_warning("test"), "test");
    }

    #[test]
    fn error_code_returns_plain_text() {
        assert_eq!(style::error_code("ERR_001"), "ERR_001");
    }

    #[test]
    fn error_message_returns_plain_text() {
        assert_eq!(style::error_message("message"), "message");
    }

    #[test]
    fn source_context_returns_plain_text() {
        assert_eq!(style::source_context("context"), "context");
    }

    #[test]
    fn metadata_key_returns_plain_text() {
        assert_eq!(style::metadata_key("key"), "key");
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::style;

    #[test]
    fn error_kind_critical_produces_output() {
        let result = style::error_kind_critical("ServiceUnavailable");
        assert!(!result.is_empty());
        assert!(result.contains("ServiceUnavailable"));
    }

    #[test]
    fn error_kind_warning_produces_output() {
        let result = style::error_kind_warning("BadRequest");
        assert!(!result.is_empty());
        assert!(result.contains("BadRequest"));
    }

    #[test]
    fn error_code_produces_output() {
        let result = style::error_code("ERR_001");
        assert!(!result.is_empty());
        assert!(result.contains("ERR_001"));
    }

    #[test]
    fn error_message_produces_output() {
        let result = style::error_message("Connection failed");
        assert!(!result.is_empty());
        assert!(result.contains("Connection failed"));
    }

    #[test]
    fn source_context_produces_output() {
        let result = style::source_context("Caused by: timeout");
        assert!(!result.is_empty());
        assert!(result.contains("Caused by: timeout"));
    }

    #[test]
    fn metadata_key_produces_output() {
        let result = style::metadata_key("request_id");
        assert!(!result.is_empty());
        assert!(result.contains("request_id"));
    }

    #[test]
    fn style_functions_preserve_content() {
        let input = "test content with special chars: äöü";
        assert!(style::error_kind_critical(input).contains(input));
        assert!(style::error_kind_warning(input).contains(input));
        assert!(style::error_code(input).contains(input));
        assert!(style::error_message(input).contains(input));
        assert!(style::source_context(input).contains(input));
        assert!(style::metadata_key(input).contains(input));
    }
}
