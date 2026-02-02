// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0307: invalid receiver type for self parameter

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0307",
    title:       LocalizedText::new(
        "Invalid receiver type for self parameter",
        "Недопустимый тип получателя для параметра self",
        "self 매개변수에 대한 잘못된 수신자 타입"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Methods in Rust take a special first parameter called the receiver.
Valid receiver types are:
- self (syntactic sugar for self: Self)
- &self (syntactic sugar for self: &Self)
- &mut self (syntactic sugar for self: &mut Self)
- self: Box<Self>, self: Rc<Self>, self: Arc<Self>
- self: Pin<P> where P is one of the above types

The error occurs when using an invalid receiver type that doesn't match
these patterns.",
        "\
Методы в Rust принимают специальный первый параметр - получатель (receiver).
Допустимые типы получателя:
- self (сахар для self: Self)
- &self (сахар для self: &Self)
- &mut self (сахар для self: &mut Self)
- self: Box<Self>, self: Rc<Self>, self: Arc<Self>

Ошибка возникает при использовании недопустимого типа получателя.",
        "\
Rust의 메서드는 수신자라는 특별한 첫 번째 매개변수를 받습니다.
유효한 수신자 타입:
- self (self: Self의 문법적 설탕)
- &self (self: &Self의 문법적 설탕)
- &mut self (self: &mut Self의 문법적 설탕)
- self: Box<Self>, self: Rc<Self>, self: Arc<Self>"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a valid receiver type",
                "Использовать допустимый тип получателя",
                "유효한 수신자 타입 사용"
            ),
            code:        "impl Trait for Foo {\n    fn foo(&self) {} // or &mut self, self, etc.\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Method Receivers",
            url:   "https://doc.rust-lang.org/reference/items/associated-items.html#methods"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0307.html"
        }
    ]
};
