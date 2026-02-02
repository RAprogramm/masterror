// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0004: non-exhaustive patterns in match expression

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0004",
    title:       LocalizedText::new(
        "Non-exhaustive patterns in match",
        "Неполное покрытие вариантов в match",
        "match에서 패턴이 완전하지 않음"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when a `match` expression doesn't cover all possible values
of the input type. The compiler requires that match expressions handle every
possible case to guarantee a value can be assigned.

Example:
    enum Color { Red, Green, Blue }
    match color {
        Color::Red => {},
        Color::Green => {},
        // missing Color::Blue!
    }",
        "\
Эта ошибка возникает, когда выражение `match` не покрывает все возможные
значения входного типа. Компилятор требует, чтобы match обрабатывал каждый
возможный случай.",
        "\
이 오류는 `match` 표현식이 입력 타입의 모든 가능한 값을 다루지 않을 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Cover all enum variants explicitly",
                "Явно указать все варианты enum",
                "모든 열거형 변형을 명시적으로 처리"
            ),
            code:        "match color {\n    Color::Red => {},\n    Color::Green => {},\n    Color::Blue => {},\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use wildcard pattern to catch remaining cases",
                "Использовать шаблон _ для остальных случаев",
                "와일드카드 패턴으로 나머지 케이스 처리"
            ),
            code:        "match color {\n    Color::Red => {},\n    _ => {},\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Patterns and Matching",
            url:   "https://doc.rust-lang.org/book/ch18-00-patterns.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0004.html"
        }
    ]
};
