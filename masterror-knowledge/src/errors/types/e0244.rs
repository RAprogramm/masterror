// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0244: too many type parameters

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0244",
    title:       LocalizedText::new(
        "Too many type parameters provided",
        "Предоставлено слишком много параметров типа",
        "제공된 타입 매개변수가 너무 많음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Too many type parameters were found in a type or trait usage.

You attempted to provide more type parameters than the type or trait
actually accepts. The number of type arguments must match the number
of type parameters in the definition.

Note: This error code is no longer emitted by the compiler.",
        "\
В использовании типа или трейта найдено слишком много параметров типа.

Вы попытались предоставить больше параметров типа, чем тип или трейт
фактически принимает.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
타입 또는 트레이트 사용에서 너무 많은 타입 매개변수가 제공되었습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Provide only the required type parameters",
            "Укажите только требуемые параметры типа",
            "필요한 타입 매개변수만 제공"
        ),
        code:        "struct Foo<T> { x: T }\n\nstruct Bar { x: Foo<i32> }  // only one parameter"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Generic Types",
            url:   "https://doc.rust-lang.org/book/ch10-01-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0244.html"
        }
    ]
};
