// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0029: range pattern with non-numeric/char type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0029",
    title:       LocalizedText::new(
        "Range pattern with non-numeric type",
        "Диапазонный паттерн с нечисловым типом",
        "숫자가 아닌 타입의 범위 패턴"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when you use a range pattern with types other than numbers
or characters. Range patterns only work with numeric types and `char`.

Example:
    match string {
        \"hello\" ..= \"world\" => {}  // Error: strings don't support ranges
    }",
        "\
Эта ошибка возникает при использовании диапазонного паттерна с типами,
отличными от чисел или символов.",
        "\
이 오류는 숫자나 문자가 아닌 타입으로 범위 패턴을 사용할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a guard clause instead",
            "Использовать условие вместо диапазона",
            "대신 가드 절 사용"
        ),
        code:        "match string {\n    s if s >= \"hello\" && s <= \"world\" => {},\n    _ => {},\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0029.html"
    }]
};
