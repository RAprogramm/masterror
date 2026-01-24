// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0282",
    title:       LocalizedText::new(
        "Type annotations needed",
        "Требуются аннотации типа",
        "타입 어노테이션이 필요함"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "The compiler cannot infer the type. Provide an explicit type annotation.",
        "Компилятор не может вывести тип. Укажите явную аннотацию типа.",
        "컴파일러가 타입을 추론할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add type annotation",
            "Добавить аннотацию",
            "타입 어노테이션 추가"
        ),
        code:        "let numbers: Vec<i32> = input.parse().unwrap();"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0282.html"
    }]
};
