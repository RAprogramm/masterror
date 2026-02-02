// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0199: implementing trait was marked as unsafe while the trait is safe

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0199",
    title:       LocalizedText::new(
        "Implementing trait was marked as unsafe while the trait is safe",
        "Реализация трейта помечена как unsafe, хотя трейт безопасен",
        "트레이트가 안전한데 구현이 unsafe로 표시됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A trait implementation was marked as unsafe while the trait is safe.
Safe traits should not have unsafe implementations. The unsafe keyword
should only be used when implementing unsafe traits.

Only unsafe traits can have unsafe implementations.",
        "\
Реализация трейта помечена как unsafe, хотя трейт безопасен.
Безопасные трейты не должны иметь unsafe реализации. Ключевое слово
unsafe должно использоваться только при реализации unsafe трейтов.

Только unsafe трейты могут иметь unsafe реализации.",
        "\
트레이트가 안전한데 트레이트 구현이 unsafe로 표시되었습니다.
안전한 트레이트는 unsafe 구현을 가지면 안 됩니다. unsafe 키워드는
unsafe 트레이트를 구현할 때만 사용해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove the unsafe keyword from trait impl",
            "Удалить unsafe из реализации трейта",
            "트레이트 impl에서 unsafe 키워드 제거"
        ),
        code:        "struct Foo;\n\ntrait Bar { }\n\nimpl Bar for Foo { } // ok! no unsafe"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Unsafe Traits",
            url:   "https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0199.html"
        }
    ]
};
