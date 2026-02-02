// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0788: coverage attribute in invalid position (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0788",
    title:       LocalizedText::new(
        "Coverage attribute in invalid position",
        "Атрибут coverage в неправильном месте",
        "잘못된 위치의 coverage 속성"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

A `#[coverage(off|on)]` attribute was found in a position where it is not allowed.

Coverage attributes can be applied to:
- Function and method declarations with a body
- Closure expressions
- `impl` blocks and modules",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Атрибут `#[coverage]` может применяться к функциям, замыканиям,
блокам impl и модулям.",
        "\
참고: 이 오류 코드는 더 이상 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Apply coverage to valid items",
                "Применяйте coverage к допустимым элементам",
                "유효한 항목에 coverage 적용"
            ),
            code:        "#[coverage(off)]\nfn uncovered_fn() { /* ... */ }"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0788.html"
        }
    ]
};
