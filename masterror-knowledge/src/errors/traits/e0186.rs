// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0186: method has a self declaration in the trait, but not in the impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0186",
    title:       LocalizedText::new(
        "Method has a self declaration in the trait, but not in the impl",
        "Метод имеет объявление self в трейте, но не в impl",
        "메서드가 트레이트에는 self 선언이 있지만 impl에는 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated function for a trait was defined to be a method (taking a self
parameter), but an implementation of the trait declared the same function to
be static (without self). The function signature must match exactly.",
        "\
Ассоциированная функция трейта определена как метод (с параметром self),
но реализация трейта объявила ту же функцию как статическую (без self).
Сигнатура функции должна точно совпадать.",
        "\
트레이트의 연관 함수가 메서드(self 매개변수 있음)로 정의되어 있지만
트레이트 구현은 동일한 함수를 정적(self 없음)으로 선언합니다.
함수 시그니처는 정확히 일치해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match the trait signature - add self parameter",
            "Соответствовать сигнатуре трейта - добавить параметр self",
            "트레이트 시그니처와 일치 - self 매개변수 추가"
        ),
        code:        "trait Foo {\n    fn foo(&self);\n}\n\nstruct Bar;\n\nimpl Foo for Bar {\n    fn foo(&self) {} // ok! matches trait\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Implementing a Trait",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0186.html"
        }
    ]
};
