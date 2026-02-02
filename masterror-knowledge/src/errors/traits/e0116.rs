// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0116: cannot define inherent impl for a type outside of the crate

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0116",
    title:       LocalizedText::new(
        "Cannot define inherent impl for a type outside of the crate",
        "Нельзя определить собственную реализацию для типа из другого крейта",
        "다른 크레이트의 타입에 대한 고유 구현을 정의할 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An inherent implementation was defined for a type outside the current crate.
Rust's orphan rules only allow you to implement methods directly on types
that you own (defined in your crate).

Note: Using type aliases does not work around this restriction.",
        "\
Собственная реализация была определена для типа из другого крейта.
Правила сирот Rust позволяют реализовывать методы только для типов,
определённых в вашем крейте.

Примечание: использование псевдонимов типов не обходит это ограничение.",
        "\
현재 크레이트 외부의 타입에 대한 고유 구현이 정의되었습니다.
Rust의 고아 규칙은 자신의 크레이트에서 정의된 타입에만 메서드를
직접 구현할 수 있도록 허용합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Define and implement a trait for the type",
                "Определить и реализовать трейт для типа",
                "타입에 대한 트레이트 정의 및 구현"
            ),
            code:        "trait MyTrait {\n    fn my_method(&self);\n}\nimpl MyTrait for Vec<u8> {\n    fn my_method(&self) { }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Create a wrapper type (newtype pattern)",
                "Создать тип-обёртку (паттерн newtype)",
                "래퍼 타입 생성 (newtype 패턴)"
            ),
            code:        "struct MyBytes(Vec<u8>);\nimpl MyBytes {\n    fn my_method(&self) { }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Newtype Pattern",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0116.html"
        }
    ]
};
