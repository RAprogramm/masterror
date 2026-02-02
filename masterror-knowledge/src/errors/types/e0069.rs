// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0069: return with no value in non-unit function

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0069",
    title:       LocalizedText::new(
        "Return with no value in non-unit function",
        "Return без значения в функции, возвращающей не ()",
        "non-unit 함수에서 값 없이 return"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when a function with a non-unit return type contains a bare
`return;` statement without a value. A bare `return;` is equivalent to
`return ();`, which doesn't match the expected return type.

Example:
    fn foo() -> u8 {
        return;  // Error: expected u8, returns ()
    }",
        "\
Эта ошибка возникает, когда функция с не-unit типом возврата содержит
голый `return;` без значения.",
        "\
이 오류는 non-unit 반환 타입을 가진 함수가 값 없이 `return;`을 포함할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Return a value of the correct type",
                "Вернуть значение правильного типа",
                "올바른 타입의 값 반환"
            ),
            code:        "fn foo() -> u8 {\n    return 5;\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Change return type to unit",
                "Изменить тип возврата на ()",
                "반환 타입을 unit으로 변경"
            ),
            code:        "fn foo() {\n    return;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0069.html"
    }]
};
