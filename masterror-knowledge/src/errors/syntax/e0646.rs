// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0646: main function with where clause

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0646",
    title:       LocalizedText::new(
        "Main function cannot have where clause",
        "Функция main не может иметь where-предложение",
        "main 함수에는 where 절을 사용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `main` function is not allowed to have a `where` clause. The Rust
compiler does not permit `where` clauses on the `main` function due to
its special role as the entry point of a program.",
        "\
Функция `main` не может иметь `where`-предложение. Компилятор Rust не
допускает `where`-предложения для функции `main` из-за её особой роли
как точки входа программы.",
        "\
`main` 함수에는 `where` 절을 사용할 수 없습니다. Rust 컴파일러는
프로그램의 진입점으로서의 특별한 역할 때문에 `main` 함수에
`where` 절을 허용하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove where clause from main",
                "Удалить where-предложение из main",
                "main에서 where 절 제거"
            ),
            code:        "fn main() {\n    // your code here\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Move generic constraints to helper function",
                "Переместить обобщённые ограничения в вспомогательную функцию",
                "제네릭 제약 조건을 헬퍼 함수로 이동"
            ),
            code:        "fn helper<T: Copy>() { /* ... */ }\n\nfn main() {\n    helper();\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "main Function",
            url:   "https://doc.rust-lang.org/reference/items/functions.html#main-functions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0646.html"
        }
    ]
};
