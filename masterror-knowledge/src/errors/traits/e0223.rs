// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0223: ambiguous associated type retrieval

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0223",
    title:       LocalizedText::new(
        "Ambiguous associated type retrieval",
        "Неоднозначное получение ассоциированного типа",
        "모호한 연관 타입 조회"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Attempting to retrieve an associated type from a trait directly is
ambiguous because the associated type is only made concrete in
specific implementations of the trait.

Associated types are defined in traits but only have concrete types
when the trait is implemented for a specific struct or type. You
cannot access an associated type directly from the trait itself
without specifying which implementation you want.",
        "\
Попытка получить ассоциированный тип напрямую из трейта неоднозначна,
потому что ассоциированный тип становится конкретным только в
определённых реализациях трейта.

Вы не можете получить доступ к ассоциированному типу напрямую из
трейта без указания конкретной реализации.",
        "\
트레이트에서 연관 타입을 직접 조회하는 것은 모호합니다.
연관 타입은 특정 구현에서만 구체적인 타입을 갖기 때문입니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use fully qualified syntax",
            "Используйте полностью квалифицированный синтаксис",
            "완전 정규화 구문 사용"
        ),
        code:        "let foo: <Struct as Trait>::X;"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Fully Qualified Syntax",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0223.html"
        }
    ]
};
