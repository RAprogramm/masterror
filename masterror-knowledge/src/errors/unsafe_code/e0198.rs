// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0198: negative implementation was marked as unsafe

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0198",
    title:       LocalizedText::new(
        "Negative implementation was marked as unsafe",
        "Негативная реализация помечена как unsafe",
        "부정 구현이 unsafe로 표시됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A negative trait implementation is incorrectly marked as unsafe. Negative
implementations exclude a type from implementing a particular trait. Since
not being able to use a trait is always a safe operation, negative
implementations are inherently safe and should never be marked as unsafe.

Negative implementations are only allowed for auto traits.",
        "\
Негативная реализация трейта неправильно помечена как unsafe. Негативные
реализации исключают тип из реализации конкретного трейта. Поскольку
невозможность использовать трейт - это всегда безопасная операция,
негативные реализации по своей сути безопасны и не должны быть помечены как unsafe.

Негативные реализации разрешены только для auto трейтов.",
        "\
부정 트레이트 구현이 잘못 unsafe로 표시되었습니다. 부정 구현은 특정
트레이트를 구현하지 못하도록 타입을 제외합니다. 트레이트를 사용할 수
없다는 것은 항상 안전한 작업이므로 부정 구현은 본질적으로 안전하며
unsafe로 표시하면 안 됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove the unsafe keyword from negative impl",
            "Удалить unsafe из негативной реализации",
            "부정 impl에서 unsafe 키워드 제거"
        ),
        code:        "#![feature(auto_traits)]\n\nstruct Foo;\n\nauto trait Enterprise {}\n\nimpl !Enterprise for Foo { } // ok! no unsafe"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Auto Traits",
            url:   "https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0198.html"
        }
    ]
};
