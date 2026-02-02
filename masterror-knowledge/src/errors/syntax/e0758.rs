// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0758: unterminated multi-line comment

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0758",
    title:       LocalizedText::new(
        "Unterminated multi-line comment",
        "Незавершённый многострочный комментарий",
        "종료되지 않은 여러 줄 주석"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A multi-line comment (either regular `/* */` or doc comment `/*! */`)
is unterminated.

Multi-line comments must be properly closed with `*/`.",
        "\
Многострочный комментарий (обычный `/* */` или doc `/*! */`)
не завершён.

Многострочные комментарии должны заканчиваться на `*/`.",
        "\
여러 줄 주석(`/* */` 또는 문서 주석 `/*! */`)이 종료되지 않았습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Close the comment with */",
            "Закройте комментарий с помощью */",
            "*/로 주석 닫기"
        ),
        code:        "/* This is a\n   multi-line comment */"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0758.html"
    }]
};
