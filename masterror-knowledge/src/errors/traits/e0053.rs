// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0053: method parameter type mismatch in trait impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0053",
    title:       LocalizedText::new(
        "Parameter type mismatch in impl",
        "Несоответствие типов параметров в impl",
        "impl에서 매개변수 타입 불일치"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This error occurs when a trait method implementation has parameter types or
mutability that don't match the trait definition.

Example:
    trait Foo { fn foo(x: u16); fn bar(&self); }
    impl Foo for Bar {
        fn foo(x: i16) { }      // Error: expected u16, found i16
        fn bar(&mut self) { }   // Error: mutability mismatch
    }",
        "\
Эта ошибка возникает, когда типы параметров или изменяемость в реализации
метода трейта не соответствуют определению трейта.",
        "\
이 오류는 트레이트 메서드 구현의 매개변수 타입이나 가변성이 정의와 일치하지 않을 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match trait's exact parameter types",
            "Точно соответствовать типам параметров трейта",
            "트레이트의 정확한 매개변수 타입과 일치"
        ),
        code:        "impl Foo for Bar {\n    fn foo(x: u16) { }\n    fn bar(&self) { }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0053.html"
    }]
};
