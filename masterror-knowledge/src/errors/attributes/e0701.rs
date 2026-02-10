// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0701: #[non_exhaustive] misplaced (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0701",
    title:       LocalizedText::new(
        "Non-exhaustive attribute misplaced",
        "Атрибут non_exhaustive в неправильном месте",
        "non_exhaustive 속성 잘못된 위치"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

Previously, the `#[non_exhaustive]` attribute was incorrectly placed on
something other than a struct or enum. This attribute can only be applied
to structs and enums.",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Атрибут `#[non_exhaustive]` может применяться только к структурам и enum.",
        "\
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Apply to struct or enum only",
                "Применяйте только к struct или enum",
                "struct 또는 enum에만 적용"
            ),
            code:        "#[non_exhaustive]\nstruct Config {\n    field: u32,\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0701.html"
        }
    ]
};
