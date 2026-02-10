// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0229: associated item constraint in unexpected context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0229",
    title:       LocalizedText::new(
        "Associated item constraint in unexpected context",
        "Ограничение ассоциированного элемента в неожиданном контексте",
        "예상치 못한 컨텍스트에서 연관 항목 제약"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
An associated item constraint was written in an unexpected context.
Associated type constraints (like `A = Bar`) cannot be used directly
in trait object or cast syntax.

They must be placed in the appropriate location: either in the type
parameter bounds or the `where` clause.",
        "\
Ограничение ассоциированного элемента было написано в неожиданном контексте.
Ограничения ассоциированных типов (вроде `A = Bar`) нельзя использовать
напрямую в синтаксисе трейт-объектов или приведения типов.

Они должны быть размещены в параметрах типа или в `where` clause.",
        "\
연관 항목 제약이 예상치 못한 컨텍스트에서 작성되었습니다.
연관 타입 제약은 타입 매개변수 바운드나 where 절에 배치해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Move constraints to type parameter bounds",
                "Переместите ограничения в параметры типа",
                "제약을 타입 매개변수 바운드로 이동"
            ),
            code:        "fn baz<I: Foo<A=Bar>>(x: &<I as Foo>::A) {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Move constraints to where clause",
                "Переместите ограничения в where clause",
                "제약을 where 절로 이동"
            ),
            code:        "fn baz<I>(x: &<I as Foo>::A) where I: Foo<A=Bar> {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Trait Bounds",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0229.html"
        }
    ]
};
