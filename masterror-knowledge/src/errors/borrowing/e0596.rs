// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0596: cannot borrow as mutable, as it is not declared as mutable

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0596",
    title:       LocalizedText::new(
        "Cannot borrow as mutable (not declared as mutable)",
        "Нельзя заимствовать как изменяемое (не объявлено как изменяемое)",
        "가변으로 빌릴 수 없음 (가변으로 선언되지 않음)"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
You're trying to get a mutable reference to something that wasn't declared
as mutable. To modify through a reference, the original binding must be `mut`.

This is Rust's way of making mutation explicit and visible in the code.",
        "\
Вы пытаетесь получить изменяемую ссылку на то, что не было объявлено
как изменяемое. Для изменения через ссылку оригинал должен быть `mut`.",
        "\
가변으로 선언되지 않은 것에 대한 가변 참조를 얻으려고 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add mut to the variable declaration",
                "Добавить mut к объявлению переменной",
                "변수 선언에 mut 추가"
            ),
            code:        "let mut x = vec![1, 2, 3];"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Add mut to function parameter",
                "Добавить mut к параметру функции",
                "함수 매개변수에 mut 추가"
            ),
            code:        "fn process(data: &mut Vec<i32>) { ... }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0596.html"
    }]
};
