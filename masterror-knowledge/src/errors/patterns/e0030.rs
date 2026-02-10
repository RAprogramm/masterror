// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0030: invalid range pattern (lower > upper)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0030",
    title:       LocalizedText::new(
        "Invalid range pattern",
        "Недопустимый диапазонный паттерн",
        "잘못된 범위 패턴"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when a range pattern has a start value greater than the
end value, making the range empty.

Example:
    match 5u32 {
        1000 ..= 5 => {}  // Error: 1000 > 5, empty range
    }",
        "\
Эта ошибка возникает, когда начальное значение диапазона больше конечного,
что делает диапазон пустым.",
        "\
이 오류는 범위 패턴의 시작 값이 끝 값보다 클 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Swap the range bounds",
            "Поменять границы диапазона местами",
            "범위 경계 교환"
        ),
        code:        "match 5u32 {\n    5 ..= 1000 => {}\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0030.html"
    }]
};
