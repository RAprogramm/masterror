// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0763: unterminated byte literal

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0763",
    title:       LocalizedText::new(
        "Unterminated byte literal",
        "Незавершённый байтовый литерал",
        "종료되지 않은 바이트 리터럴"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A byte constant wasn't correctly ended.

Byte literals (prefixed with `b`) must be properly terminated with a
closing quote.",
        "\
Байтовый литерал не завершён правильно.

Байтовые литералы (с префиксом `b`) должны заканчиваться кавычкой.",
        "\
바이트 상수가 올바르게 종료되지 않았습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add closing quote",
            "Добавьте закрывающую кавычку",
            "닫는 따옴표 추가"
        ),
        code:        "let c = b'a'; // closed properly"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0763.html"
    }]
};
