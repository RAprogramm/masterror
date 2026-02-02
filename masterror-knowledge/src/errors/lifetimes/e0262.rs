// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0262: invalid lifetime parameter name

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0262",
    title:       LocalizedText::new(
        "Invalid lifetime parameter name",
        "Недопустимое имя параметра времени жизни",
        "잘못된 수명 매개변수 이름"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
An invalid name was used for a lifetime parameter.

The name `'static` cannot be used as a named lifetime parameter because
it is a special built-in lifetime that denotes the lifetime of the
entire program. It has special meaning in Rust and is reserved, so it
cannot be redefined or used as a custom generic lifetime parameter.",
        "\
Для параметра времени жизни использовано недопустимое имя.

Имя `'static` нельзя использовать как именованный параметр времени жизни,
потому что это специальное встроенное время жизни, обозначающее время
жизни всей программы.",
        "\
수명 매개변수에 잘못된 이름이 사용되었습니다.
`'static`은 예약된 이름이므로 사용자 정의 수명 매개변수로 사용할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a valid custom lifetime name",
            "Используйте допустимое пользовательское имя времени жизни",
            "유효한 사용자 정의 수명 이름 사용"
        ),
        code:        "fn foo<'a>(x: &'a str) {}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Static Lifetime",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0262.html"
        }
    ]
};
