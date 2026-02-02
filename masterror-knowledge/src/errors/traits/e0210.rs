// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0210: orphan rules violation

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0210",
    title:       LocalizedText::new(
        "Orphan rules violation for trait implementation",
        "Нарушение правил сирот при реализации трейта",
        "트레이트 구현의 고아 규칙 위반"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This error occurs when implementing a foreign trait for a foreign type
in a way that violates Rust's orphan rules.

Type parameters must be 'covered' by a local type when implementing
a foreign trait. A local type must appear BEFORE any use of type
parameters in the trait implementation.

For impl<P1, ..., Pm> ForeignTrait<T1, ..., Tn> for T0:
1. At least one of T0..=Tn must be a local type (Ti)
2. No uncovered type parameters may appear in T0..Ti (excluding Ti)",
        "\
Ошибка возникает при реализации внешнего трейта для внешнего типа
с нарушением правил сирот Rust.

Параметры типов должны быть 'покрыты' локальным типом.
Локальный тип должен появиться ДО использования параметров типов.",
        "\
외부 트레이트를 외부 타입에 구현할 때 Rust의 고아 규칙을 위반하면
이 오류가 발생합니다. 로컬 타입이 타입 매개변수보다 먼저 나타나야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Wrap the type parameter in a local type",
                "Обернуть параметр типа в локальный тип",
                "타입 매개변수를 로컬 타입으로 래핑"
            ),
            code:        "struct MyType<T>(T);\nimpl<T> ForeignTrait for MyType<T> { }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Put local type first in trait parameters",
                "Поставить локальный тип первым в параметрах трейта",
                "트레이트 매개변수에서 로컬 타입을 먼저 배치"
            ),
            code:        "impl<T> ForeignTrait2<MyType<T>, T> for MyType2 { }"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Orphan Rule",
            url:   "https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0210.html"
        }
    ]
};
