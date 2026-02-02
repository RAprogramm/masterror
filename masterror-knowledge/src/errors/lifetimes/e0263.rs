// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0263: duplicate lifetime declaration

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0263",
    title:       LocalizedText::new(
        "Duplicate lifetime declaration",
        "Дублирующееся объявление времени жизни",
        "중복된 수명 선언"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A lifetime was declared more than once in the same scope.

A lifetime parameter cannot have the same name as another lifetime
parameter within the same function or generic declaration.

Note: This error code is no longer emitted by the compiler.",
        "\
Время жизни было объявлено более одного раза в той же области видимости.

Параметр времени жизни не может иметь то же имя, что и другой параметр
времени жизни в той же функции или обобщённом объявлении.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
수명이 같은 범위에서 두 번 이상 선언되었습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename duplicate lifetime to unique identifier",
            "Переименуйте дублирующееся время жизни в уникальный идентификатор",
            "중복된 수명을 고유 식별자로 이름 변경"
        ),
        code:        "fn foo<'a, 'b, 'c>(x: &'a str, y: &'b str, z: &'c str) {}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Lifetimes",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0263.html"
        }
    ]
};
