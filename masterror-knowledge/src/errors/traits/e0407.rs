// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0407: method not in trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0407",
    title:       LocalizedText::new(
        "Method not a member of trait",
        "Метод не является членом трейта",
        "메서드가 트레이트의 멤버가 아님"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A method was defined in a trait implementation that doesn't exist in the
trait definition itself. When implementing a trait, you can only define
methods that are declared in the trait.",
        "\
В реализации трейта определён метод, которого нет в определении трейта.
При реализации трейта можно определять только те методы, которые
объявлены в трейте.",
        "\
트레이트 구현에서 트레이트 정의에 존재하지 않는 메서드가 정의되었습니다.
트레이트를 구현할 때는 트레이트에 선언된 메서드만 정의할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add the method to the trait definition",
                "Добавить метод в определение трейта",
                "트레이트 정의에 메서드 추가"
            ),
            code:        "trait Foo {\n    fn a();\n    fn b(); // Add missing method\n}\n\nimpl Foo for Bar {\n    fn a() {}\n    fn b() {}\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Implement in a separate impl block",
                "Реализовать в отдельном блоке impl",
                "별도의 impl 블록에서 구현"
            ),
            code:        "impl Foo for Bar {\n    fn a() {}\n}\n\nimpl Bar {\n    fn b() {} // Separate impl for extra methods\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0407.html"
    }]
};
