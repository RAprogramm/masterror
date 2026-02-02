// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0050: wrong number of parameters in trait impl method

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0050",
    title:       LocalizedText::new(
        "Wrong parameter count in impl method",
        "Неверное количество параметров в методе impl",
        "impl 메서드에서 잘못된 매개변수 수"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This error occurs when a trait method implementation has a different number
of parameters than the trait definition.

Example:
    trait Foo { fn foo(&self, x: u8) -> bool; }
    impl Foo for Bar {
        fn foo(&self) -> bool { true }  // Error: expected 2 params, found 1
    }",
        "\
Эта ошибка возникает, когда реализация метода трейта имеет другое количество
параметров, чем определение трейта.",
        "\
이 오류는 트레이트 메서드 구현의 매개변수 수가 정의와 다를 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match trait's parameter count",
            "Соответствовать количеству параметров трейта",
            "트레이트의 매개변수 수와 일치"
        ),
        code:        "impl Foo for Bar {\n    fn foo(&self, x: u8) -> bool { true }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0050.html"
    }]
};
