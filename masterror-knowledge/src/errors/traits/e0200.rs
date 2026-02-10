// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0200: unsafe trait implemented without unsafe impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0200",
    title:       LocalizedText::new(
        "Unsafe trait implemented without unsafe impl",
        "Небезопасный трейт реализован без unsafe impl",
        "unsafe impl 없이 unsafe 트레이트 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An unsafe trait was implemented without an unsafe implementation block.
Rust requires that any implementation of an unsafe trait must also be
declared as `unsafe` to indicate that the implementer understands and
accepts the safety responsibilities.

When implementing an unsafe trait, you must explicitly mark the
implementation with the `unsafe` keyword to acknowledge you're
implementing code that may have safety implications.",
        "\
Небезопасный трейт был реализован без блока unsafe impl.
Rust требует, чтобы любая реализация небезопасного трейта также была
объявлена как `unsafe`, чтобы показать, что разработчик понимает
и принимает ответственность за безопасность.",
        "\
unsafe 트레이트가 unsafe impl 블록 없이 구현되었습니다.
Rust는 unsafe 트레이트의 모든 구현이 `unsafe`로 선언되어야
안전 책임을 이해하고 수용함을 나타냅니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add unsafe keyword to impl block",
            "Добавить ключевое слово unsafe к блоку impl",
            "impl 블록에 unsafe 키워드 추가"
        ),
        code:        "unsafe impl Bar for Foo { }"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Unsafe Traits",
            url:   "https://doc.rust-lang.org/reference/items/traits.html#unsafe-traits"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0200.html"
        }
    ]
};
