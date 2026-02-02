// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0026: nonexistent field in struct pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0026",
    title:       LocalizedText::new(
        "Nonexistent field in struct pattern",
        "Несуществующее поле в паттерне структуры",
        "구조체 패턴에 존재하지 않는 필드"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when a struct pattern tries to extract a field that doesn't
exist in the struct definition.

Example:
    struct Thing { x: u32, y: u32 }
    match thing {
        Thing { x, z } => {}  // Error: Thing has no field `z`
    }",
        "\
Эта ошибка возникает, когда паттерн структуры пытается извлечь поле,
которого нет в определении структуры.",
        "\
이 오류는 구조체 패턴이 구조체 정의에 없는 필드를 추출하려고 할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use field renaming syntax if needed",
            "Использовать переименование поля при необходимости",
            "필요한 경우 필드 이름 변경 구문 사용"
        ),
        code:        "match thing {\n    Thing { x, y: z } => {}  // renames y to z\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0026.html"
    }]
};
