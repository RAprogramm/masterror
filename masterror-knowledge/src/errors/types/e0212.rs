// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0212: cannot use associated type with uninferred generics

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0212",
    title:       LocalizedText::new(
        "Cannot use associated type with uninferred generics",
        "Нельзя использовать ассоциированный тип с невыведенными дженериками",
        "추론되지 않은 제네릭과 연관 타입 사용 불가"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Cannot use the associated type of a trait with uninferred generic parameters.
The compiler cannot determine what concrete type should be used for
the generic parameter, making it impossible to resolve the associated type.

This commonly occurs with higher-ranked trait bounds (HRTB) where the
lifetime is quantified with `for<'x>`, and the compiler doesn't know
which specific lifetime to substitute when accessing the associated type.",
        "\
Нельзя использовать ассоциированный тип трейта с невыведенными параметрами
дженериков. Компилятор не может определить, какой конкретный тип
использовать для параметра.

Часто встречается с высокоранговыми ограничениями трейтов (HRTB).",
        "\
추론되지 않은 제네릭 매개변수를 가진 트레이트의 연관 타입을 사용할 수 없습니다.
컴파일러가 어떤 구체적인 타입을 사용해야 할지 결정할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly specify generic parameters",
                "Явно укажите параметры дженериков",
                "제네릭 매개변수를 명시적으로 지정"
            ),
            code:        "fn foo3<I: for<'x> Foo<&'x isize>>(\n    x: <I as Foo<&isize>>::A) {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a named lifetime parameter",
                "Используйте именованный параметр времени жизни",
                "명명된 수명 매개변수 사용"
            ),
            code:        "fn foo4<'a, I: for<'x> Foo<&'x isize>>(\n    x: <I as Foo<&'a isize>>::A) {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Higher-ranked Trait Bounds",
            url:   "https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0212.html"
        }
    ]
};
