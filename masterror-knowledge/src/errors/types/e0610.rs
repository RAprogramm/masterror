// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0610: primitive type has no fields

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0610",
    title:       LocalizedText::new(
        "Primitive type has no fields",
        "Примитивный тип не имеет полей",
        "원시 타입에는 필드가 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to access a field on a primitive type. Primitive types in Rust
(like `u32`, `i32`, `bool`, etc.) do not have fields and cannot be accessed
using dot notation for fields.

Only struct types support named field access.",
        "\
Попытка доступа к полю примитивного типа. Примитивные типы в Rust
(такие как `u32`, `i32`, `bool` и т.д.) не имеют полей и не могут быть
доступны с помощью точечной нотации для полей.

Только структуры поддерживают доступ к именованным полям.",
        "\
원시 타입에서 필드에 접근하려고 시도했습니다. Rust의 원시 타입에는
필드가 없으며 점 표기법으로 접근할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use struct types for named data",
            "Использовать структуры для именованных данных",
            "명명된 데이터에는 구조체 타입 사용"
        ),
        code:        "struct Point { x: u32, y: i64 }\nlet p = Point { x: 0, y: -12 };\nprintln!(\"{}\", p.x);"
    }],
    links:       &[
        DocLink {
            title: "Primitive Types",
            url:   "https://doc.rust-lang.org/book/ch03-02-data-types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0610.html"
        }
    ]
};
