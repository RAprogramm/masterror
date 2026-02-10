// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0214: incorrect generic type syntax

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0214",
    title:       LocalizedText::new(
        "Generic type described with parentheses instead of angle brackets",
        "Обобщённый тип описан круглыми скобками вместо угловых",
        "제네릭 타입이 꺾쇠괄호 대신 괄호로 기술됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Generic type parameter specified using parentheses rather than angle brackets.

Incorrect syntax: Vec(&str)
Correct syntax: Vec<&str>

Parentheses are ONLY used with generic types when defining parameters
for `Fn`-family traits (like `Fn()`, `FnMut()`, `FnOnce()`). For all
other generic types, angle brackets must be used.",
        "\
Параметр обобщённого типа указан с использованием круглых скобок
вместо угловых.

Неправильно: Vec(&str)
Правильно: Vec<&str>

Круглые скобки используются ТОЛЬКО для трейтов семейства Fn.",
        "\
제네릭 타입 매개변수가 꺾쇠괄호 대신 괄호로 지정되었습니다.
괄호는 Fn 계열 트레이트에만 사용됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use angle brackets for generic types",
            "Используйте угловые скобки для обобщённых типов",
            "제네릭 타입에 꺾쇠괄호 사용"
        ),
        code:        "let v: Vec<&str> = vec![\"foo\"];"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Generic Types",
            url:   "https://doc.rust-lang.org/book/ch10-01-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0214.html"
        }
    ]
};
