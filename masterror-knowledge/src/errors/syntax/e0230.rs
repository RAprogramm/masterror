// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0230: invalid identifier in rustc_on_unimplemented

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0230",
    title:       LocalizedText::new(
        "Invalid identifier in #[rustc_on_unimplemented]",
        "Недопустимый идентификатор в #[rustc_on_unimplemented]",
        "#[rustc_on_unimplemented]에서 잘못된 식별자"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
The `#[rustc_on_unimplemented]` attribute contains an identifier in
curly braces that doesn't match any of the following:
- The trait's type parameters
- The string `Self`

Placeholders in curly braces must be valid - either existing type
parameters or the special `Self` keyword.",
        "\
Атрибут `#[rustc_on_unimplemented]` содержит идентификатор в фигурных
скобках, который не соответствует ни одному из параметров типа трейта
или строке `Self`.",
        "\
`#[rustc_on_unimplemented]` 속성에 중괄호 안의 식별자가
트레이트의 타입 매개변수나 `Self`와 일치하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use correct type parameter name or Self",
                "Используйте правильное имя параметра типа или Self",
                "올바른 타입 매개변수 이름 또는 Self 사용"
            ),
            code:        "#[rustc_on_unimplemented = \"error on `{Self}` with param `<{A}>`\"]\ntrait MyTrait<A> {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Escape literal braces with double braces",
                "Экранируйте литеральные скобки двойными скобками",
                "이중 중괄호로 리터럴 중괄호 이스케이프"
            ),
            code:        "#[rustc_on_unimplemented = \"use {{braces}} literally\"]"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Internals: Custom Error Messages",
            url:   "https://doc.rust-lang.org/unstable-book/language-features/rustc-attrs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0230.html"
        }
    ]
};
