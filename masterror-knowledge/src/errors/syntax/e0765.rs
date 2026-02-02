// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0765: unterminated string

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0765",
    title:       LocalizedText::new(
        "Unterminated string",
        "Незавершённая строка",
        "종료되지 않은 문자열"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A double quote string was not terminated.

String literals must be closed with a matching double quote.",
        "\
Строка в двойных кавычках не завершена.

Строковые литералы должны заканчиваться двойной кавычкой.",
        "\
큰따옴표 문자열이 종료되지 않았습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add closing double quote",
            "Добавьте закрывающую двойную кавычку",
            "닫는 큰따옴표 추가"
        ),
        code:        "let s = \"hello\"; // closed properly"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0765.html"
    }]
};
