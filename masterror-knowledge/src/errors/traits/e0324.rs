// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0324: method implemented when another trait item expected

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0324",
    title:       LocalizedText::new(
        "Method implemented when another trait item expected",
        "Реализован метод вместо ожидаемого элемента трейта",
        "다른 트레이트 항목이 예상되었지만 메서드가 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A method was implemented when another trait item was expected.
This happens when you implement a trait item with the wrong kind of definition.

For example, if a trait declares N as a constant (const N: u32), but the
implementation defines it as a method (fn N() {}), this error occurs.",
        "\
Метод был реализован, когда ожидался другой элемент трейта.
Это происходит, когда вы реализуете элемент трейта с неправильным определением.

Например, если трейт объявляет N как константу (const N: u32), а реализация
определяет его как метод (fn N() {}), возникает эта ошибка.",
        "\
다른 트레이트 항목이 예상되었지만 메서드가 구현되었습니다.
트레이트 항목을 잘못된 종류의 정의로 구현할 때 발생합니다.

예를 들어, 트레이트가 N을 상수(const N: u32)로 선언했지만
구현이 메서드(fn N() {})로 정의하면 이 오류가 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Match the trait definition - use const for const, fn for fn",
                "Соответствовать определению трейта",
                "트레이트 정의에 맞추기 - const에는 const, fn에는 fn"
            ),
            code:        "trait Foo {\n    const N: u32;\n    fn M();\n}\n\nimpl Foo for Bar {\n    const N: u32 = 0; // const, not fn\n    fn M() {}\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Trait Implementations",
            url:   "https://doc.rust-lang.org/reference/items/implementations.html#trait-implementations"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0324.html"
        }
    ]
};
