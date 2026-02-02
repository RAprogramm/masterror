// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
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

#[cfg(feature = "std")]
mod std_style;

#[cfg(not(feature = "std"))]
mod nostd_style;

#[cfg(test)]
mod tests;

/// Color styling functions with automatic TTY detection.
///
/// Each function applies appropriate ANSI color codes when stderr supports
/// colors. When colors are not supported, the original text is returned
/// unchanged.
pub mod style {
    #[cfg(not(feature = "std"))]
    pub use super::nostd_style::*;
    #[cfg(feature = "std")]
    pub use super::std_style::*;
}
