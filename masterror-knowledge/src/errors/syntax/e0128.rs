// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0128: forward declared generic parameter in default

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0128",
    title:       LocalizedText::new(
        "Forward declared generic parameter in default",
        "Опережающее объявление параметра типа в значении по умолчанию",
        "기본값에서 전방 선언된 제네릭 매개변수"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when a type parameter's default value references another
type parameter that hasn't been declared yet.

Example:
    struct Foo<T = U, U = ()> { ... }  // Error: U not declared yet

Type parameters are evaluated in order, so defaults can only reference
parameters that come before them.",
        "\
Эта ошибка возникает, когда значение по умолчанию параметра типа ссылается
на другой параметр типа, который ещё не объявлен.",
        "\
이 오류는 타입 매개변수의 기본값이 아직 선언되지 않은 다른 타입 매개변수를 참조할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Reorder type parameters",
            "Изменить порядок параметров типа",
            "타입 매개변수 순서 변경"
        ),
        code:        "struct Foo<U = (), T = U> {\n    field1: T,\n    field2: U,\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0128.html"
    }]
};
