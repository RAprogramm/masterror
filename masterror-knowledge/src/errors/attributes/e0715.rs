// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0715: marker trait impl overrides associated item

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0715",
    title:       LocalizedText::new(
        "Marker trait impl overrides item",
        "Реализация маркерного трейта переопределяет элемент",
        "마커 트레이트 impl이 항목 재정의"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An `impl` for a `#[marker]` trait tried to override an associated item.

Marker traits are allowed to have multiple implementations for the same type,
so it is not permitted to override anything in those implementations, as it
would be ambiguous which override should actually be used.",
        "\
Реализация `#[marker]` трейта пытается переопределить ассоциированный элемент.

Маркерные трейты могут иметь несколько реализаций для одного типа,
поэтому переопределение запрещено из-за неоднозначности.",
        "\
`#[marker]` 트레이트의 `impl`이 연관 항목을 재정의하려 했습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove override from impl",
                "Удалите переопределение из impl",
                "impl에서 재정의 제거"
            ),
            code:        "#[marker]\ntrait Marker {\n    const N: usize = 0;\n}\n\nimpl Marker for MyType {} // no override"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0715.html"
        }
    ]
};
