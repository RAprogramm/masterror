// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0666: nested impl Trait not allowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0666",
    title:       LocalizedText::new(
        "Nested impl Trait not allowed",
        "Вложенный impl Trait не допускается",
        "중첩 impl Trait 허용되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
`impl Trait` types cannot appear nested in the generic arguments of other
`impl Trait` types.

You cannot use an `impl Trait` type as a generic argument inside another
`impl Trait` syntax.",
        "\
Типы `impl Trait` не могут появляться вложенными в обобщённые аргументы
других типов `impl Trait`.

Вы не можете использовать тип `impl Trait` как обобщённый аргумент внутри
другого синтаксиса `impl Trait`.",
        "\
`impl Trait` 타입은 다른 `impl Trait` 타입의 제네릭 인수에 중첩되어
나타날 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use named generic type parameters",
            "Использовать именованные обобщённые параметры типа",
            "명명된 제네릭 타입 매개변수 사용"
        ),
        code:        "fn foo<T: MyInnerTrait>(\n    bar: impl MyGenericTrait<T>,\n) {}"
    }],
    links:       &[
        DocLink {
            title: "impl Trait",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0666.html"
        }
    ]
};
