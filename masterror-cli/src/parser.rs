// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Cargo JSON output parser.

use serde::Deserialize;

/// Top-level cargo message.
#[derive(Deserialize)]
pub struct CargoMessage {
    pub reason:   String,
    pub message:  Option<DiagnosticMessage>,
    /// Full rendered compiler output.
    pub rendered: Option<String>
}

/// Compiler diagnostic message.
#[derive(Deserialize)]
pub struct DiagnosticMessage {
    pub level:    String,
    pub message:  String,
    pub code:     Option<DiagnosticCode>,
    pub rendered: Option<String>
}

/// Error code info.
#[derive(Deserialize)]
pub struct DiagnosticCode {
    pub code: String
}

impl CargoMessage {
    /// Check if this is a compiler error message.
    pub fn is_error(&self) -> bool {
        self.reason == "compiler-message"
            && self.message.as_ref().is_some_and(|m| m.level == "error")
    }

    /// Get the error code if present.
    pub fn error_code(&self) -> Option<&str> {
        self.message
            .as_ref()
            .and_then(|m| m.code.as_ref())
            .map(|c| c.code.as_str())
    }

    /// Get the error message.
    pub fn error_message(&self) -> Option<&str> {
        self.message.as_ref().map(|m| m.message.as_str())
    }

    /// Get rendered output (from message or top-level).
    pub fn rendered_output(&self) -> Option<&str> {
        self.message
            .as_ref()
            .and_then(|m| m.rendered.as_deref())
            .or(self.rendered.as_deref())
    }
}
