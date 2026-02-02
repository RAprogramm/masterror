// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0084: repr on zero-variant enum

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0084",
    title:       LocalizedText::new(
        "Repr on zero-variant enum",
        "Repr для enum без вариантов",
        "변형이 없는 열거형에 repr"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when an integer representation attribute is applied to a
zero-variant enum. Zero-variant enums cannot be instantiated, so there's no
value to represent.

Example:
    #[repr(i32)]
    enum Empty {}  // Error: zero-variant enum",
        "\
Эта ошибка возникает при применении атрибута целочисленного представления
к enum без вариантов. Такие enum не могут быть инстанцированы.",
        "\
이 오류는 변형이 없는 열거형에 정수 표현 속성을 적용할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add variants to the enum",
                "Добавить варианты в enum",
                "열거형에 변형 추가"
            ),
            code:        "#[repr(i32)]\nenum NotEmpty {\n    First,\n    Second,\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the repr attribute",
                "Удалить атрибут repr",
                "repr 속성 제거"
            ),
            code:        "enum Empty {}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0084.html"
    }]
};
