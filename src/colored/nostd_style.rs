// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! No-op styling functions for no-std builds.
//!
//! All functions return the input text unchanged, providing API compatibility
//! with the std feature while avoiding any allocations beyond string
//! conversion.

use alloc::string::{String, ToString};

macro_rules! identity_style {
    ($($(#[$meta:meta])* $name:ident),* $(,)?) => {
        $(
            $(#[$meta])*
            #[inline]
            pub fn $name(text: impl AsRef<str>) -> String {
                text.as_ref().to_string()
            }
        )*
    };
}

identity_style! {
    /// Style critical error kind text (no-op in no-std).
    error_kind_critical,
    /// Style warning-level error kind text (no-op in no-std).
    error_kind_warning,
    /// Style error code text (no-op in no-std).
    error_code,
    /// Style error message text (no-op in no-std).
    error_message,
    /// Style source context text (no-op in no-std).
    source_context,
    /// Style metadata key text (no-op in no-std).
    metadata_key,
    /// Style hint label (no-op in no-std).
    hint_label,
    /// Style hint message (no-op in no-std).
    hint_text,
    /// Style suggestion label (no-op in no-std).
    suggestion_label,
    /// Style suggestion message (no-op in no-std).
    suggestion_text,
    /// Style command/code snippet (no-op in no-std).
    command,
    /// Style documentation link label (no-op in no-std).
    docs_label,
    /// Style URL (no-op in no-std).
    url,
    /// Style "see also" label (no-op in no-std).
    related_label,
    /// Style backtrace header label (no-op in no-std).
    backtrace_label,
    /// Style backtrace arrow symbol (no-op in no-std).
    backtrace_arrow,
    /// Style backtrace function name (no-op in no-std).
    backtrace_function,
    /// Style backtrace file location (no-op in no-std).
    backtrace_location,
}

/// Create a clickable hyperlink (no-op in no-std).
#[inline]
pub fn backtrace_link(display: &str, _absolute_path: &str, _line: Option<u32>) -> String {
    display.to_string()
}
