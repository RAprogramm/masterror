// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0434: cannot capture dynamic environment in fn item

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0434",
    title:       LocalizedText::new(
        "Cannot capture dynamic environment in fn item",
        "Нельзя захватить динамическое окружение во вложенной функции",
        "fn 항목에서 동적 환경을 캡처할 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An inner function tried to access a variable from its containing scope.
Rust's inner functions cannot capture variables from their dynamic
environment like closures can. Inner functions are essentially treated
as top-level items.",
        "\
Вложенная функция попыталась использовать переменную из внешней
области видимости. Вложенные функции в Rust не могут захватывать
переменные из динамического окружения, как это делают замыкания.",
        "\
내부 함수가 포함하는 스코프의 변수에 접근하려고 시도했습니다.
Rust의 내부 함수는 클로저처럼 동적 환경에서 변수를 캡처할 수
없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a closure instead",
                "Использовать замыкание",
                "대신 클로저 사용"
            ),
            code:        "fn foo() {\n    let y = 5;\n    let bar = || { y }; // Closure captures y\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use const or static items",
                "Использовать const или static",
                "const 또는 static 항목 사용"
            ),
            code:        "fn foo() {\n    const Y: u32 = 5;\n    fn bar() -> u32 { Y } // Can access const\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0434.html"
    }]
};
