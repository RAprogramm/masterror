// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0061: wrong number of function arguments

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0061",
    title:       LocalizedText::new(
        "Wrong number of function arguments",
        "Неверное количество аргументов функции",
        "잘못된 함수 인수 수"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when a function is called with the wrong number of arguments.
The number of arguments must exactly match the function signature.

Example:
    fn f(a: u16, b: &str) {}
    f(2);  // Error: expected 2 arguments, found 1",
        "\
Эта ошибка возникает при вызове функции с неверным количеством аргументов.
Количество аргументов должно точно соответствовать сигнатуре функции.",
        "\
이 오류는 잘못된 수의 인수로 함수를 호출할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Provide all required arguments",
            "Предоставить все необходимые аргументы",
            "모든 필수 인수 제공"
        ),
        code:        "fn f(a: u16, b: &str) {}\nf(2, \"test\");  // Correct"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Functions",
            url:   "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0061.html"
        }
    ]
};
