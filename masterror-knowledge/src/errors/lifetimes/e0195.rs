// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0195: lifetime parameters or bounds on method do not match the trait
//! declaration

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0195",
    title:       LocalizedText::new(
        "Lifetime parameters or bounds on method do not match the trait declaration",
        "Параметры времени жизни или ограничения метода не соответствуют объявлению трейта",
        "메서드의 라이프타임 매개변수 또는 바운드가 트레이트 선언과 일치하지 않음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The lifetime parameters of the method do not match the trait declaration.
When implementing a trait method with lifetime parameters, the lifetime
bounds and constraints must be identical between the trait definition
and its implementation.",
        "\
Параметры времени жизни метода не соответствуют объявлению трейта.
При реализации метода трейта с параметрами времени жизни, ограничения
времени жизни должны быть идентичны между определением трейта и его
реализацией.",
        "\
메서드의 라이프타임 매개변수가 트레이트 선언과 일치하지 않습니다.
라이프타임 매개변수가 있는 트레이트 메서드를 구현할 때 라이프타임
바운드와 제약 조건은 트레이트 정의와 구현 사이에서 동일해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match lifetime declarations and bounds exactly",
            "Точно соответствовать объявлениям и ограничениям времени жизни",
            "라이프타임 선언과 바운드를 정확히 일치시키기"
        ),
        code:        "trait Trait {\n    fn t<'a,'b:'a>(x: &'a str, y: &'b str);\n}\n\nstruct Foo;\n\nimpl Trait for Foo {\n    fn t<'a,'b:'a>(x: &'a str, y: &'b str) { // ok!\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Lifetime Annotations",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0195.html"
        }
    ]
};
