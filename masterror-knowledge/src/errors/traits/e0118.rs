// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0118: no nominal type found for inherent implementation

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0118",
    title:       LocalizedText::new(
        "No nominal type found for inherent implementation",
        "Номинальный тип не найден для собственной реализации",
        "고유 구현을 위한 명목적 타입을 찾을 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An inherent implementation was defined for something which isn't a struct,
enum, union, or trait object. Inherent implementations can only be created
for these specific nominal types.

You cannot define an inherent impl for generic type parameters like T.",
        "\
Собственная реализация была определена для чего-то, что не является
структурой, перечислением, объединением или трейт-объектом.
Собственные реализации можно создавать только для этих номинальных типов.

Нельзя определить собственную реализацию для параметров типа вроде T.",
        "\
구조체, 열거형, 공용체 또는 트레이트 객체가 아닌 것에 대해 고유 구현이
정의되었습니다. 고유 구현은 이러한 명목적 타입에만 생성할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement a trait instead",
                "Реализовать трейт вместо этого",
                "대신 트레이트 구현"
            ),
            code:        "trait MyTrait {\n    fn get_state(&self) -> String;\n}\n\nimpl<T> MyTrait for T {\n    fn get_state(&self) -> String {\n        \"state\".to_owned()\n    }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Create a newtype wrapper",
                "Создать тип-обёртку",
                "newtype 래퍼 생성"
            ),
            code:        "struct TypeWrapper<T>(T);\n\nimpl<T> TypeWrapper<T> {\n    fn get_state(&self) -> String {\n        \"state\".to_owned()\n    }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Implementations",
            url:   "https://doc.rust-lang.org/reference/items/implementations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0118.html"
        }
    ]
};
