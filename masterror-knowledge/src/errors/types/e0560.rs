// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0560: unknown field in struct

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0560",
    title:       LocalizedText::new(
        "Unknown field specified in struct",
        "Неизвестное поле указано в структуре",
        "구조체에 알 수 없는 필드 지정"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unknown field was specified in a structure. This error occurs when you try
to initialize a struct with a field name that doesn't exist in the struct
definition. The compiler cannot find the specified field.

Verify that the field name is spelled correctly and actually exists in the
struct definition.",
        "\
Неизвестное поле было указано в структуре. Эта ошибка возникает при попытке
инициализировать структуру с именем поля, которое не существует в определении
структуры. Компилятор не может найти указанное поле.",
        "\
구조체에 알 수 없는 필드가 지정되었습니다. 이 오류는 구조체 정의에
존재하지 않는 필드 이름으로 구조체를 초기화하려고 할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add the missing field to struct definition",
                "Добавить отсутствующее поле в определение структуры",
                "구조체 정의에 누락된 필드 추가"
            ),
            code:        "struct Simba {\n    mother: u32,\n    father: u32, // add missing field\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the erroneous field from initialization",
                "Удалить ошибочное поле из инициализации",
                "초기화에서 잘못된 필드 제거"
            ),
            code:        "let s = Simba { mother: 1 }; // remove non-existent field"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0560.html"
    }]
};
