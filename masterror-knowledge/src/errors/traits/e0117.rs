// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0117: only traits defined in the current crate can be implemented for
//! arbitrary types

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0117",
    title:       LocalizedText::new(
        "Only traits defined in the current crate can be implemented for arbitrary types",
        "Только трейты из текущего крейта можно реализовать для произвольных типов",
        "현재 크레이트에서 정의된 트레이트만 임의 타입에 구현 가능"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This is a violation of Rust's orphan rules for trait implementations.
You cannot implement a foreign trait (defined in another crate) when:
- The type implementing the trait is foreign (not defined in your crate), AND
- All type parameters passed to the trait are also foreign

This rule ensures coherence - preventing conflicting implementations across crates.",
        "\
Это нарушение правил сирот Rust для реализаций трейтов.
Нельзя реализовать внешний трейт (из другого крейта) когда:
- Тип, реализующий трейт, внешний (не из вашего крейта), И
- Все параметры типа, переданные трейту, тоже внешние

Это правило обеспечивает согласованность и предотвращает конфликты реализаций.",
        "\
이것은 트레이트 구현에 대한 Rust의 고아 규칙 위반입니다.
다음 경우에 외부 트레이트를 구현할 수 없습니다:
- 트레이트를 구현하는 타입이 외부 타입이고
- 트레이트에 전달된 모든 타입 매개변수도 외부인 경우"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement the trait on a local type",
                "Реализовать трейт для локального типа",
                "로컬 타입에 트레이트 구현"
            ),
            code:        "pub struct Foo;\n\nimpl Drop for Foo {\n    fn drop(&mut self) { }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Define a local trait instead",
                "Определить локальный трейт",
                "대신 로컬 트레이트 정의"
            ),
            code:        "trait Bar {\n    fn get(&self) -> usize;\n}\n\nimpl Bar for u32 {\n    fn get(&self) -> usize { 0 }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 1023: Rebalancing Coherence",
            url:   "https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0117.html"
        }
    ]
};
