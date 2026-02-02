// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0107: wrong number of generic arguments

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0107",
    title:       LocalizedText::new(
        "Wrong number of generic arguments",
        "Неправильное количество обобщённых аргументов",
        "제네릭 인수 개수 불일치"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An incorrect number of generic arguments was provided. The compiler expects
a specific number of generic type parameters, lifetime parameters, or const
parameters, but you've supplied a different amount.

This commonly happens when:
- Using a generic type without providing all required type arguments
- Providing more type arguments than the type accepts
- Forgetting lifetime parameters when they are required",
        "\
Указано неправильное количество обобщённых аргументов. Компилятор ожидает
определённое количество параметров типа, времени жизни или констант,
но вы указали другое количество.

Это часто происходит когда:
- Используется обобщённый тип без всех необходимых аргументов
- Указано больше аргументов типа, чем принимает тип
- Забыты параметры времени жизни",
        "\
제공된 제네릭 인수의 수가 올바르지 않습니다. 컴파일러는 특정 수의 타입,
라이프타임 또는 const 매개변수를 기대하지만 다른 수가 제공되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Provide the correct number of type arguments",
                "Указать правильное количество аргументов типа",
                "올바른 수의 타입 인수 제공"
            ),
            code:        "struct Foo<T> { x: T }\nstruct Bar<T> { x: Foo<T> } // provide one type argument"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Check the type definition for required parameters",
                "Проверить определение типа на требуемые параметры",
                "필요한 매개변수에 대한 타입 정의 확인"
            ),
            code:        "fn foo<T, U>(x: T, y: U) {}\nfoo::<bool, u32>(x, 12); // two type arguments needed"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Generic Data Types",
            url:   "https://doc.rust-lang.org/book/ch10-01-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0107.html"
        }
    ]
};
