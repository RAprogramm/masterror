// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0197: inherent implementation was marked unsafe

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0197",
    title:       LocalizedText::new(
        "Inherent implementation was marked unsafe",
        "Собственная реализация помечена как unsafe",
        "고유 구현이 unsafe로 표시됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An inherent implementation was marked unsafe. Inherent implementations
(implementations that do not implement a trait but provide methods
associated with a type) are always safe because they are not implementing
an unsafe trait.

Remove the unsafe keyword from the inherent implementation.",
        "\
Собственная реализация помечена как unsafe. Собственные реализации
(реализации, которые не реализуют трейт, но предоставляют методы,
связанные с типом) всегда безопасны, потому что они не реализуют
небезопасный трейт.

Удалите ключевое слово unsafe из собственной реализации.",
        "\
고유 구현이 unsafe로 표시되었습니다. 고유 구현(트레이트를 구현하지
않지만 타입과 연관된 메서드를 제공하는 구현)은 unsafe 트레이트를
구현하지 않기 때문에 항상 안전합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove the unsafe keyword",
            "Удалить ключевое слово unsafe",
            "unsafe 키워드 제거"
        ),
        code:        "struct Foo;\n\nimpl Foo { } // ok! no unsafe"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Implementations",
            url:   "https://doc.rust-lang.org/reference/items/implementations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0197.html"
        }
    ]
};
