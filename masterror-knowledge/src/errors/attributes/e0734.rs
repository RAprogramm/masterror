// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0734: stability attribute outside stdlib

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0734",
    title:       LocalizedText::new(
        "Stability attribute outside standard library",
        "Атрибут стабильности вне стандартной библиотеки",
        "표준 라이브러리 외부의 안정성 속성"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A stability attribute (`#[stable]` or `#[unstable]`) has been used outside
of the standard library.

These attributes are meant to only be used by the standard library and are
rejected in your own crates.",
        "\
Атрибут стабильности (`#[stable]` или `#[unstable]`) использован
вне стандартной библиотеки.

Эти атрибуты предназначены только для стандартной библиотеки.",
        "\
안정성 속성(`#[stable]` 또는 `#[unstable]`)이 표준 라이브러리
외부에서 사용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove stability attributes",
                "Удалите атрибуты стабильности",
                "안정성 속성 제거"
            ),
            code:        "// Instead of:\n// #[stable(feature = \"a\", since = \"1.0\")]\nfn foo() {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0734.html"
        }
    ]
};
