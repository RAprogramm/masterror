// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0243: not enough type parameters

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0243",
    title:       LocalizedText::new(
        "Not enough type parameters provided",
        "Предоставлено недостаточно параметров типа",
        "제공된 타입 매개변수가 충분하지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Not enough type parameters were found in a type or trait usage.

When a struct, trait, or other type is defined as generic (with type
parameters), all those type parameters must be specified when using
that type. If you fail to provide the required type parameters, you
get this error.

Note: This error code is no longer emitted by the compiler.",
        "\
В использовании типа или трейта найдено недостаточно параметров типа.

Когда struct, trait или другой тип определён как обобщённый,
все параметры типа должны быть указаны при использовании.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
타입 또는 트레이트 사용에서 충분한 타입 매개변수가 제공되지 않았습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Provide all required type parameters",
            "Укажите все требуемые параметры типа",
            "필요한 모든 타입 매개변수 제공"
        ),
        code:        "struct Foo<T> { x: T }\n\nstruct Bar { x: Foo<i32> }  // provide T"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Generic Types",
            url:   "https://doc.rust-lang.org/book/ch10-01-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0243.html"
        }
    ]
};
