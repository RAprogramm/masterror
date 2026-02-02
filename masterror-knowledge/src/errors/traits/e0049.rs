// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0049: wrong number of type parameters in trait impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0049",
    title:       LocalizedText::new(
        "Wrong type parameter count in impl",
        "Неверное количество параметров типа в impl",
        "impl에서 잘못된 타입 매개변수 수"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This error occurs when a trait method implementation has a different number
of type parameters than the trait definition.

Example:
    trait Foo { fn foo<T>(x: T); }
    impl Foo for Bar {
        fn foo(x: bool) {}  // Error: expected 1 type parameter, found 0
    }",
        "\
Эта ошибка возникает, когда реализация метода трейта имеет другое количество
параметров типа, чем определение трейта.",
        "\
이 오류는 트레이트 메서드 구현의 타입 매개변수 수가 정의와 다를 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match trait's type parameter count",
            "Соответствовать количеству параметров типа трейта",
            "트레이트의 타입 매개변수 수와 일치"
        ),
        code:        "impl Foo for Bar {\n    fn foo<T>(x: T) { }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0049.html"
    }]
};
