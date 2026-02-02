// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0617: invalid type for variadic function

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0617",
    title:       LocalizedText::new(
        "Invalid type for variadic function",
        "Недопустимый тип для функции с переменным числом аргументов",
        "가변 인수 함수에 대한 잘못된 타입"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to pass an invalid type of variable into a variadic function.

When calling C variadic functions (with `...` parameters), the C ABI has
specific rules about which types can be passed. Some Rust types (like `f32`)
must be cast to their corresponding C types (like `c_double`).",
        "\
Попытка передать недопустимый тип переменной в функцию с переменным
числом аргументов.

При вызове C-функций с переменным числом аргументов (с `...` параметрами),
ABI C имеет особые правила о том, какие типы можно передавать. Некоторые
типы Rust (например, `f32`) должны быть приведены к соответствующим
типам C (например, `c_double`).",
        "\
가변 인수 함수에 잘못된 타입의 변수를 전달하려고 시도했습니다.

C 가변 인수 함수를 호출할 때 C ABI에는 전달할 수 있는 타입에 대한
특정 규칙이 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Cast to appropriate C type",
            "Привести к соответствующему типу C",
            "적절한 C 타입으로 캐스팅"
        ),
        code:        "unsafe { printf(\"%f\\n\\0\".as_ptr() as _, 0f64); } // use f64 instead of f32"
    }],
    links:       &[
        DocLink {
            title: "std::os::raw",
            url:   "https://doc.rust-lang.org/std/os/raw/"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0617.html"
        }
    ]
};
