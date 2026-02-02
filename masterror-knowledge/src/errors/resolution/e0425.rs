// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0425",
    title:       LocalizedText::new(
        "Cannot find value in this scope",
        "Не удаётся найти значение в этой области видимости",
        "이 스코프에서 값을 찾을 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "You're using a variable, function, or constant that doesn't exist in scope.",
        "Вы используете переменную, функцию или константу, которая не существует в текущей области.",
        "스코프에 존재하지 않는 변수, 함수 또는 상수를 사용하고 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Declare the variable",
            "Объявить переменную",
            "변수 선언"
        ),
        code:        "let x = 10;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0425.html"
    }]
};
