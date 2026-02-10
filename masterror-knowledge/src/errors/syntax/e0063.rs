// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0063: missing struct field

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0063",
    title:       LocalizedText::new(
        "Missing struct field",
        "Отсутствует поле структуры",
        "구조체 필드 누락"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when a struct is instantiated without providing values for
all of its fields. Every field must be initialized.

Example:
    struct Foo { x: i32, y: i32 }
    let f = Foo { x: 0 };  // Error: missing field `y`",
        "\
Эта ошибка возникает при создании экземпляра структуры без предоставления
значений для всех её полей. Каждое поле должно быть инициализировано.",
        "\
이 오류는 모든 필드에 값을 제공하지 않고 구조체를 인스턴스화할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Provide all required fields",
                "Указать все обязательные поля",
                "모든 필수 필드 제공"
            ),
            code:        "struct Foo { x: i32, y: i32 }\nlet f = Foo { x: 0, y: 0 };"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use struct update syntax with Default",
                "Использовать синтаксис обновления структуры с Default",
                "Default와 구조체 업데이트 구문 사용"
            ),
            code:        "#[derive(Default)]\nstruct Foo { x: i32, y: i32 }\nlet f = Foo { x: 0, ..Default::default() };"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Structs",
            url:   "https://doc.rust-lang.org/book/ch05-01-defining-structs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0063.html"
        }
    ]
};
