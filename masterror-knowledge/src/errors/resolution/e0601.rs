// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0601: no main function found

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0601",
    title:       LocalizedText::new(
        "No main function found in binary crate",
        "Функция main не найдена в бинарном крейте",
        "바이너리 크레이트에서 main 함수를 찾을 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
No `main` function was found in a binary crate. The `main` function is the
entry point for any Rust binary - it's where program execution begins.

This error occurs when:
- Creating a binary crate without including a `main` function
- Accidentally deleting or commenting out the `main` function
- Attempting to run a library crate as a binary",
        "\
Функция `main` не найдена в бинарном крейте. Функция `main` является
точкой входа для любого бинарного файла Rust - это место, где начинается
выполнение программы.

Эта ошибка возникает, когда:
- Создаётся бинарный крейт без функции `main`
- Случайно удалена или закомментирована функция `main`
- Попытка запустить библиотечный крейт как бинарный",
        "\
바이너리 크레이트에서 `main` 함수를 찾을 수 없습니다. `main` 함수는
모든 Rust 바이너리의 진입점입니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add a main function",
            "Добавить функцию main",
            "main 함수 추가"
        ),
        code:        "fn main() {\n    println!(\"Hello world!\");\n}"
    }],
    links:       &[
        DocLink {
            title: "The Rust Book",
            url:   "https://doc.rust-lang.org/book/"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0601.html"
        }
    ]
};
