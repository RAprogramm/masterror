// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0437: associated type not in trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0437",
    title:       LocalizedText::new(
        "Associated type not in trait",
        "Ассоциированный тип не в трейте",
        "연관 타입이 트레이트에 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated type was defined in a trait implementation that doesn't
exist in the original trait definition. When implementing a trait, you
can only define associated types that are explicitly declared in the trait.",
        "\
В реализации трейта определён ассоциированный тип, которого нет в
исходном определении трейта. При реализации трейта можно определять
только те ассоциированные типы, которые объявлены в трейте.",
        "\
트레이트 구현에서 원래 트레이트 정의에 존재하지 않는 연관 타입이
정의되었습니다. 트레이트에 선언된 연관 타입만 정의할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the extraneous associated type",
                "Удалить лишний ассоциированный тип",
                "불필요한 연관 타입 제거"
            ),
            code:        "trait Foo {}\n\nimpl Foo for i32 {} // Remove type Bar = bool;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Add associated type to trait definition",
                "Добавить ассоциированный тип в определение трейта",
                "트레이트 정의에 연관 타입 추가"
            ),
            code:        "trait Foo {\n    type Bar;\n}\n\nimpl Foo for i32 {\n    type Bar = bool;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0437.html"
    }]
};
