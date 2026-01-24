// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! No-op styling functions for no-std builds.
//!
//! All functions return the input text unchanged, providing API compatibility
//! with the std feature while avoiding any allocations beyond string
//! conversion.

use alloc::string::{String, ToString};

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

/// Style hint label.
pub fn hint_label(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style hint message.
pub fn hint_text(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style suggestion label.
pub fn suggestion_label(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style suggestion message.
pub fn suggestion_text(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style command/code snippet.
pub fn command(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style documentation link label.
pub fn docs_label(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style URL.
pub fn url(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}

/// Style "see also" label.
pub fn related_label(text: impl AsRef<str>) -> String {
    text.as_ref().to_string()
}
