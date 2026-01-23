// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0621: explicit lifetime required in the type of X

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0621",
    title:       LocalizedText::new(
        "Explicit lifetime required in the type",
        "Требуется явное время жизни в типе",
        "타입에 명시적 라이프타임이 필요함"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The compiler cannot infer lifetimes in this context. You need to add
explicit lifetime annotations to show how references relate.",
        "\
Компилятор не может вывести времена жизни в этом контексте.",
        "\
컴파일러가 이 컨텍스트에서 라이프타임을 추론할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add lifetime parameter to function",
            "Добавить параметр времени жизни к функции",
            "함수에 라이프타임 매개변수 추가"
        ),
        code:        "fn process<'a>(data: &'a str) -> &'a str { data }"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0621.html"
    }]
};
