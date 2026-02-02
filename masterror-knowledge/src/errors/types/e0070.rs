// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0070: invalid left-hand side of assignment

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0070",
    title:       LocalizedText::new(
        "Invalid left-hand side of assignment",
        "Недопустимое левое значение в присваивании",
        "할당의 잘못된 좌변"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when you try to assign to something that isn't a valid
place expression. Only variables, dereferences, indexing expressions, and
field references can be assigned to.

Example:
    1 = 3;            // Error: can't assign to a literal
    some_func() = 4;  // Error: can't assign to function result",
        "\
Эта ошибка возникает при попытке присвоить что-то, что не является
допустимым place-выражением.",
        "\
이 오류는 유효한 place 표현식이 아닌 것에 할당하려고 할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Assign to a mutable variable",
            "Присвоить изменяемой переменной",
            "가변 변수에 할당"
        ),
        code:        "let mut x = 0;\nx = 3;  // Correct"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Place Expressions",
            url:   "https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0070.html"
        }
    ]
};
