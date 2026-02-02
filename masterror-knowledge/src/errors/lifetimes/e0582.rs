// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0582: lifetime only in associated-type binding

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0582",
    title:       LocalizedText::new(
        "Lifetime only present in associated-type binding",
        "Время жизни присутствует только в привязке ассоциированного типа",
        "라이프타임이 연관 타입 바인딩에서만 존재"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A lifetime is only present in an associated-type binding, and not in the input
types to the trait. This creates an unsatisfiable constraint because the
compiler cannot guarantee that the lifetime is actually used.

The lifetime must also appear in the input types to properly constrain it.",
        "\
Время жизни присутствует только в привязке ассоциированного типа, а не
во входных типах трейта. Это создаёт неудовлетворимое ограничение, поскольку
компилятор не может гарантировать, что время жизни действительно используется.

Время жизни должно также появляться во входных типах.",
        "\
라이프타임이 연관 타입 바인딩에만 존재하고 트레이트의 입력 타입에는 없습니다.
이는 컴파일러가 라이프타임이 실제로 사용된다는 것을 보장할 수 없기 때문에
충족할 수 없는 제약을 만듭니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Include lifetime in input types",
            "Включить время жизни во входные типы",
            "입력 타입에 라이프타임 포함"
        ),
        code:        "where F: for<'a> Fn(&'a i32) -> Option<&'a i32>"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0582.html"
    }]
};
