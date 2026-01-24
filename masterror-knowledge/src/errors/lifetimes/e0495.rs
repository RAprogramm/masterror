// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0495: cannot infer an appropriate lifetime

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0495",
    title:       LocalizedText::new(
        "Cannot infer an appropriate lifetime",
        "Невозможно вывести подходящее время жизни",
        "적절한 라이프타임을 추론할 수 없음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The compiler found conflicting lifetime requirements and couldn't
determine which one to use.",
        "\
Компилятор обнаружил конфликтующие требования времён жизни.",
        "\
컴파일러가 충돌하는 라이프타임 요구사항을 찾았습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add explicit lifetime bounds",
            "Добавить явные ограничения времени жизни",
            "명시적 라이프타임 바운드 추가"
        ),
        code:        "fn process<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0495.html"
    }]
};
