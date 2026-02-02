// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0119: conflicting implementations of trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0119",
    title:       LocalizedText::new(
        "Conflicting implementations of trait",
        "Конфликтующие реализации трейта",
        "트레이트의 충돌하는 구현"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
There are multiple conflicting trait implementations for the same type.
Rust cannot allow a trait to be implemented more than once for a given type.

A common scenario is when you implement a trait for a generic type T
(covering all types) and then try to implement the same trait for a
specific concrete type. This creates a conflict because the concrete
implementation overlaps with the generic implementation.",
        "\
Существует несколько конфликтующих реализаций трейта для одного типа.
Rust не позволяет реализовать трейт более одного раза для данного типа.

Типичный сценарий - когда вы реализуете трейт для обобщённого типа T
(покрывающего все типы), а затем пытаетесь реализовать тот же трейт
для конкретного типа. Это создаёт конфликт, так как конкретная
реализация пересекается с обобщённой.",
        "\
동일한 타입에 대해 여러 개의 충돌하는 트레이트 구현이 있습니다.
Rust는 주어진 타입에 대해 트레이트를 두 번 이상 구현하는 것을 허용하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the conflicting implementation",
                "Удалить конфликтующую реализацию",
                "충돌하는 구현 제거"
            ),
            code:        "trait MyTrait {\n    fn get(&self) -> usize;\n}\n\n// Keep only one implementation\nimpl<T> MyTrait for T {\n    fn get(&self) -> usize { 0 }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use specialization (nightly only)",
                "Использовать специализацию (только nightly)",
                "특수화 사용 (nightly만 해당)"
            ),
            code:        "#![feature(specialization)]\n\nimpl<T> MyTrait for T {\n    default fn get(&self) -> usize { 0 }\n}\n\nimpl MyTrait for Foo {\n    fn get(&self) -> usize { self.value }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Traits",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0119.html"
        }
    ]
};
