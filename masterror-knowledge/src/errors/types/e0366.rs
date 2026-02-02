// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0366: Drop implemented on concrete specialization of generic type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0366",
    title:       LocalizedText::new(
        "Drop on concrete specialization of generic type",
        "Drop на конкретной специализации обобщённого типа",
        "제네릭 타입의 구체적 특수화에 대한 Drop"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Rust does not allow implementing Drop for a specific concrete type that is a
specialization of a generic struct. This would violate coherence rules - you
cannot specialize Drop to only work for a subset of implementations of a
generic type.

Example: impl Drop for Foo<u32> {} is not allowed when Foo<T> is generic.",
        "\
Rust не позволяет реализовывать Drop для конкретного типа, который является
специализацией обобщённой структуры. Это нарушило бы правила когерентности -
нельзя специализировать Drop только для подмножества реализаций обобщённого типа.

Пример: impl Drop for Foo<u32> {} не разрешён, когда Foo<T> обобщённый.",
        "\
Rust는 제네릭 구조체의 특수화인 특정 구체적 타입에 대해 Drop을 구현하는 것을
허용하지 않습니다. 이는 일관성 규칙을 위반합니다 - 제네릭 타입 구현의 하위 집합에
대해서만 Drop을 특수화할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Wrap in non-generic struct and implement Drop on wrapper",
                "Обернуть в необобщённую структуру и реализовать Drop",
                "비제네릭 구조체로 감싸고 래퍼에 Drop 구현"
            ),
            code:        "struct Bar {\n    t: Foo<u32>\n}\n\nimpl Drop for Bar {\n    fn drop(&mut self) {}\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Drop Trait",
            url:   "https://doc.rust-lang.org/std/ops/trait.Drop.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0366.html"
        }
    ]
};
