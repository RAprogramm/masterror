// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0383: partial reinitialization of uninitialized structure

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0383",
    title:       LocalizedText::new(
        "Partial reinitialization of uninitialized structure",
        "Частичная переинициализация неинициализированной структуры",
        "초기화되지 않은 구조체의 부분 재초기화"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
You're trying to partially reinitialize a struct that was moved from.
After a move, the entire struct is invalid - you can't assign to just
one field.

You must reinitialize the entire struct.",
        "\
Вы пытаетесь частично переинициализировать структуру после перемещения.
После перемещения вся структура недействительна - нельзя присвоить
только одно поле.

Нужно переинициализировать всю структуру.",
        "\
이동된 구조체를 부분적으로 재초기화하려고 합니다.
이동 후 전체 구조체가 무효화됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Reinitialize the entire struct",
            "Переинициализировать всю структуру",
            "전체 구조체 재초기화"
        ),
        code:        "s = MyStruct { field1: val1, field2: val2 };"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0383.html"
    }]
};
