// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0635: unknown feature

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0635",
    title:       LocalizedText::new(
        "Unknown feature in #![feature] attribute",
        "Неизвестная функция в атрибуте #![feature]",
        "#![feature] 속성에 알 수 없는 기능"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unknown or non-existent feature was specified in a `#![feature]` attribute.

Feature flags are used to opt into unstable or experimental Rust features,
and they must be valid feature names recognized by the Rust compiler.",
        "\
Неизвестная или несуществующая функция была указана в атрибуте `#![feature]`.

Флаги функций используются для включения нестабильных или экспериментальных
функций Rust и должны быть допустимыми именами функций, распознаваемыми
компилятором Rust.",
        "\
`#![feature]` 속성에 알 수 없거나 존재하지 않는 기능이 지정되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Verify the feature name spelling",
                "Проверить правописание имени функции",
                "기능 이름 철자 확인"
            ),
            code:        "#![feature(existing_feature)] // check spelling"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Check the Unstable Book for valid features",
                "Проверить Unstable Book для допустимых функций",
                "유효한 기능은 Unstable Book 확인"
            ),
            code:        "// See https://doc.rust-lang.org/unstable-book/"
        }
    ],
    links:       &[
        DocLink {
            title: "The Unstable Book",
            url:   "https://doc.rust-lang.org/unstable-book/"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0635.html"
        }
    ]
};
