// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0276: trait implementation has stricter requirements

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0276",
    title:       LocalizedText::new(
        "Trait implementation has stricter requirements",
        "Реализация трейта имеет более строгие требования",
        "트레이트 구현이 더 엄격한 요구사항을 가짐"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A trait implementation has stricter requirements than the trait definition.

This error occurs when a trait implementation adds additional trait
bounds to a method that aren't present in the original trait definition,
making the implementation stricter than the contract defined by the trait.

The implementation must honor the original trait contract without
introducing additional constraints.",
        "\
Реализация трейта имеет более строгие требования, чем определение трейта.

Ошибка возникает, когда реализация трейта добавляет дополнительные
ограничения трейтов к методу, которых нет в исходном определении трейта.",
        "\
트레이트 구현이 트레이트 정의보다 더 엄격한 요구사항을 가집니다.
구현은 원래 트레이트 계약을 준수해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove extra bounds from implementation",
                "Удалите дополнительные ограничения из реализации",
                "구현에서 추가 바운드 제거"
            ),
            code:        "impl Foo for bool {\n    fn foo<T>(x: T) {} // no extra where clause\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Add bounds to original trait definition",
                "Добавьте ограничения в исходное определение трейта",
                "원래 트레이트 정의에 바운드 추가"
            ),
            code:        "trait Foo {\n    fn foo<T: Copy>(x: T);  // add bound to trait\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Trait Implementations",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0276.html"
        }
    ]
};
