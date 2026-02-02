// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0777: literal in derive

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0777",
    title:       LocalizedText::new(
        "Literal value in derive",
        "Литеральное значение в derive",
        "derive에 리터럴 값"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A literal value (like a string) was used inside `#[derive]` instead of a
path to a trait.

The `#[derive]` attribute only accepts trait paths as arguments, not string
literals or other literal values.",
        "\
В `#[derive]` использовано литеральное значение (например, строка)
вместо пути к трейту.

`#[derive]` принимает только пути к трейтам.",
        "\
`#[derive]`에 트레이트 경로 대신 리터럴 값이 사용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove quotes from trait name",
                "Удалите кавычки из имени трейта",
                "트레이트 이름에서 따옴표 제거"
            ),
            code:        "#[derive(Clone)] // not \"Clone\"\nstruct Foo;"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Derivable Traits",
            url:   "https://doc.rust-lang.org/book/appendix-03-derivable-traits.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0777.html"
        }
    ]
};
