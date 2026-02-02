// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0379: trait method declared const

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0379",
    title:       LocalizedText::new(
        "Trait method cannot be declared const",
        "Метод трейта не может быть объявлен const",
        "트레이트 메서드는 const로 선언할 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Trait methods cannot be declared as const by design. This is a fundamental
restriction in Rust's type system.

The const qualifier is not allowed on trait method declarations or their
implementations. For details, see RFC 911.",
        "\
Методы трейтов не могут быть объявлены как const по дизайну. Это фундаментальное
ограничение в системе типов Rust.

Квалификатор const не разрешён в объявлениях методов трейтов или их реализациях.",
        "\
트레이트 메서드는 설계상 const로 선언할 수 없습니다. 이는 Rust 타입 시스템의
근본적인 제한입니다.

const 한정자는 트레이트 메서드 선언이나 그 구현에 허용되지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove const keyword from trait method",
                "Удалить const из метода трейта",
                "트레이트 메서드에서 const 키워드 제거"
            ),
            code:        "trait Foo {\n    fn bar() -> u32; // not const fn\n}\n\nimpl Foo for () {\n    fn bar() -> u32 { 0 }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 911: const_fn",
            url:   "https://github.com/rust-lang/rfcs/pull/911"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0379.html"
        }
    ]
};
