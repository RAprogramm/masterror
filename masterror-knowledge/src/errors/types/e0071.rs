// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0071: struct literal used for non-struct type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0071",
    title:       LocalizedText::new(
        "Struct literal used for non-struct type",
        "Синтаксис структуры для не-структурного типа",
        "비구조체 타입에 구조체 리터럴 사용"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when struct literal syntax `{ field: value }` is used with
a type that isn't a struct, enum variant, or union.

Example:
    type U32 = u32;
    let t = U32 { value: 4 };  // Error: u32 is not a struct",
        "\
Эта ошибка возникает при использовании синтаксиса структурного литерала
с типом, который не является структурой, вариантом enum или union.",
        "\
이 오류는 구조체, 열거형 변형 또는 공용체가 아닌 타입에 구조체 리터럴 구문을 사용할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use correct initialization syntax",
                "Использовать правильный синтаксис инициализации",
                "올바른 초기화 구문 사용"
            ),
            code:        "type U32 = u32;\nlet t: U32 = 4;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Define an actual struct",
                "Определить настоящую структуру",
                "실제 구조체 정의"
            ),
            code:        "struct U32 { value: u32 }\nlet t = U32 { value: 4 };"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0071.html"
    }]
};
