// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0025: field bound multiple times in pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0025",
    title:       LocalizedText::new(
        "Field bound multiple times in pattern",
        "Поле связано несколько раз в паттерне",
        "패턴에서 필드가 여러 번 바인딩됨"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
Each field of a struct can only be bound once in a pattern. This error occurs
when you bind the same field multiple times.

Example:
    struct Foo { a: u8, b: u8 }
    let Foo { a: x, a: y } = foo;  // Error: field `a` bound twice",
        "\
Каждое поле структуры может быть связано только один раз в паттерне.",
        "\
구조체의 각 필드는 패턴에서 한 번만 바인딩할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Bind each field only once",
            "Связать каждое поле только один раз",
            "각 필드를 한 번만 바인딩"
        ),
        code:        "let Foo { a: x, b: y } = foo;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0025.html"
    }]
};
