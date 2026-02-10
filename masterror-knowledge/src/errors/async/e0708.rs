// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0708: async non-move closure with parameters (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0708",
    title:       LocalizedText::new(
        "Async non-move closure with parameters",
        "Async замыкание без move с параметрами",
        "매개변수가 있는 async non-move 클로저"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

Previously, `async` closures with parameters required the `move` keyword.
Modern Rust has relaxed this restriction.",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Ранее `async` замыкания с параметрами требовали ключевое слово `move`.",
        "\
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add move keyword (legacy fix)",
                "Добавьте ключевое слово move",
                "move 키워드 추가"
            ),
            code:        "let add_one = async move |num: u8| {\n    num + 1\n};"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0708.html"
        }
    ]
};
