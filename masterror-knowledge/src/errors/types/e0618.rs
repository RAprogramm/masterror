// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0618: expected function, found non-callable

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0618",
    title:       LocalizedText::new(
        "Expected function, found non-callable type",
        "Ожидалась функция, найден невызываемый тип",
        "함수가 예상되었으나 호출 불가능한 타입 발견"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to call something which isn't a function nor a method.
Only functions and methods can be invoked with parentheses.

Common mistakes:
- Calling an enum variant as a function
- Using `()` on a non-callable value like an integer",
        "\
Попытка вызвать что-то, что не является функцией или методом.
Только функции и методы могут быть вызваны с помощью скобок.

Распространённые ошибки:
- Вызов варианта enum как функции
- Использование `()` на невызываемом значении, например целом числе",
        "\
함수나 메서드가 아닌 것을 호출하려고 시도했습니다.
함수와 메서드만 괄호로 호출할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Only call actual functions or methods",
            "Вызывать только реальные функции или методы",
            "실제 함수나 메서드만 호출"
        ),
        code:        "fn my_function() {}\nmy_function(); // ok"
    }],
    links:       &[
        DocLink {
            title: "Functions",
            url:   "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0618.html"
        }
    ]
};
