// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0044: foreign items cannot have type/const parameters

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0044",
    title:       LocalizedText::new(
        "Foreign items cannot have type parameters",
        "Внешние элементы не могут иметь параметры типа",
        "외부 항목은 타입 매개변수를 가질 수 없음"
    ),
    category:    Category::Abi,
    explanation: LocalizedText::new(
        "\
Foreign items declared in `extern` blocks cannot have generic type or const
parameters. Foreign functions (from C libraries) have concrete signatures.

Example:
    extern \"C\" {
        fn some_func<T>(x: T);  // Error: generic not allowed
    }",
        "\
Внешние элементы, объявленные в блоках `extern`, не могут иметь обобщённые
параметры типа или константы. Внешние функции имеют конкретные сигнатуры.",
        "\
`extern` 블록에 선언된 외부 항목은 제네릭 타입이나 const 매개변수를 가질 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Create separate declarations for each type",
            "Создать отдельные объявления для каждого типа",
            "각 타입에 대해 별도의 선언 생성"
        ),
        code:        "extern \"C\" {\n    fn some_func_i32(x: i32);\n    fn some_func_i64(x: i64);\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: External blocks",
            url:   "https://doc.rust-lang.org/reference/items/external-blocks.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0044.html"
        }
    ]
};
