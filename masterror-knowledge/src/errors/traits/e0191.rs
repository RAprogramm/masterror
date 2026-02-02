// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0191: associated type wasn't specified for a trait object

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0191",
    title:       LocalizedText::new(
        "Associated type wasn't specified for a trait object",
        "Ассоциированный тип не указан для трейт-объекта",
        "트레이트 객체에 대한 연관 타입이 지정되지 않음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
You attempted to create a trait object without specifying all required
associated types. Trait objects must have all their associated types
explicitly defined.",
        "\
Вы попытались создать трейт-объект без указания всех необходимых
ассоциированных типов. Трейт-объекты должны иметь все свои
ассоциированные типы явно определёнными.",
        "\
필요한 모든 연관 타입을 지정하지 않고 트레이트 객체를 생성하려고
했습니다. 트레이트 객체는 모든 연관 타입이 명시적으로 정의되어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Specify all associated types",
            "Указать все ассоциированные типы",
            "모든 연관 타입 지정"
        ),
        code:        "trait Trait {\n    type Bar;\n}\n\ntype Foo = dyn Trait<Bar=i32>; // specify associated type"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Associated Types",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0191.html"
        }
    ]
};
