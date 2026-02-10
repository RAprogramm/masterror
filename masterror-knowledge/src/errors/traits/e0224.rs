// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0224: trait object with no traits

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0224",
    title:       LocalizedText::new(
        "Trait object declared with no traits",
        "Трейт-объект объявлен без трейтов",
        "트레이트 없이 트레이트 객체 선언됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A trait object was declared with no traits. Rust requires that trait
objects must have at least one trait specified.

The `dyn` keyword must be followed by at least one actual trait.
Having only a lifetime bound without any trait is not allowed.",
        "\
Трейт-объект был объявлен без трейтов. Rust требует, чтобы
трейт-объекты имели хотя бы один указанный трейт.

За ключевым словом `dyn` должен следовать хотя бы один трейт.",
        "\
트레이트 없이 트레이트 객체가 선언되었습니다.
`dyn` 키워드 뒤에는 최소 하나의 트레이트가 지정되어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add at least one trait",
            "Добавьте хотя бы один трейт",
            "최소 하나의 트레이트 추가"
        ),
        code:        "type Foo = dyn 'static + Copy;"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Trait Objects",
            url:   "https://doc.rust-lang.org/book/ch17-02-trait-objects.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0224.html"
        }
    ]
};
