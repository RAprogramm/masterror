// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0023: wrong number of fields in pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0023",
    title:       LocalizedText::new(
        "Wrong number of fields in pattern",
        "Неверное количество полей в паттерне",
        "패턴에서 잘못된 필드 수"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when a pattern provides the wrong number of sub-patterns for
an enum variant's fields.

Example:
    enum Fruit { Apple(String, String) }
    match fruit {
        Fruit::Apple(a) => {},  // Error: Apple has 2 fields, not 1
    }

Each enum variant has a specific number of fields, and your pattern must
provide exactly that many sub-patterns.",
        "\
Эта ошибка возникает, когда паттерн содержит неверное количество подпаттернов
для полей варианта enum.",
        "\
이 오류는 패턴이 열거형 변형의 필드에 대해 잘못된 수의 하위 패턴을 제공할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match the exact number of fields",
            "Указать точное количество полей",
            "정확한 필드 수와 일치"
        ),
        code:        "match fruit {\n    Fruit::Apple(a, b) => {},\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Patterns",
            url:   "https://doc.rust-lang.org/book/ch18-00-patterns.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0023.html"
        }
    ]
};
