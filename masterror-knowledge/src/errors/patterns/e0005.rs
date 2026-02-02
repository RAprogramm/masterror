// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0005: refutable pattern in local binding

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0005",
    title:       LocalizedText::new(
        "Refutable pattern in local binding",
        "Опровержимый паттерн в локальной привязке",
        "로컬 바인딩에서 반박 가능한 패턴"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when you use a pattern in a `let` binding that can fail to
match (refutable pattern). Patterns in `let` must be irrefutable - they must
always match.

Example:
    let Some(x) = maybe_value;  // Error: maybe_value could be None

The pattern `Some(x)` is refutable because the value could be `None`.",
        "\
Эта ошибка возникает, когда в `let` используется паттерн, который может
не совпасть (опровержимый паттерн). Паттерны в `let` должны быть
неопровержимыми - всегда совпадать.",
        "\
이 오류는 `let` 바인딩에서 실패할 수 있는 패턴(반박 가능한 패턴)을 사용할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use if let for refutable patterns",
                "Использовать if let для опровержимых паттернов",
                "반박 가능한 패턴에 if let 사용"
            ),
            code:        "if let Some(x) = maybe_value {\n    // use x\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use match to handle all cases",
                "Использовать match для всех случаев",
                "모든 케이스를 처리하기 위해 match 사용"
            ),
            code:        "match maybe_value {\n    Some(x) => { /* use x */ },\n    None => {},\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Refutable Patterns",
            url:   "https://doc.rust-lang.org/book/ch18-02-refutability.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0005.html"
        }
    ]
};
