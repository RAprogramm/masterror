// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0780: doc(inline) with anonymous import

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0780",
    title:       LocalizedText::new(
        "Cannot use doc(inline) with anonymous imports",
        "Нельзя использовать doc(inline) с анонимным импортом",
        "익명 임포트와 doc(inline) 사용 불가"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
The `#[doc(inline)]` attribute was applied to an anonymous import (using `as _`).

Anonymous imports are always rendered with `#[doc(no_inline)]` by default,
making the `#[doc(inline)]` attribute invalid in this context.",
        "\
Атрибут `#[doc(inline)]` применён к анонимному импорту (с `as _`).

Анонимные импорты всегда отображаются с `#[doc(no_inline)]`.",
        "\
`#[doc(inline)]` 속성이 익명 임포트(`as _` 사용)에 적용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove doc(inline) attribute",
                "Удалите атрибут doc(inline)",
                "doc(inline) 속성 제거"
            ),
            code:        "pub use foo::Foo as _; // without #[doc(inline)]"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0780.html"
        }
    ]
};
