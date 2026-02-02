// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0228: undeducible lifetime bound for trait objects

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0228",
    title:       LocalizedText::new(
        "Undeducible lifetime bound for trait object",
        "Невыводимое ограничение времени жизни для трейт-объекта",
        "트레이트 객체의 추론 불가능한 수명 바운드"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The lifetime bound for this object type cannot be deduced from context
and must be specified.

This error occurs when a trait object is used as a type argument in
a generic type that has multiple lifetime bounds, and Rust cannot
automatically infer which lifetime should apply to the trait object.",
        "\
Ограничение времени жизни для этого типа объекта не может быть выведено
из контекста и должно быть указано.

Эта ошибка возникает, когда трейт-объект используется как аргумент типа
в обобщённом типе с несколькими ограничениями времени жизни.",
        "\
이 객체 타입의 수명 바운드를 컨텍스트에서 추론할 수 없으므로
명시적으로 지정해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly specify the trait object's lifetime",
                "Явно укажите время жизни трейт-объекта",
                "트레이트 객체의 수명을 명시적으로 지정"
            ),
            code:        "type Foo<'a, 'b> = TwoBounds<'a, 'b, dyn Trait + 'b>;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Reduce to a single lifetime bound",
                "Сократите до одного ограничения времени жизни",
                "단일 수명 바운드로 축소"
            ),
            code:        "struct OneBound<'a, T: 'a> {\n    x: &'a i32,\n    z: T,\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 599: Default Object Bound",
            url:   "https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0228.html"
        }
    ]
};
