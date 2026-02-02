// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0057: wrong number of closure arguments

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0057",
    title:       LocalizedText::new(
        "Wrong number of closure arguments",
        "Неверное количество аргументов замыкания",
        "잘못된 클로저 인수 수"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when calling a closure with the wrong number of arguments.

Example:
    let f = |x| x * 3;
    f();      // Error: expects 1 argument, got 0
    f(2, 3);  // Error: expects 1 argument, got 2",
        "\
Эта ошибка возникает при вызове замыкания с неверным количеством аргументов.",
        "\
이 오류는 잘못된 수의 인수로 클로저를 호출할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Pass the correct number of arguments",
            "Передать правильное количество аргументов",
            "올바른 수의 인수 전달"
        ),
        code:        "let f = |x| x * 3;\nlet result = f(4);  // Correct: 1 argument"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Closures",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0057.html"
        }
    ]
};
