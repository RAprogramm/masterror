// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0323: associated const implemented when type expected

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0323",
    title:       LocalizedText::new(
        "Associated const implemented when type expected",
        "Реализована константа вместо ожидаемого типа",
        "타입이 예상되었지만 연관 상수가 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated constant (const) was implemented in a trait implementation,
but the trait definition expects a different kind of associated item
(typically an associated type).

The item name exists in the trait, but it's defined as a type, not a const.",
        "\
Ассоциированная константа (const) была реализована в реализации трейта,
но определение трейта ожидает другой вид ассоциированного элемента
(обычно ассоциированный тип).

Имя элемента существует в трейте, но определено как тип, а не константа.",
        "\
연관 상수(const)가 트레이트 구현에서 구현되었지만, 트레이트 정의는
다른 종류의 연관 항목(일반적으로 연관 타입)을 예상합니다.

항목 이름이 트레이트에 존재하지만 const가 아닌 type으로 정의되어 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use type instead of const if trait expects type",
                "Использовать type вместо const",
                "트레이트가 타입을 예상하면 const 대신 type 사용"
            ),
            code:        "trait Foo {\n    type N;\n}\n\nimpl Foo for Bar {\n    type N = u32; // not const N\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Associated Types",
            url:   "https://doc.rust-lang.org/reference/items/associated-items.html#associated-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0323.html"
        }
    ]
};
