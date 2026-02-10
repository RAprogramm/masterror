// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0753: inner doc comment in invalid context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0753",
    title:       LocalizedText::new(
        "Inner doc comment in invalid context",
        "Внутренний doc-комментарий в неправильном месте",
        "잘못된 컨텍스트의 내부 문서 주석"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
An inner doc comment (`//!`) was used in an invalid context.

Inner doc comments can only be used before items, typically at the beginning
of a module or crate to document the module itself.",
        "\
Внутренний doc-комментарий (`//!`) использован в неправильном месте.

Такие комментарии можно использовать только перед элементами,
обычно в начале модуля для документирования самого модуля.",
        "\
내부 문서 주석(`//!`)이 잘못된 컨텍스트에서 사용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use outer doc comment for items",
                "Используйте внешний doc-комментарий для элементов",
                "항목에 외부 문서 주석 사용"
            ),
            code:        "/// I am an outer doc comment\nfn foo() {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use inner comment at module start",
                "Используйте внутренний комментарий в начале модуля",
                "모듈 시작에 내부 주석 사용"
            ),
            code:        "//! Module documentation\nfn foo() {}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0753.html"
    }]
};
