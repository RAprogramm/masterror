// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0015: non-const function called in const context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0015",
    title:       LocalizedText::new(
        "Non-const function in const context",
        "Не-const функция в const контексте",
        "const 컨텍스트에서 non-const 함수"
    ),
    category:    Category::Consts,
    explanation: LocalizedText::new(
        "\
This error occurs when you call a non-`const` function within a constant or
static expression. Only `const fn` functions can be evaluated at compile time.

Example:
    fn create_some() -> Option<u8> { Some(1) }
    const FOO: Option<u8> = create_some();  // Error: not a const fn",
        "\
Эта ошибка возникает при вызове не-const функции в константном выражении.
Только функции с `const fn` могут вычисляться во время компиляции.",
        "\
이 오류는 상수 표현식에서 non-const 함수를 호출할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Mark the function as const fn",
            "Пометить функцию как const fn",
            "함수를 const fn으로 표시"
        ),
        code:        "const fn create_some() -> Option<u8> { Some(1) }\nconst FOO: Option<u8> = create_some();"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Const Functions",
            url:   "https://doc.rust-lang.org/reference/const_eval.html#const-functions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0015.html"
        }
    ]
};
