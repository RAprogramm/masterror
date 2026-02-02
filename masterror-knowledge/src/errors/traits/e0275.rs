// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0275: trait requirement overflow

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0275",
    title:       LocalizedText::new(
        "Trait requirement overflow",
        "Переполнение требований трейта",
        "트레이트 요구사항 오버플로우"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An evaluation of a trait requirement overflows due to infinite recursion.

The error happens when there is an unbounded recursion in resolving
type bounds that creates a circular dependency, causing the trait
resolution process to loop infinitely.

Example: impl<T> Foo for T where Bar<T>: Foo
- To check if T implements Foo, compiler checks if Bar<T> implements Foo
- To check Bar<T>, it checks if Bar<Bar<T>> implements Foo
- This continues infinitely...",
        "\
Оценка требования трейта переполняется из-за бесконечной рекурсии.

Ошибка возникает при неограниченной рекурсии в разрешении ограничений
типов, создающей циклическую зависимость.",
        "\
트레이트 요구사항 평가가 무한 재귀로 인해 오버플로우됩니다.
타입 바운드 해결에서 순환 종속성이 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove self-referential trait bounds",
            "Удалите самоссылающиеся ограничения трейтов",
            "자기 참조 트레이트 바운드 제거"
        ),
        code:        "trait Foo {}\n\nimpl Foo for i32 {}  // concrete implementation instead"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Trait Bounds",
            url:   "https://doc.rust-lang.org/reference/trait-bounds.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0275.html"
        }
    ]
};
