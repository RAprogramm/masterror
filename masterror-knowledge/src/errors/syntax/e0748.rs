// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0748: raw string not terminated

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0748",
    title:       LocalizedText::new(
        "Raw string not correctly terminated",
        "Сырая строка неправильно завершена",
        "raw 문자열이 올바르게 종료되지 않음"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A raw string isn't correctly terminated because the trailing `#` count
doesn't match its leading `#` count.

The number of hash symbols (`#`) at the end must exactly match the number
at the beginning.",
        "\
Сырая строка неправильно завершена, так как количество `#` в конце
не соответствует количеству в начале.",
        "\
raw 문자열이 올바르게 종료되지 않았습니다. 끝의 `#` 개수가
시작의 `#` 개수와 일치해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match trailing hash count",
            "Выровняйте количество # в конце",
            "끝의 해시 개수 맞추기"
        ),
        code:        "let s = r#\"Hello\"#; // one # at start and end"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0748.html"
    }]
};
