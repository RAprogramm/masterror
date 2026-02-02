// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0325: associated type implemented when const expected

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0325",
    title:       LocalizedText::new(
        "Associated type implemented when const expected",
        "Реализован тип вместо ожидаемой константы",
        "상수가 예상되었지만 연관 타입이 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated type was implemented in a trait implementation, but the trait
definition expects a constant. The item name exists in the trait, but it's
defined as a const, not a type.

Verify that the trait item name isn't misspelled and ensure your implementation
matches the exact kind of item defined in the trait.",
        "\
Ассоциированный тип был реализован в реализации трейта, но определение трейта
ожидает константу. Имя элемента существует в трейте, но определено как const,
а не type.

Проверьте правильность написания имени и соответствие типа элемента.",
        "\
연관 타입이 트레이트 구현에서 구현되었지만, 트레이트 정의는 상수를 예상합니다.
항목 이름이 트레이트에 존재하지만 type이 아닌 const로 정의되어 있습니다.

트레이트 항목 이름의 철자가 맞는지 확인하고 구현이 트레이트에 정의된
정확한 종류의 항목과 일치하는지 확인하세요."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use const instead of type if trait expects const",
                "Использовать const вместо type",
                "트레이트가 const를 예상하면 type 대신 const 사용"
            ),
            code:        "trait Foo {\n    const N: u32;\n}\n\nimpl Foo for Bar {\n    const N: u32 = 0; // not type N = u32\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Associated Constants",
            url:   "https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0325.html"
        }
    ]
};
