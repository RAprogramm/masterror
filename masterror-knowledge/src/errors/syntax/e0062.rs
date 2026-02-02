// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0062: duplicate field in struct initializer

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0062",
    title:       LocalizedText::new(
        "Duplicate field in struct initializer",
        "Дублирование поля в инициализаторе структуры",
        "구조체 이니셜라이저에서 중복 필드"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when a struct field is specified more than once during
initialization. Each field must be assigned exactly once.

Example:
    struct Foo { x: i32 }
    let f = Foo {
        x: 0,
        x: 0,  // Error: field `x` specified twice
    };",
        "\
Эта ошибка возникает, когда поле структуры указано более одного раза
при инициализации. Каждое поле должно быть присвоено ровно один раз.",
        "\
이 오류는 구조체 초기화 중 필드가 두 번 이상 지정될 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove duplicate field assignment",
            "Удалить дублирующееся присваивание поля",
            "중복 필드 할당 제거"
        ),
        code:        "struct Foo { x: i32 }\nlet f = Foo { x: 0 };"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0062.html"
    }]
};
