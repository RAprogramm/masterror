// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0403: duplicate type parameter name

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0403",
    title:       LocalizedText::new(
        "Some type parameters have the same name",
        "Несколько параметров типа имеют одинаковое имя",
        "일부 타입 매개변수가 같은 이름을 가짐"
    ),
    category:    Category::Generics,
    explanation: LocalizedText::new(
        "\
Multiple type parameters with the same name were declared in a function, trait,
or other generic item. Rust requires all type parameters within the same scope
to have unique names.

Type parameters in associated items also cannot shadow parameters from the
containing item.",
        "\
В функции, трейте или другом обобщённом элементе объявлено несколько
параметров типа с одинаковым именем. Rust требует, чтобы все параметры
типа в одной области видимости имели уникальные имена.",
        "\
함수, 트레이트 또는 다른 제네릭 항목에서 같은 이름의 타입 매개변수가
여러 개 선언되었습니다. Rust는 같은 스코프 내 모든 타입 매개변수가
고유한 이름을 가질 것을 요구합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename clashing type parameters to be unique",
            "Переименовать конфликтующие параметры типа",
            "충돌하는 타입 매개변수 이름 변경"
        ),
        code:        "fn f<T, U>(s: T, u: U) {} // Use different names"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0403.html"
    }]
};
