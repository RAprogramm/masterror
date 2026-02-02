// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0631: type mismatch in closure arguments

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0631",
    title:       LocalizedText::new(
        "Type mismatch in closure arguments",
        "Несоответствие типов в аргументах замыкания",
        "클로저 인수의 타입 불일치"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A closure was passed to a function but the closure's argument types don't
match the expected types defined by the function's trait bounds.

The function expects a closure with specific argument types via a trait
like `Fn(T)`, but you passed a closure with different argument types.",
        "\
Замыкание было передано в функцию, но типы аргументов замыкания не
соответствуют ожидаемым типам, определённым ограничениями трейтов функции.

Функция ожидает замыкание с определёнными типами аргументов через трейт
типа `Fn(T)`, но вы передали замыкание с другими типами аргументов.",
        "\
클로저가 함수에 전달되었지만 클로저의 인수 타입이 함수의 트레이트 바운드에서
정의한 예상 타입과 일치하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Correct the closure's argument type",
                "Исправить тип аргумента замыкания",
                "클로저의 인수 타입 수정"
            ),
            code:        "fn foo<F: Fn(i32)>(f: F) {}\nfoo(|x: i32| {}); // correct type"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Let the type be inferred",
                "Позволить типу быть выведенным",
                "타입이 추론되도록 함"
            ),
            code:        "fn foo<F: Fn(i32)>(f: F) {}\nfoo(|x| {}); // type inferred as i32"
        }
    ],
    links:       &[
        DocLink {
            title: "Closures",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0631.html"
        }
    ]
};
