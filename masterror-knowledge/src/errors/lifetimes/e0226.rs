// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0226: multiple explicit lifetime bounds on trait object

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0226",
    title:       LocalizedText::new(
        "Multiple explicit lifetime bounds on trait object",
        "Несколько явных ограничений времени жизни для трейт-объекта",
        "트레이트 객체에 여러 명시적 수명 바운드"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
More than one explicit lifetime bound was used on a trait object.
Trait objects in Rust can only have ONE explicit lifetime bound.

If you need to work with multiple lifetimes, consider restructuring
your code or using a single lifetime that encompasses the requirements
of both.",
        "\
Для трейт-объекта было использовано более одного явного ограничения
времени жизни. Трейт-объекты в Rust могут иметь только ОДНО явное
ограничение времени жизни.",
        "\
트레이트 객체에 둘 이상의 명시적 수명 바운드가 사용되었습니다.
트레이트 객체는 하나의 명시적 수명 바운드만 가질 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove all but one lifetime bound",
            "Удалите все ограничения времени жизни, кроме одного",
            "하나를 제외한 모든 수명 바운드 제거"
        ),
        code:        "trait Foo {}\n\ntype T<'a> = dyn Foo + 'a;"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Trait Object Lifetime Bounds",
            url:   "https://doc.rust-lang.org/reference/types/trait-object.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0226.html"
        }
    ]
};
