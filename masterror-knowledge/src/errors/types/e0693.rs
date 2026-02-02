// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0693: incorrect repr(align) syntax

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0693",
    title:       LocalizedText::new(
        "Incorrect repr(align) syntax",
        "Неправильный синтаксис repr(align)",
        "잘못된 repr(align) 구문"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `align` representation hint was declared using incorrect syntax.

Common mistakes include:
- Using `=` instead of parentheses: `#[repr(align=8)]`
- Using string quotes: `#[repr(align=\"8\")]`",
        "\
Подсказка представления `align` была объявлена с использованием
неправильного синтаксиса.

Распространённые ошибки:
- Использование `=` вместо скобок: `#[repr(align=8)]`
- Использование строковых кавычек: `#[repr(align=\"8\")]`",
        "\
`align` 표현 힌트가 잘못된 구문으로 선언되었습니다.

일반적인 실수:
- 괄호 대신 `=` 사용
- 문자열 따옴표 사용"
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use correct syntax with parentheses",
            "Использовать правильный синтаксис со скобками",
            "괄호가 있는 올바른 구문 사용"
        ),
        code:        "#[repr(align(8))]\nstruct Align8(i8);"
    }],
    links:       &[
        DocLink {
            title: "Type Layout",
            url:   "https://doc.rust-lang.org/reference/type-layout.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0693.html"
        }
    ]
};
