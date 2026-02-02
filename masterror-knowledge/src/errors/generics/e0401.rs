// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0401: inner items do not inherit generic parameters

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0401",
    title:       LocalizedText::new(
        "Inner items do not inherit generic parameters",
        "Вложенные элементы не наследуют параметры типов",
        "내부 항목은 제네릭 매개변수를 상속하지 않음"
    ),
    category:    Category::Generics,
    explanation: LocalizedText::new(
        "\
Nested items (functions, types, or structs) cannot use generic parameters from
their enclosing scope. Inner items are treated as top-level items that can
only be accessed from within their containing scope.

This is a deliberate design choice - inner functions need their own generic
parameters to be self-contained.",
        "\
Вложенные элементы (функции, типы или структуры) не могут использовать
параметры типов из внешней области видимости. Внутренние элементы
рассматриваются как элементы верхнего уровня.

Это сделано намеренно - внутренние функции должны быть самодостаточными.",
        "\
중첩된 항목(함수, 타입 또는 구조체)은 외부 스코프의 제네릭 매개변수를
사용할 수 없습니다. 내부 항목은 최상위 항목으로 취급됩니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a closure instead of inner function",
                "Использовать замыкание вместо функции",
                "내부 함수 대신 클로저 사용"
            ),
            code:        "fn foo<T>(x: T) {\n    let bar = |y: T| { /* closure captures T */ };\n    bar(x);\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly declare generic parameters in inner item",
                "Явно объявить параметры типов во вложенном элементе",
                "내부 항목에 제네릭 매개변수 명시적 선언"
            ),
            code:        "fn foo<T: Copy>(x: T) {\n    fn bar<T: Copy>(y: T) { }\n    bar(x);\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0401.html"
    }]
};
