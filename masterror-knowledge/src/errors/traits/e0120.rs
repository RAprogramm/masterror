// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0120: Drop implemented on trait object or reference

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0120",
    title:       LocalizedText::new(
        "Drop implemented on trait object or reference",
        "Drop реализован для трейт-объекта или ссылки",
        "트레이트 객체 또는 참조에 Drop 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Drop was implemented on a trait object or reference, which is not allowed.
Only structs, enums, and unions can implement the Drop trait.

The Drop trait is used for cleanup when a value goes out of scope, and
it only makes sense for owned types, not for references or trait objects.",
        "\
Drop был реализован для трейт-объекта или ссылки, что не разрешено.
Только структуры, перечисления и объединения могут реализовать трейт Drop.

Трейт Drop используется для очистки когда значение выходит из области
видимости и имеет смысл только для владеющих типов.",
        "\
트레이트 객체 또는 참조에 Drop이 구현되었는데, 이는 허용되지 않습니다.
구조체, 열거형, 공용체만 Drop 트레이트를 구현할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a wrapper struct with generic type bound",
                "Использовать структуру-обёртку с ограничением типа",
                "제네릭 타입 바운드가 있는 래퍼 구조체 사용"
            ),
            code:        "trait MyTrait {}\nstruct MyWrapper<T: MyTrait> { foo: T }\n\nimpl<T: MyTrait> Drop for MyWrapper<T> {\n    fn drop(&mut self) {}\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a wrapper containing trait object",
                "Использовать обёртку с трейт-объектом",
                "트레이트 객체를 포함하는 래퍼 사용"
            ),
            code:        "trait MyTrait {}\n\nstruct MyWrapper<'a> { foo: &'a dyn MyTrait }\n\nimpl<'a> Drop for MyWrapper<'a> {\n    fn drop(&mut self) {}\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Drop Trait",
            url:   "https://doc.rust-lang.org/book/ch15-03-drop.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0120.html"
        }
    ]
};
