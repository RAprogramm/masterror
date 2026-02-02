// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0261: undeclared lifetime

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0261",
    title:       LocalizedText::new(
        "Use of undeclared lifetime",
        "Использование необъявленного времени жизни",
        "선언되지 않은 수명 사용"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A lifetime parameter was used without first declaring it.

Lifetimes must be explicitly declared as generic parameters before
they can be used in function signatures, struct definitions, or
impl blocks.

The lifetime parameter must be declared in angle brackets before
being referenced in the type or function signature.",
        "\
Параметр времени жизни был использован без предварительного объявления.

Времена жизни должны быть явно объявлены как параметры дженериков,
прежде чем их можно использовать в сигнатурах функций, определениях
структур или блоках impl.",
        "\
수명 매개변수가 선언되지 않고 사용되었습니다.
수명은 사용하기 전에 제네릭 매개변수로 선언해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Declare lifetime in function signature",
                "Объявите время жизни в сигнатуре функции",
                "함수 시그니처에서 수명 선언"
            ),
            code:        "fn foo<'a>(x: &'a str) {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Declare lifetime in struct definition",
                "Объявите время жизни в определении структуры",
                "구조체 정의에서 수명 선언"
            ),
            code:        "struct Foo<'a> {\n    x: &'a str,\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Declare lifetime on impl block",
                "Объявите время жизни в блоке impl",
                "impl 블록에서 수명 선언"
            ),
            code:        "impl<'a> Foo<'a> {\n    fn foo(x: &'a str) {}\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Lifetimes",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0261.html"
        }
    ]
};
