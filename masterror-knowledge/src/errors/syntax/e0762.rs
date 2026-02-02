// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0762: unterminated character literal

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0762",
    title:       LocalizedText::new(
        "Unterminated character literal",
        "Незавершённый символьный литерал",
        "종료되지 않은 문자 리터럴"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A character literal wasn't ended with a quote.

Character literals must be enclosed in single quotes.",
        "\
Символьный литерал не завершён кавычкой.

Символьные литералы должны быть заключены в одинарные кавычки.",
        "\
문자 리터럴이 따옴표로 끝나지 않았습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add closing quote",
            "Добавьте закрывающую кавычку",
            "닫는 따옴표 추가"
        ),
        code:        "static C: char = 'a'; // closed properly"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0762.html"
    }]
};
