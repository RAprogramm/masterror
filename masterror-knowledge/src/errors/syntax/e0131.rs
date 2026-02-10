// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0131: main function is not allowed to have generic parameters

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0131",
    title:       LocalizedText::new(
        "main function is not allowed to have generic parameters",
        "Функция main не может иметь обобщённые параметры",
        "main 함수는 제네릭 매개변수를 가질 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The main function was defined with generic parameters, which is not allowed.
The main function is the entry point of a Rust program and has special
restrictions - it cannot be generic or accept any parameters.",
        "\
Функция main была определена с обобщёнными параметрами, что не разрешено.
Функция main является точкой входа программы Rust и имеет особые
ограничения - она не может быть обобщённой или принимать параметры.",
        "\
main 함수가 제네릭 매개변수로 정의되었는데, 이는 허용되지 않습니다.
main 함수는 Rust 프로그램의 진입점이며 제네릭이거나 매개변수를
받을 수 없다는 특별한 제한이 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove generic parameters from main",
            "Удалить обобщённые параметры из main",
            "main에서 제네릭 매개변수 제거"
        ),
        code:        "fn main() {\n    // program entry point\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Entry Point",
            url:   "https://doc.rust-lang.org/reference/crates-and-source-files.html#main-functions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0131.html"
        }
    ]
};
