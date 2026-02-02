// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0642: patterns not allowed in trait methods

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0642",
    title:       LocalizedText::new(
        "Patterns not allowed in trait methods",
        "Шаблоны не допускаются в методах трейтов",
        "트레이트 메서드에서 패턴 허용되지 않음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Trait methods currently cannot take patterns as arguments. Rust does not
allow pattern matching in trait method signatures.

While you can use destructuring patterns in regular function parameters,
trait method declarations must use single parameter names with their
full types.",
        "\
Методы трейтов в настоящее время не могут принимать шаблоны в качестве
аргументов. Rust не допускает сопоставление с шаблоном в сигнатурах
методов трейтов.

Хотя вы можете использовать деструктурирующие шаблоны в параметрах
обычных функций, объявления методов трейтов должны использовать
одиночные имена параметров с их полными типами.",
        "\
트레이트 메서드는 현재 인수로 패턴을 받을 수 없습니다. Rust는 트레이트
메서드 시그니처에서 패턴 매칭을 허용하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use single parameter name with full type",
            "Использовать одиночное имя параметра с полным типом",
            "전체 타입과 함께 단일 매개변수 이름 사용"
        ),
        code:        "trait Foo {\n    fn foo(x_and_y: (i32, i32)); // ok\n}"
    }],
    links:       &[
        DocLink {
            title: "Traits",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0642.html"
        }
    ]
};
