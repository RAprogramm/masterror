// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0326: associated constant type doesn't match trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0326",
    title:       LocalizedText::new(
        "Associated constant type doesn't match trait",
        "Тип ассоциированной константы не соответствует трейту",
        "연관 상수 타입이 트레이트와 일치하지 않음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
When implementing a trait, any associated constants must have types that
exactly match the types declared in the trait definition. If the type of
an associated constant in the implementation differs from the trait
definition, this error is raised.

Example: trait declares const BAR: bool but implementation uses const BAR: u32.",
        "\
При реализации трейта все ассоциированные константы должны иметь типы,
точно соответствующие типам в определении трейта. Если тип константы
в реализации отличается от определения трейта, возникает эта ошибка.

Пример: трейт объявляет const BAR: bool, а реализация использует const BAR: u32.",
        "\
트레이트를 구현할 때 모든 연관 상수의 타입은 트레이트 정의에 선언된
타입과 정확히 일치해야 합니다. 구현의 연관 상수 타입이 트레이트 정의와
다르면 이 오류가 발생합니다.

예: 트레이트가 const BAR: bool을 선언했지만 구현이 const BAR: u32를 사용함."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Match the type from trait definition",
                "Соответствовать типу из определения трейта",
                "트레이트 정의의 타입과 일치시키기"
            ),
            code:        "trait Foo {\n    const BAR: bool;\n}\n\nimpl Foo for Bar {\n    const BAR: bool = true; // matches trait\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Associated Constants",
            url:   "https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0326.html"
        }
    ]
};
