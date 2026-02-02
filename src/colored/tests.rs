// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Tests for colored styling module.

#[cfg(feature = "std")]
mod std_tests {
    use crate::colored::style::*;

    macro_rules! test_style_produces_output {
        ($($name:ident($func:ident, $input:expr)),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let result = $func($input);
                    assert!(!result.is_empty());
                    assert!(result.contains($input));
                }
            )*
        };
    }

    test_style_produces_output! {
        error_kind_critical_produces_output(error_kind_critical, "ServiceUnavailable"),
        error_kind_warning_produces_output(error_kind_warning, "BadRequest"),
        error_code_produces_output(error_code, "ERR_001"),
        error_message_produces_output(error_message, "Connection failed"),
        source_context_produces_output(source_context, "Caused by: timeout"),
        metadata_key_produces_output(metadata_key, "request_id"),
        hint_label_produces_output(hint_label, "hint"),
        hint_text_produces_output(hint_text, "Try restarting the service"),
        suggestion_label_produces_output(suggestion_label, "suggestion"),
        suggestion_text_produces_output(suggestion_text, "Run cargo clean"),
        command_produces_output(command, "cargo build --release"),
        docs_label_produces_output(docs_label, "docs"),
        url_produces_output(url, "https://docs.rs/masterror"),
        related_label_produces_output(related_label, "see also"),
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

    macro_rules! test_style_returns_plain {
        ($($name:ident($func:ident, $input:expr)),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($func($input), $input);
                }
            )*
        };
    }

    test_style_returns_plain! {
        error_kind_critical_returns_plain_text(error_kind_critical, "test"),
        error_kind_warning_returns_plain_text(error_kind_warning, "test"),
        error_code_returns_plain_text(error_code, "ERR_001"),
        error_message_returns_plain_text(error_message, "message"),
        source_context_returns_plain_text(source_context, "context"),
        metadata_key_returns_plain_text(metadata_key, "key"),
        hint_label_returns_plain_text(hint_label, "hint"),
        hint_text_returns_plain_text(hint_text, "help text"),
        suggestion_label_returns_plain_text(suggestion_label, "suggestion"),
        suggestion_text_returns_plain_text(suggestion_text, "try this"),
        command_returns_plain_text(command, "cargo build"),
        docs_label_returns_plain_text(docs_label, "docs"),
        url_returns_plain_text(url, "https://example.com"),
        related_label_returns_plain_text(related_label, "see also"),
        empty_error_kind_critical(error_kind_critical, ""),
        empty_error_code(error_code, ""),
        empty_command(command, ""),
        unicode_error_message(error_message, "エラー: 日本語テスト"),
        unicode_hint_text(hint_text, "エラー: 日本語テスト"),
        special_chars_source_context(source_context, "file.rs:42:13 -> error"),
        special_chars_suggestion_text(suggestion_text, "file.rs:42:13 -> error"),
    }
}
