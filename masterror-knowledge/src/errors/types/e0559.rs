// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0559: unknown field in enum struct variant

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0559",
    title:       LocalizedText::new(
        "Unknown field in enum struct variant",
        "Неизвестное поле в структурном варианте перечисления",
        "열거형 구조체 변형에 알 수 없는 필드"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unknown field was specified in an enum's struct variant. This error occurs
when you try to initialize a struct variant of an enum with a field name that
doesn't exist in that variant's definition.

Verify that you're using the correct field name as defined in the enum variant.",
        "\
Неизвестное поле было указано в структурном варианте перечисления.
Эта ошибка возникает при попытке инициализировать структурный вариант
перечисления с именем поля, которое не существует в определении варианта.",
        "\
열거형의 구조체 변형에 알 수 없는 필드가 지정되었습니다. 이 오류는
해당 변형의 정의에 존재하지 않는 필드 이름으로 열거형의 구조체 변형을
초기화하려고 할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use the correct field name",
            "Использовать правильное имя поля",
            "올바른 필드 이름 사용"
        ),
        code:        "enum Field { Fool { x: u32 } }\nlet s = Field::Fool { x: 0 }; // use 'x' not 'joke'"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0559.html"
    }]
};
