// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Color styling functions using owo_colors with automatic TTY detection.

use owo_colors::{OwoColorize, Stream, Style};

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

// ─────────────────────────────────────────────────────────────────────────────
// Diagnostic styling
// ─────────────────────────────────────────────────────────────────────────────

/// Style hint label in blue.
///
/// # Color Behavior
///
/// - TTY: Blue text
/// - Non-TTY: Plain text
pub fn hint_label(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.blue())
        .to_string()
}

/// Style hint message in bright blue.
///
/// # Color Behavior
///
/// - TTY: Bright blue text
/// - Non-TTY: Plain text
pub fn hint_text(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.bright_blue())
        .to_string()
}

/// Style suggestion label in magenta.
///
/// # Color Behavior
///
/// - TTY: Magenta text
/// - Non-TTY: Plain text
pub fn suggestion_label(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.magenta())
        .to_string()
}

/// Style suggestion message in bright magenta.
///
/// # Color Behavior
///
/// - TTY: Bright magenta text
/// - Non-TTY: Plain text
pub fn suggestion_text(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.bright_magenta())
        .to_string()
}

/// Style command/code snippet in bold bright white.
///
/// # Color Behavior
///
/// - TTY: Bold bright white text
/// - Non-TTY: Plain text
pub fn command(text: impl AsRef<str>) -> String {
    let style = Style::new().bold().bright_white();
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.style(style))
        .to_string()
}

/// Style documentation link label in cyan.
///
/// # Color Behavior
///
/// - TTY: Cyan text
/// - Non-TTY: Plain text
pub fn docs_label(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.cyan())
        .to_string()
}

/// Style URL in underlined cyan.
///
/// # Color Behavior
///
/// - TTY: Underlined cyan text
/// - Non-TTY: Plain text
pub fn url(text: impl AsRef<str>) -> String {
    let style = Style::new().underline().cyan();
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.style(style))
        .to_string()
}

/// Style "see also" label in dimmed text.
///
/// # Color Behavior
///
/// - TTY: Dimmed text
/// - Non-TTY: Plain text
pub fn related_label(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.dimmed())
        .to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// Backtrace styling
// ─────────────────────────────────────────────────────────────────────────────

/// Style backtrace header label in dimmed text.
///
/// # Color Behavior
///
/// - TTY: Dimmed text
/// - Non-TTY: Plain text
pub fn backtrace_label(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.dimmed())
        .to_string()
}

/// Style backtrace arrow symbol in yellow.
///
/// # Color Behavior
///
/// - TTY: Yellow text
/// - Non-TTY: Plain text
pub fn backtrace_arrow(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.yellow())
        .to_string()
}

/// Style backtrace function name in bright cyan.
///
/// # Color Behavior
///
/// - TTY: Bright cyan text
/// - Non-TTY: Plain text
pub fn backtrace_function(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.bright_cyan())
        .to_string()
}

/// Style backtrace file location in dimmed text.
///
/// # Color Behavior
///
/// - TTY: Dimmed text
/// - Non-TTY: Plain text
pub fn backtrace_location(text: impl AsRef<str>) -> String {
    text.as_ref()
        .if_supports_color(Stream::Stderr, |t| t.dimmed())
        .to_string()
}

/// Create a clickable hyperlink for file location in backtrace.
///
/// Uses OSC 8 escape sequences supported by modern terminals.
/// Falls back to plain text if not a TTY.
///
/// # Arguments
///
/// * `display` - Text to display (e.g., "src/main.rs:16")
/// * `absolute_path` - Absolute file path for the link
/// * `line` - Line number (optional)
///
/// # Color Behavior
///
/// - TTY: Clickable hyperlink with dimmed text
/// - Non-TTY: Plain text
pub fn backtrace_link(display: &str, absolute_path: &str, line: Option<u32>) -> String {
    use std::io::IsTerminal;

    if !std::io::stderr().is_terminal() {
        return display.to_string();
    }

    // Check NO_COLOR
    if std::env::var_os("NO_COLOR").is_some() {
        return display.to_string();
    }

    let url = if let Some(ln) = line {
        format!("editor://open?path={}&line={}", absolute_path, ln)
    } else {
        format!("editor://open?path={}", absolute_path)
    };

    // OSC 8 hyperlink: \x1b]8;;URL\x07TEXT\x1b]8;;\x07
    let styled = display
        .if_supports_color(Stream::Stderr, |t| t.dimmed())
        .to_string();

    format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, styled)
}
