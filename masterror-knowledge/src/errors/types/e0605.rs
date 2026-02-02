// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0605: non-primitive cast

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0605",
    title:       LocalizedText::new(
        "Non-primitive cast attempted",
        "Попытка непримитивного приведения типа",
        "비원시 타입 캐스팅 시도"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An invalid cast was attempted. Only primitive types can be directly cast
into each other using the `as` operator.

For complex type conversions, use the `Into`/`From` traits, constructor
methods, or type-specific conversion functions.",
        "\
Была предпринята попытка недопустимого приведения типа. Только примитивные
типы могут быть напрямую приведены друг к другу с помощью оператора `as`.

Для сложных преобразований типов используйте трейты `Into`/`From`,
методы-конструкторы или функции преобразования типов.",
        "\
잘못된 캐스팅이 시도되었습니다. `as` 연산자를 사용하여 직접 캐스팅할 수
있는 것은 원시 타입뿐입니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use From/Into traits for complex conversions",
                "Использовать трейты From/Into для сложных преобразований",
                "복잡한 변환에는 From/Into 트레이트 사용"
            ),
            code:        "let v: Vec<u8> = x.into();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Cast only between primitive types",
                "Приводить только между примитивными типами",
                "원시 타입 간에만 캐스팅"
            ),
            code:        "let x = 0u8 as u32; // ok"
        }
    ],
    links:       &[
        DocLink {
            title: "Type Cast Expressions",
            url:   "https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0605.html"
        }
    ]
};
