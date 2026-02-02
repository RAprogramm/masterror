// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0370: enum discriminant overflow

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0370",
    title:       LocalizedText::new(
        "Enum discriminant overflow",
        "Переполнение дискриминанта перечисления",
        "열거형 판별자 오버플로우"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The maximum value of an enum was reached, so it cannot be automatically set
in the next enum value. When an enum variant's discriminant reaches the maximum
value for its representation type, the compiler cannot automatically assign
the next sequential value to the following variant.

Example: #[repr(i64)] enum with X = 0x7fffffffffffffff cannot have Y after it
without explicit value.",
        "\
Достигнуто максимальное значение enum, поэтому следующее значение не может
быть установлено автоматически. Когда дискриминант варианта достигает
максимального значения для типа представления, компилятор не может
автоматически назначить следующее последовательное значение.",
        "\
열거형의 최대값에 도달하여 다음 열거형 값을 자동으로 설정할 수 없습니다.
열거형 변형의 판별자가 표현 타입의 최대값에 도달하면, 컴파일러는
다음 변형에 순차적 값을 자동으로 할당할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly set the next enum value",
                "Явно установить следующее значение enum",
                "다음 열거형 값을 명시적으로 설정"
            ),
            code:        "#[repr(i64)]\nenum Foo {\n    X = 0x7fffffffffffffff,\n    Y = 0, // explicit value\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Place maximum value variant at the end",
                "Поместить вариант с максимальным значением в конец",
                "최대값 변형을 끝에 배치"
            ),
            code:        "#[repr(i64)]\nenum Foo {\n    Y = 0,\n    X = 0x7fffffffffffffff, // last variant\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Enumerations",
            url:   "https://doc.rust-lang.org/reference/items/enumerations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0370.html"
        }
    ]
};
