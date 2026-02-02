// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0124: duplicate field name in struct

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0124",
    title:       LocalizedText::new(
        "Duplicate field name in struct",
        "Дублирующееся имя поля в структуре",
        "구조체에서 중복 필드 이름"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when a struct is declared with two fields having the same
name. Each field must have a unique name.

Example:
    struct Foo {
        field1: i32,
        field1: i32,  // Error: duplicate field name
    }",
        "\
Эта ошибка возникает, когда структура объявлена с двумя полями с одинаковым
именем. Каждое поле должно иметь уникальное имя.",
        "\
이 오류는 구조체에 같은 이름의 필드가 두 개 있을 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use unique field names",
            "Использовать уникальные имена полей",
            "고유한 필드 이름 사용"
        ),
        code:        "struct Foo {\n    field1: i32,\n    field2: i32,\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0124.html"
    }]
};
