// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0623: lifetime mismatch

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0623",
    title:       LocalizedText::new(
        "Lifetime mismatch",
        "Несоответствие времён жизни",
        "라이프타임 불일치"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
Two lifetimes in your code don't match where they should.",
        "\
Два времени жизни в коде не совпадают там, где должны.",
        "\
코드에서 두 라이프타임이 일치해야 하는 곳에서 일치하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Ensure consistent lifetime annotations",
            "Обеспечить согласованные аннотации",
            "일관된 라이프타임 어노테이션 확보"
        ),
        code:        "fn foo<'a>(x: &'a str) -> &'a str { x }"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0623.html"
    }]
};
