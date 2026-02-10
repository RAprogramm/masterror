// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0200: unsafe trait was implemented without an unsafe impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0200",
    title:       LocalizedText::new(
        "Unsafe trait was implemented without an unsafe impl",
        "Unsafe трейт реализован без unsafe impl",
        "unsafe 트레이트가 unsafe impl 없이 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An unsafe trait was implemented without an unsafe implementation. Unsafe
traits must have unsafe implementations. The unsafe keyword is required
to indicate that the implementation upholds the safety contract of the
unsafe trait.",
        "\
Unsafe трейт реализован без unsafe реализации. Unsafe трейты должны
иметь unsafe реализации. Ключевое слово unsafe требуется, чтобы
указать, что реализация соблюдает контракт безопасности unsafe трейта.",
        "\
unsafe 트레이트가 unsafe 구현 없이 구현되었습니다. unsafe 트레이트는
unsafe 구현을 가져야 합니다. unsafe 키워드는 구현이 unsafe 트레이트의
안전성 계약을 준수함을 나타내기 위해 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add unsafe keyword to trait impl",
            "Добавить unsafe к реализации трейта",
            "트레이트 impl에 unsafe 키워드 추가"
        ),
        code:        "struct Foo;\n\nunsafe trait Bar { }\n\nunsafe impl Bar for Foo { } // ok! unsafe impl"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Unsafe Traits",
            url:   "https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0200.html"
        }
    ]
};
