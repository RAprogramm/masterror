// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0716: temporary value dropped while borrowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0716",
    title:       LocalizedText::new(
        "Temporary value dropped while borrowed",
        "Временное значение уничтожено во время заимствования",
        "빌린 동안 임시 값이 삭제됨"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A temporary value was created, borrowed, and then immediately dropped.
The borrow outlives the temporary.

Temporaries only live until the end of the statement by default.",
        "\
Было создано временное значение, заимствовано и сразу уничтожено.",
        "\
임시 값이 생성되고, 빌려지고, 즉시 삭제되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Bind temporary to a variable",
            "Привязать временное значение к переменной",
            "임시 값을 변수에 바인딩"
        ),
        code:        "let value = create_value();\nlet reference = &value;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0716.html"
    }]
};
