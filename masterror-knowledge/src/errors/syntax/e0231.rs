// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0231: invalid format string in rustc_on_unimplemented

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0231",
    title:       LocalizedText::new(
        "Invalid format string in #[rustc_on_unimplemented]",
        "Недопустимая строка формата в #[rustc_on_unimplemented]",
        "#[rustc_on_unimplemented]에서 잘못된 형식 문자열"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
The `#[rustc_on_unimplemented]` attribute has an invalid format string.
Curly braces in the attribute must contain a valid identifier (either
a type parameter name or `{Self}`), not empty braces.

The format string supports:
- Type parameters: Referenced by name in curly braces (e.g., `{A}`)
- `{Self}`: Refers to the type that failed to implement the trait
- Standard Rust string formatting rules",
        "\
Атрибут `#[rustc_on_unimplemented]` имеет недопустимую строку формата.
Фигурные скобки должны содержать допустимый идентификатор,
а не пустые скобки.",
        "\
`#[rustc_on_unimplemented]` 속성에 잘못된 형식 문자열이 있습니다.
중괄호에는 유효한 식별자가 포함되어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use valid identifier in braces",
            "Используйте допустимый идентификатор в скобках",
            "중괄호에 유효한 식별자 사용"
        ),
        code:        "#[rustc_on_unimplemented = \"error on `{Self}` with params `<{A}>`\"]\ntrait MyTrait<A> {}"
    }],
    links:       &[
        DocLink {
            title: "Rust Internals: Custom Error Messages",
            url:   "https://doc.rust-lang.org/unstable-book/language-features/rustc-attrs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0231.html"
        }
    ]
};
