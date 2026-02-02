// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0221: ambiguous associated type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0221",
    title:       LocalizedText::new(
        "Ambiguous associated type",
        "Неоднозначный ассоциированный тип",
        "모호한 연관 타입"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated type is ambiguous due to multiple traits defining
associated types with the same name.

When a trait inherits from another trait and both define an associated
type with the same name, using `Self::A` becomes ambiguous - the
compiler cannot determine which trait's associated type you're
referring to.",
        "\
Ассоциированный тип неоднозначен, так как несколько трейтов определяют
ассоциированные типы с одинаковым именем.

Когда трейт наследует от другого трейта и оба определяют ассоциированный
тип с одинаковым именем, использование `Self::A` становится неоднозначным.",
        "\
여러 트레이트가 같은 이름의 연관 타입을 정의하여 연관 타입이 모호합니다.
`Self::A`를 사용할 때 컴파일러가 어떤 트레이트의 연관 타입인지 결정할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Rename one of the associated types",
                "Переименуйте один из ассоциированных типов",
                "연관 타입 중 하나의 이름 변경"
            ),
            code:        "trait Bar : Foo {\n    type B: T2;  // renamed from A\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use fully qualified syntax",
                "Используйте полностью квалифицированный синтаксис",
                "완전 정규화 구문 사용"
            ),
            code:        "fn do_something() {\n    let _: <Self as Bar>::A;  // explicitly specify Bar's A\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Fully Qualified Syntax",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0221.html"
        }
    ]
};
