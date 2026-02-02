// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0040: explicit destructor call

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0040",
    title:       LocalizedText::new(
        "Explicit destructor call",
        "Явный вызов деструктора",
        "명시적 소멸자 호출"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Rust does not allow explicit calls to the `drop()` method. The `Drop` trait's
destructor is called automatically when a value goes out of scope.

Example:
    let x = MyType::new();
    x.drop();  // Error: explicit use of destructor method",
        "\
Rust не позволяет явно вызывать метод `drop()`. Деструктор трейта Drop
вызывается автоматически, когда значение выходит из области видимости.",
        "\
Rust는 `drop()` 메서드를 명시적으로 호출하는 것을 허용하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use std::mem::drop() function",
            "Использовать функцию std::mem::drop()",
            "std::mem::drop() 함수 사용"
        ),
        code:        "let x = MyType::new();\ndrop(x);  // Takes ownership and drops"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Drop Trait",
            url:   "https://doc.rust-lang.org/book/ch15-03-drop.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0040.html"
        }
    ]
};
