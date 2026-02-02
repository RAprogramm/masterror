// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0415: duplicate function parameter name

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0415",
    title:       LocalizedText::new(
        "More than one function parameter have the same name",
        "Несколько параметров функции имеют одинаковое имя",
        "여러 함수 매개변수가 같은 이름을 가짐"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A function was declared with multiple parameters that share the same
identifier name. Rust does not allow duplicate parameter names in
a function signature - each parameter must have a unique name.",
        "\
Функция объявлена с несколькими параметрами, имеющими одинаковое имя.
Rust не допускает дублирование имён параметров в сигнатуре функции -
каждый параметр должен иметь уникальное имя.",
        "\
함수가 같은 이름을 가진 여러 매개변수로 선언되었습니다. Rust는
함수 시그니처에서 매개변수 이름 중복을 허용하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename parameters to be unique",
            "Переименовать параметры, чтобы они были уникальны",
            "매개변수 이름을 고유하게 변경"
        ),
        code:        "fn foo(f: i32, g: i32) {} // Different names"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0415.html"
    }]
};
