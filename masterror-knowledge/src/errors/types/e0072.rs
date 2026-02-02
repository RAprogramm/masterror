// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0072: recursive type has infinite size

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0072",
    title:       LocalizedText::new(
        "Recursive type has infinite size",
        "Рекурсивный тип имеет бесконечный размер",
        "재귀 타입이 무한 크기를 가짐"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when a struct or enum contains itself directly without
indirection, making it impossible to compute its size.

Example:
    struct Node {
        value: i32,
        next: Option<Node>,  // Error: infinite size
    }

Rust needs to know the size of types at compile time.",
        "\
Эта ошибка возникает, когда структура или enum содержит себя напрямую
без косвенности, что делает невозможным вычисление её размера.",
        "\
이 오류는 구조체나 열거형이 간접 참조 없이 자신을 직접 포함하여 크기를 계산할 수 없을 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use Box for indirection",
            "Использовать Box для косвенности",
            "간접 참조를 위해 Box 사용"
        ),
        code:        "struct Node {\n    value: i32,\n    next: Option<Box<Node>>,  // Box has known size\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Using Box for Recursive Types",
            url:   "https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0072.html"
        }
    ]
};
