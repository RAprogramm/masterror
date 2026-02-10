// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0580: main function has wrong type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0580",
    title:       LocalizedText::new(
        "The `main` function has wrong type",
        "Функция `main` имеет неправильный тип",
        "`main` 함수의 타입이 잘못됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `main` function was incorrectly declared. The `main` function is Rust's
entry point and must have a specific signature. It should not accept any
parameters directly.

To get command-line arguments, use `std::env::args`. To exit with a specific
code, use `std::process::exit`.",
        "\
Функция `main` была неправильно объявлена. Функция `main` - это точка входа
Rust и должна иметь определённую сигнатуру. Она не должна принимать параметры
напрямую.

Для получения аргументов командной строки используйте `std::env::args`.",
        "\
`main` 함수가 잘못 선언되었습니다. `main` 함수는 Rust의 진입점이며
특정 시그니처를 가져야 합니다. 매개변수를 직접 받아서는 안 됩니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use correct main signature",
                "Использовать правильную сигнатуру main",
                "올바른 main 시그니처 사용"
            ),
            code:        "fn main() {\n    // your code\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use std::env::args for command-line arguments",
                "Использовать std::env::args для аргументов командной строки",
                "명령줄 인수에 std::env::args 사용"
            ),
            code:        "use std::env;\n\nfn main() {\n    for arg in env::args() {\n        println!(\"{}\", arg);\n    }\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0580.html"
    }]
};
