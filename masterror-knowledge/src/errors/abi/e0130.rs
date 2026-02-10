// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0130: patterns aren't allowed in foreign function declarations

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0130",
    title:       LocalizedText::new(
        "Patterns aren't allowed in foreign function declarations",
        "Паттерны не разрешены в объявлениях внешних функций",
        "외부 함수 선언에서 패턴이 허용되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A pattern (like tuple destructuring) was used as an argument in a foreign
function declaration. Foreign functions (declared with extern \"C\") must
use simple identifiers with explicit type annotations rather than patterns.",
        "\
Паттерн (например, деструктуризация кортежа) был использован как аргумент
в объявлении внешней функции. Внешние функции (объявленные с extern \"C\")
должны использовать простые идентификаторы с явными аннотациями типов.",
        "\
외부 함수 선언에서 패턴(튜플 구조 분해 등)이 인수로 사용되었습니다.
외부 함수(extern \"C\"로 선언)는 패턴 대신 명시적 타입 주석이 있는
간단한 식별자를 사용해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a struct instead of tuple destructuring",
                "Использовать структуру вместо деструктуризации кортежа",
                "튜플 구조 분해 대신 구조체 사용"
            ),
            code:        "struct SomeStruct {\n    a: u32,\n    b: u32,\n}\n\nextern \"C\" {\n    fn foo(s: SomeStruct);\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a simple identifier with the tuple type",
                "Использовать простой идентификатор с типом кортежа",
                "튜플 타입과 함께 간단한 식별자 사용"
            ),
            code:        "extern \"C\" {\n    fn foo(a: (u32, u32));\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: FFI",
            url:   "https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0130.html"
        }
    ]
};
