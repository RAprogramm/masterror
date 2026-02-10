// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0220: associated type not found in trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0220",
    title:       LocalizedText::new(
        "Associated type not found in trait",
        "Ассоциированный тип не найден в трейте",
        "트레이트에서 연관 타입을 찾을 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
The associated type used was not defined in the trait.

Common causes:
1. Using an associated type that doesn't exist
2. Misspelling the associated type name
3. Using the wrong trait

Ensure that any associated type you use is properly declared in
the trait body using the `type` keyword.",
        "\
Используемый ассоциированный тип не был определён в трейте.

Частые причины:
1. Использование несуществующего ассоциированного типа
2. Опечатка в имени ассоциированного типа
3. Использование неправильного трейта",
        "\
사용된 연관 타입이 트레이트에 정의되지 않았습니다.
연관 타입이 트레이트 본문에 `type` 키워드로 선언되었는지 확인하세요."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use correct associated type name",
                "Используйте правильное имя ассоциированного типа",
                "올바른 연관 타입 이름 사용"
            ),
            code:        "trait T1 {\n    type Bar;\n}\n\ntype Foo = T1<Bar=i32>; // use Bar, not F"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Declare the associated type in trait",
                "Объявите ассоциированный тип в трейте",
                "트레이트에 연관 타입 선언"
            ),
            code:        "trait T2 {\n    type Bar;\n    type Baz; // declare it\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Associated Types",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0220.html"
        }
    ]
};
