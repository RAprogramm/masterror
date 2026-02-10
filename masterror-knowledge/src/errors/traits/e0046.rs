// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0046: missing trait implementation items

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0046",
    title:       LocalizedText::new(
        "Missing trait implementation items",
        "Отсутствуют элементы реализации трейта",
        "트레이트 구현 항목 누락"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This error occurs when implementing a trait but failing to provide
implementations for all required items (methods, types, constants).

Example:
    trait Foo { fn foo(&self); }
    impl Foo for Bar {}  // Error: missing method `foo`",
        "\
Эта ошибка возникает при реализации трейта без предоставления реализаций
для всех обязательных элементов (методов, типов, констант).",
        "\
이 오류는 트레이트를 구현할 때 필수 항목의 구현을 제공하지 않으면 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Implement all required trait methods",
            "Реализовать все обязательные методы трейта",
            "모든 필수 트레이트 메서드 구현"
        ),
        code:        "impl Foo for Bar {\n    fn foo(&self) {\n        // implementation\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Implementing Traits",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0046.html"
        }
    ]
};
