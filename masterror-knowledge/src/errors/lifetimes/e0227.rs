// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0227: ambiguous lifetime bounds

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0227",
    title:       LocalizedText::new(
        "Ambiguous lifetime bounds",
        "Неоднозначные ограничения времени жизни",
        "모호한 수명 바운드"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
Unable to determine a unique region in derived region bounds.

This error occurs when the Rust compiler cannot determine exactly one
unique lifetime from a set of derived region bounds. When a trait
object could satisfy multiple lifetime constraints, you need to be
explicit about which lifetime applies.",
        "\
Невозможно определить единственный регион из производных ограничений регионов.

Эта ошибка возникает, когда компилятор Rust не может определить ровно
одно уникальное время жизни из набора производных ограничений регионов.",
        "\
파생된 영역 바운드에서 고유한 영역을 결정할 수 없습니다.
명시적으로 어떤 수명이 적용되는지 지정해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Explicitly specify a lifetime bound",
            "Явно укажите ограничение времени жизни",
            "수명 바운드를 명시적으로 지정"
        ),
        code:        "struct Baz<'foo, 'bar, 'baz>\nwhere\n    'baz: 'foo + 'bar,\n{\n    obj: dyn FooBar<'foo, 'bar> + 'baz,\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Lifetimes",
            url:   "https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0227.html"
        }
    ]
};
