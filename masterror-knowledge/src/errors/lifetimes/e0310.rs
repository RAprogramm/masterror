// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0310: parameter type may not live long enough (requires 'static)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0310",
    title:       LocalizedText::new(
        "Parameter type may not live long enough",
        "Параметр типа может не жить достаточно долго",
        "매개변수 타입이 충분히 오래 살지 않을 수 있음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
Type parameters in type definitions have lifetimes associated with them that
represent how long the data stored within them is guaranteed to live. When a
type parameter is used with a reference that requires a specific lifetime
(like 'static), the type parameter must be constrained to meet that requirement.

Example: struct Foo<T> { foo: &'static T } fails because T is not constrained
to the 'static lifetime that the reference requires.",
        "\
Параметры типов имеют связанные с ними времена жизни, которые представляют,
как долго данные гарантированно существуют. Когда параметр типа используется
со ссылкой, требующей определённого времени жизни (например 'static),
параметр типа должен быть ограничен для соответствия этому требованию.",
        "\
타입 정의의 타입 매개변수에는 저장된 데이터가 얼마나 오래 유효한지를 나타내는
라이프타임이 연결되어 있습니다. 타입 매개변수가 특정 라이프타임('static 등)을
요구하는 참조와 함께 사용될 때, 해당 요구사항을 충족하도록 제약되어야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add 'static lifetime bound to type parameter",
                "Добавить ограничение 'static к параметру типа",
                "타입 매개변수에 'static 라이프타임 바운드 추가"
            ),
            code:        "struct Foo<T: 'static> {\n    foo: &'static T\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Static Lifetime",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0310.html"
        }
    ]
};
