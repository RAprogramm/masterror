// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0185: method has a self declaration in the impl, but not in the trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0185",
    title:       LocalizedText::new(
        "Method has a self declaration in the impl, but not in the trait",
        "Метод имеет объявление self в impl, но не в трейте",
        "메서드가 impl에는 self 선언이 있지만 트레이트에는 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated function in a trait is defined as static (taking no self
parameter), but the implementation declares it as a method (taking a self
parameter). Trait implementations must match the exact signature of the
trait definition.",
        "\
Ассоциированная функция в трейте определена как статическая (без параметра
self), но реализация объявляет её как метод (с параметром self).
Реализации трейтов должны точно соответствовать сигнатуре определения трейта.",
        "\
트레이트의 연관 함수가 정적(self 매개변수 없음)으로 정의되어 있지만
구현은 메서드(self 매개변수 있음)로 선언합니다. 트레이트 구현은
트레이트 정의의 정확한 시그니처와 일치해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match the trait signature - remove self parameter",
            "Соответствовать сигнатуре трейта - удалить параметр self",
            "트레이트 시그니처와 일치 - self 매개변수 제거"
        ),
        code:        "trait Foo {\n    fn foo();\n}\n\nstruct Bar;\n\nimpl Foo for Bar {\n    fn foo() {} // ok! matches trait\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Implementing a Trait",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0185.html"
        }
    ]
};
