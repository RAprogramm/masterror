// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0579: lower range not less than upper range

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0579",
    title:       LocalizedText::new(
        "Lower range wasn't less than upper range",
        "Нижняя граница диапазона не меньше верхней",
        "하한이 상한보다 작지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A lower range wasn't less than the upper range. When matching against an
exclusive range pattern, the compiler verifies that the range is non-empty.

Exclusive range patterns include the start point but exclude the end point,
which means the start of the range must be strictly less than the end.

For example, `5..5` is an empty range because 5 is not less than 5.",
        "\
Нижняя граница диапазона не была меньше верхней. При сопоставлении с
исключающим диапазоном компилятор проверяет, что диапазон не пуст.

Исключающие диапазоны включают начальную точку, но исключают конечную,
поэтому начало должно быть строго меньше конца.",
        "\
하한이 상한보다 작지 않았습니다. 배타적 범위 패턴과 매칭할 때 컴파일러는
범위가 비어 있지 않은지 확인합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Ensure range start is less than end",
            "Убедиться, что начало диапазона меньше конца",
            "범위 시작이 끝보다 작은지 확인"
        ),
        code:        "match 5u32 {\n    1..2 => {}\n    5..6 => {} // valid: 5 < 6\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0579.html"
    }]
};
