// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0424: self used without self receiver

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0424",
    title:       LocalizedText::new(
        "self keyword used in function without self parameter",
        "Ключевое слово self использовано в функции без параметра self",
        "self 매개변수 없는 함수에서 self 키워드 사용됨"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The self keyword was used inside an associated function that doesn't have
a self receiver parameter. The self keyword is only valid in methods -
associated functions that have self as their first parameter.

Methods require a self receiver: self, &self, &mut self, or self: &mut Pin<Self>.",
        "\
Ключевое слово self использовано внутри ассоциированной функции без
параметра self. Ключевое слово self допустимо только в методах -
ассоциированных функциях с self в качестве первого параметра.

Методы требуют параметр self: self, &self, &mut self или self: &mut Pin<Self>.",
        "\
self 수신자 매개변수가 없는 연관 함수 내에서 self 키워드가 사용되었습니다.
self 키워드는 메서드에서만 유효합니다 - self를 첫 번째 매개변수로 가지는
연관 함수입니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add self receiver to make it a method",
            "Добавить параметр self для создания метода",
            "메서드로 만들기 위해 self 수신자 추가"
        ),
        code:        "impl Foo {\n    fn foo(&self) {\n        self.bar(); // Now self is valid\n    }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0424.html"
    }]
};
