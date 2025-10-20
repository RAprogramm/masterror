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
    pub fn error_kind_critical(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    pub fn error_kind_warning(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    pub fn error_code(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    pub fn error_message(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    pub fn source_context(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }

    pub fn metadata_key(text: impl AsRef<str>) -> String {
        text.as_ref().to_string()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::style;

    #[test]
    fn test_style_functions_produce_output() {
        assert!(!style::error_kind_critical("test").is_empty());
        assert!(!style::error_kind_warning("test").is_empty());
        assert!(!style::error_code("test").is_empty());
        assert!(!style::error_message("test").is_empty());
        assert!(!style::source_context("test").is_empty());
        assert!(!style::metadata_key("test").is_empty());
    }

    #[test]
    fn test_style_preserves_text_content() {
        let input = "test content";
        assert!(style::error_kind_critical(input).contains(input));
        assert!(style::error_code(input).contains(input));
        assert!(style::error_message(input).contains(input));
    }
}
