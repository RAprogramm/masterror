// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0222: invalid associated type constraint

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0222",
    title:       LocalizedText::new(
        "Invalid associated type constraint",
        "Недопустимое ограничение ассоциированного типа",
        "잘못된 연관 타입 제약"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An attempt was made to constrain an associated type directly in a
function signature using the syntax `TraitName<AssociatedType=Type>`,
which is not allowed in this context.

When a trait inherits from multiple supertraits that define the same
associated type name, there's ambiguity. You cannot use the direct
constraint syntax in function parameters with trait objects.",
        "\
Была попытка ограничить ассоциированный тип напрямую в сигнатуре функции
с использованием синтаксиса `TraitName<AssociatedType=Type>`,
что не допускается в этом контексте.",
        "\
함수 시그니처에서 `TraitName<AssociatedType=Type>` 구문으로 연관 타입을
직접 제약하려고 했으나, 이 컨텍스트에서는 허용되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use where clause with type parameter",
            "Используйте where с параметром типа",
            "타입 매개변수와 where 절 사용"
        ),
        code:        "fn foo<CAR, COLOR>(\n    c: CAR,\n) where\n    CAR: BoxCar,\n    CAR: Vehicle<Color = COLOR>,\n    CAR: Box<Color = COLOR>\n{}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Trait Bounds",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0222.html"
        }
    ]
};
