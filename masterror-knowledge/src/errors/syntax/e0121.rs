// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0121: type placeholder _ used in item signature

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0121",
    title:       LocalizedText::new(
        "Type placeholder _ used in item signature",
        "Заполнитель типа _ использован в сигнатуре элемента",
        "아이템 시그니처에 타입 플레이스홀더 _ 사용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The type placeholder _ cannot be used within a type on an item's signature.
This applies to function return types and static variable declarations.

The _ placeholder can be used in variable bindings where the compiler can
infer the type, but not in places where an explicit type is required.",
        "\
Заполнитель типа _ не может использоваться в типе сигнатуры элемента.
Это относится к возвращаемым типам функций и объявлениям статических переменных.

Заполнитель _ можно использовать в привязках переменных, где компилятор
может вывести тип, но не там, где требуется явный тип.",
        "\
타입 플레이스홀더 _는 아이템 시그니처의 타입에서 사용할 수 없습니다.
이는 함수 반환 타입과 정적 변수 선언에 적용됩니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Provide the type explicitly in function signature",
                "Указать тип явно в сигнатуре функции",
                "함수 시그니처에 타입을 명시적으로 제공"
            ),
            code:        "fn foo() -> i32 { 5 } // not fn foo() -> _ { 5 }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Provide the type explicitly for static",
                "Указать тип явно для статической переменной",
                "정적 변수에 타입을 명시적으로 제공"
            ),
            code:        "static BAR: &str = \"test\"; // not static BAR: _ = \"test\""
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Type Inference",
            url:   "https://doc.rust-lang.org/reference/type-inference.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0121.html"
        }
    ]
};
