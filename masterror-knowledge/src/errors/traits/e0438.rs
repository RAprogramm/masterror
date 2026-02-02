// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0438: associated constant not in trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0438",
    title:       LocalizedText::new(
        "Associated constant not in trait",
        "Ассоциированная константа не в трейте",
        "연관 상수가 트레이트에 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated constant was defined in a trait implementation that doesn't
exist in the trait definition. When implementing a trait, you can only
define associated constants that are explicitly declared in the trait.",
        "\
В реализации трейта определена ассоциированная константа, которой нет
в определении трейта. При реализации трейта можно определять только
те константы, которые объявлены в трейте.",
        "\
트레이트 구현에서 트레이트 정의에 존재하지 않는 연관 상수가
정의되었습니다. 트레이트에 선언된 연관 상수만 정의할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the extraneous associated constant",
                "Удалить лишнюю ассоциированную константу",
                "불필요한 연관 상수 제거"
            ),
            code:        "trait Foo {}\n\nimpl Foo for i32 {} // Remove const BAR: bool = true;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Add associated constant to trait definition",
                "Добавить ассоциированную константу в определение трейта",
                "트레이트 정의에 연관 상수 추가"
            ),
            code:        "trait Foo {\n    const BAR: bool;\n}\n\nimpl Foo for i32 {\n    const BAR: bool = true;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0438.html"
    }]
};
