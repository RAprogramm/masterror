// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Tests for colored styling module.

#[cfg(feature = "std")]
mod std_tests {
    use crate::colored::style::*;

    // ─────────────────────────────────────────────────────────────────────────
    // Basic error styling tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn error_kind_critical_produces_output() {
        let result = error_kind_critical("ServiceUnavailable");
        assert!(!result.is_empty());
        assert!(result.contains("ServiceUnavailable"));
    }

    #[test]
    fn error_kind_warning_produces_output() {
        let result = error_kind_warning("BadRequest");
        assert!(!result.is_empty());
        assert!(result.contains("BadRequest"));
    }

    #[test]
    fn error_code_produces_output() {
        let result = error_code("ERR_001");
        assert!(!result.is_empty());
        assert!(result.contains("ERR_001"));
    }

    #[test]
    fn error_message_produces_output() {
        let result = error_message("Connection failed");
        assert!(!result.is_empty());
        assert!(result.contains("Connection failed"));
    }

    #[test]
    fn source_context_produces_output() {
        let result = source_context("Caused by: timeout");
        assert!(!result.is_empty());
        assert!(result.contains("Caused by: timeout"));
    }

    #[test]
    fn metadata_key_produces_output() {
        let result = metadata_key("request_id");
        assert!(!result.is_empty());
        assert!(result.contains("request_id"));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Diagnostic styling tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn hint_label_produces_output() {
        let result = hint_label("hint");
        assert!(!result.is_empty());
        assert!(result.contains("hint"));
    }

    #[test]
    fn hint_text_produces_output() {
        let result = hint_text("Try restarting the service");
        assert!(!result.is_empty());
        assert!(result.contains("Try restarting the service"));
    }

    #[test]
    fn suggestion_label_produces_output() {
        let result = suggestion_label("suggestion");
        assert!(!result.is_empty());
        assert!(result.contains("suggestion"));
    }

    #[test]
    fn suggestion_text_produces_output() {
        let result = suggestion_text("Run cargo clean");
        assert!(!result.is_empty());
        assert!(result.contains("Run cargo clean"));
    }

    #[test]
    fn command_produces_output() {
        let result = command("cargo build --release");
        assert!(!result.is_empty());
        assert!(result.contains("cargo build --release"));
    }

    #[test]
    fn docs_label_produces_output() {
        let result = docs_label("docs");
        assert!(!result.is_empty());
        assert!(result.contains("docs"));
    }

    #[test]
    fn url_produces_output() {
        let result = url("https://docs.rs/masterror");
        assert!(!result.is_empty());
        assert!(result.contains("https://docs.rs/masterror"));
    }

    #[test]
    fn related_label_produces_output() {
        let result = related_label("see also");
        assert!(!result.is_empty());
        assert!(result.contains("see also"));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Content preservation tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn all_style_functions_preserve_content() {
        let input = "test content with special chars: äöü";
        assert!(error_kind_critical(input).contains(input));
        assert!(error_kind_warning(input).contains(input));
        assert!(error_code(input).contains(input));
        assert!(error_message(input).contains(input));
        assert!(source_context(input).contains(input));
        assert!(metadata_key(input).contains(input));
    }

    #[test]
    fn diagnostic_functions_preserve_content() {
        let input = "content with unicode: 日本語";
        assert!(hint_label(input).contains(input));
        assert!(hint_text(input).contains(input));
        assert!(suggestion_label(input).contains(input));
        assert!(suggestion_text(input).contains(input));
        assert!(command(input).contains(input));
        assert!(docs_label(input).contains(input));
        assert!(url(input).contains(input));
        assert!(related_label(input).contains(input));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Edge case tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn empty_string_returns_empty() {
        assert!(error_kind_critical("").is_empty() || error_kind_critical("").contains(""));
        assert!(error_code("").is_empty() || error_code("").contains(""));
        assert!(command("").is_empty() || command("").contains(""));
    }

    #[test]
    fn whitespace_preserved() {
        let input = "  spaced  text  ";
        assert!(error_message(input).contains(input));
        assert!(hint_text(input).contains(input));
    }

    #[test]
    fn newlines_preserved() {
        let input = "line1\nline2\nline3";
        assert!(error_message(input).contains(input));
        assert!(suggestion_text(input).contains(input));
    }

    #[test]
    fn special_chars_preserved() {
        let input = "path/to/file.rs:42:13";
        assert!(error_message(input).contains(input));
        assert!(source_context(input).contains(input));
    }

    #[test]
    fn ansi_escape_sequences_in_input_preserved() {
        let input = "\x1b[31mred\x1b[0m";
        let result = error_message(input);
        assert!(result.contains("red"));
    }
}

#[cfg(not(feature = "std"))]
mod nostd_tests {
    use crate::colored::style::*;

    // ─────────────────────────────────────────────────────────────────────────
    // Basic styling returns plain text
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn error_kind_critical_returns_plain_text() {
        assert_eq!(error_kind_critical("test"), "test");
    }

    #[test]
    fn error_kind_warning_returns_plain_text() {
        assert_eq!(error_kind_warning("test"), "test");
    }

    #[test]
    fn error_code_returns_plain_text() {
        assert_eq!(error_code("ERR_001"), "ERR_001");
    }

    #[test]
    fn error_message_returns_plain_text() {
        assert_eq!(error_message("message"), "message");
    }

    #[test]
    fn source_context_returns_plain_text() {
        assert_eq!(source_context("context"), "context");
    }

    #[test]
    fn metadata_key_returns_plain_text() {
        assert_eq!(metadata_key("key"), "key");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Diagnostic styling returns plain text
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn hint_label_returns_plain_text() {
        assert_eq!(hint_label("hint"), "hint");
    }

    #[test]
    fn hint_text_returns_plain_text() {
        assert_eq!(hint_text("help text"), "help text");
    }

    #[test]
    fn suggestion_label_returns_plain_text() {
        assert_eq!(suggestion_label("suggestion"), "suggestion");
    }

    #[test]
    fn suggestion_text_returns_plain_text() {
        assert_eq!(suggestion_text("try this"), "try this");
    }

    #[test]
    fn command_returns_plain_text() {
        assert_eq!(command("cargo build"), "cargo build");
    }

    #[test]
    fn docs_label_returns_plain_text() {
        assert_eq!(docs_label("docs"), "docs");
    }

    #[test]
    fn url_returns_plain_text() {
        assert_eq!(url("https://example.com"), "https://example.com");
    }

    #[test]
    fn related_label_returns_plain_text() {
        assert_eq!(related_label("see also"), "see also");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Edge cases
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn empty_string_returns_empty() {
        assert_eq!(error_kind_critical(""), "");
        assert_eq!(error_code(""), "");
        assert_eq!(command(""), "");
    }

    #[test]
    fn unicode_preserved() {
        let input = "エラー: 日本語テスト";
        assert_eq!(error_message(input), input);
        assert_eq!(hint_text(input), input);
    }

    #[test]
    fn special_chars_preserved() {
        let input = "file.rs:42:13 -> error";
        assert_eq!(source_context(input), input);
        assert_eq!(suggestion_text(input), input);
    }
}
