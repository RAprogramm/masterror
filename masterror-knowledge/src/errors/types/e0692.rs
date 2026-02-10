// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0692: incompatible representation hints

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0692",
    title:       LocalizedText::new(
        "Incompatible representation hints",
        "Несовместимые подсказки представления",
        "호환되지 않는 표현 힌트"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type is annotated with `repr(transparent)` along with other conflicting
representation hints.

This is contradictory because `repr(transparent)` delegates all representation
concerns to another type, so adding additional hints like `repr(C)` conflicts
with this purpose.",
        "\
Тип аннотирован `repr(transparent)` вместе с другими конфликтующими
подсказками представления.

Это противоречиво, поскольку `repr(transparent)` делегирует все вопросы
представления другому типу, поэтому добавление дополнительных подсказок
вроде `repr(C)` конфликтует с этой целью.",
        "\
타입이 `repr(transparent)`와 함께 다른 충돌하는 표현 힌트로
주석이 달려 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove conflicting hints",
                "Удалить конфликтующие подсказки",
                "충돌하는 힌트 제거"
            ),
            code:        "#[repr(transparent)]\nstruct Grams(f32);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Move other attributes to contained type",
                "Переместить другие атрибуты на содержащийся тип",
                "다른 속성을 포함된 타입으로 이동"
            ),
            code:        "#[repr(C)]\nstruct Foo { x: i32 }\n\n#[repr(transparent)]\nstruct FooWrapper(Foo);"
        }
    ],
    links:       &[
        DocLink {
            title: "repr(transparent)",
            url:   "https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0692.html"
        }
    ]
};
