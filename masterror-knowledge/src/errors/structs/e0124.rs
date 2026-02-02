// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0124: field is already declared

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0124",
    title:       LocalizedText::new(
        "Field is already declared",
        "Поле уже объявлено",
        "필드가 이미 선언됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A struct was declared with two fields having the same name.
Rust does not allow multiple fields in a struct to share the same identifier.

Verify that all field names are correctly spelled and unique.",
        "\
Структура была объявлена с двумя полями с одинаковым именем.
Rust не позволяет нескольким полям структуры иметь одинаковый идентификатор.

Убедитесь, что все имена полей написаны правильно и уникальны.",
        "\
동일한 이름을 가진 두 개의 필드로 구조체가 선언되었습니다.
Rust는 구조체의 여러 필드가 동일한 식별자를 공유하는 것을 허용하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename one of the duplicate fields",
            "Переименовать одно из дублирующихся полей",
            "중복된 필드 중 하나의 이름 변경"
        ),
        code:        "struct Foo {\n    field1: i32,\n    field2: i32, // not field1: i32\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Defining Structs",
            url:   "https://doc.rust-lang.org/book/ch05-01-defining-structs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0124.html"
        }
    ]
};
