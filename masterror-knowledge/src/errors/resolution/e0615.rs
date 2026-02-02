// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0615: attempted to access method like a field

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0615",
    title:       LocalizedText::new(
        "Attempted to access a method like a field",
        "Попытка доступа к методу как к полю",
        "메서드를 필드처럼 접근하려고 시도"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
Attempted to access a method without calling it (i.e., without parentheses),
treating it as if it were a field or property of a struct.

Methods must be called with parentheses `()` to execute them, while fields
are accessed directly by name.",
        "\
Попытка доступа к методу без его вызова (т.е. без скобок), обращаясь с ним
как с полем или свойством структуры.

Методы должны вызываться со скобками `()` для их выполнения, тогда как
к полям доступ осуществляется напрямую по имени.",
        "\
메서드를 호출하지 않고(즉, 괄호 없이) 접근하려고 시도했습니다.
메서드는 실행하려면 괄호 `()`로 호출해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Call the method with parentheses",
            "Вызвать метод со скобками",
            "괄호로 메서드 호출"
        ),
        code:        "f.method(); // call with parentheses"
    }],
    links:       &[
        DocLink {
            title: "Method Syntax",
            url:   "https://doc.rust-lang.org/book/ch05-03-method-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0615.html"
        }
    ]
};
