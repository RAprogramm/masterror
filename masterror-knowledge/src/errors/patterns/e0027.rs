// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0027: pattern missing struct fields

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0027",
    title:       LocalizedText::new(
        "Pattern missing struct fields",
        "В паттерне отсутствуют поля структуры",
        "패턴에서 구조체 필드 누락"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when a pattern for a struct fails to specify a sub-pattern
for every field of the struct.

Example:
    struct Dog { name: String, age: u32 }
    match dog {
        Dog { age: x } => {}  // Error: missing field `name`
    }",
        "\
Эта ошибка возникает, когда паттерн для структуры не указывает подпаттерн
для каждого поля структуры.",
        "\
이 오류는 구조체 패턴이 모든 필드에 대한 하위 패턴을 지정하지 않을 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Specify all fields",
                "Указать все поля",
                "모든 필드 지정"
            ),
            code:        "match dog {\n    Dog { name: ref n, age: x } => {}\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use .. to ignore remaining fields",
                "Использовать .. для игнорирования остальных полей",
                ".. 를 사용하여 나머지 필드 무시"
            ),
            code:        "match dog {\n    Dog { age: x, .. } => {}\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0027.html"
    }]
};
