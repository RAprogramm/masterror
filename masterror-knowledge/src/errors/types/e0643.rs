// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0643: impl Trait mismatch in trait implementation

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0643",
    title:       LocalizedText::new(
        "Mismatch between impl Trait and generic parameters",
        "Несоответствие между impl Trait и обобщёнными параметрами",
        "impl Trait와 제네릭 매개변수 간 불일치"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
There is a mismatch between how generic parameters are declared in a trait
method versus how they are implemented.

You cannot substitute explicit generic parameters for `impl Trait` parameters
when implementing a trait. The implementation must match the trait signature
exactly.",
        "\
Существует несоответствие между объявлением обобщённых параметров в методе
трейта и их реализацией.

Вы не можете заменить явные обобщённые параметры на параметры `impl Trait`
при реализации трейта. Реализация должна точно соответствовать сигнатуре
трейта.",
        "\
트레이트 메서드에서 제네릭 매개변수가 선언되는 방식과 구현되는 방식 간에
불일치가 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match the trait signature exactly",
            "Точно соответствовать сигнатуре трейта",
            "트레이트 시그니처와 정확히 일치시킴"
        ),
        code:        "trait Foo {\n    fn foo(&self, _: &impl Iterator);\n}\n\nimpl Foo for () {\n    fn foo(&self, _: &impl Iterator) {} // match exactly\n}"
    }],
    links:       &[
        DocLink {
            title: "impl Trait",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0643.html"
        }
    ]
};
