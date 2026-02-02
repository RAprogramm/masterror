// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0271: type mismatch with associated types

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0271",
    title:       LocalizedText::new(
        "Type mismatch with associated type",
        "Несоответствие типов с ассоциированным типом",
        "연관 타입과 타입 불일치"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A type mismatched an associated type of a trait.

When a trait defines an associated type, implementations must assign
a concrete type. If you constrain a generic parameter with a specific
associated type requirement (e.g., `T: Trait<AssociatedType=u32>`),
the actual type used must satisfy that constraint.",
        "\
Тип не соответствует ассоциированному типу трейта.

Когда трейт определяет ассоциированный тип, реализации должны назначить
конкретный тип. Если вы ограничиваете параметр требованием к
ассоциированному типу, фактический тип должен удовлетворять этому
ограничению.",
        "\
타입이 트레이트의 연관 타입과 일치하지 않습니다.
연관 타입 요구사항을 충족하는 구현을 사용해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Change constraint to match implementation",
                "Измените ограничение, чтобы соответствовать реализации",
                "구현과 일치하도록 제약 변경"
            ),
            code:        "fn foo<T>(t: T) where T: Trait<AssociatedType = &'static str> { }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Change implementation to match constraint",
                "Измените реализацию, чтобы соответствовать ограничению",
                "제약과 일치하도록 구현 변경"
            ),
            code:        "impl Trait for i8 { type AssociatedType = u32; }"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Associated Types",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0271.html"
        }
    ]
};
