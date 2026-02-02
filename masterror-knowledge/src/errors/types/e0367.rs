// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0367: Drop implemented on specialized generic type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0367",
    title:       LocalizedText::new(
        "Drop implemented on specialized generic type",
        "Drop реализован на специализированном обобщённом типе",
        "특수화된 제네릭 타입에 Drop이 구현됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Rust does not allow implementing Drop for a specialized version of a generic
type. Drop cannot be specialized to only apply to a subset of implementations
of a generic type.

Example: impl<T: Foo> Drop for MyStruct<T> {} is not allowed because it
specializes Drop only for T that implements Foo.",
        "\
Rust не позволяет реализовывать Drop для специализированной версии обобщённого
типа. Drop нельзя специализировать только для подмножества реализаций.

Пример: impl<T: Foo> Drop for MyStruct<T> {} не разрешён, потому что
специализирует Drop только для T, реализующих Foo.",
        "\
Rust는 제네릭 타입의 특수화된 버전에 대해 Drop을 구현하는 것을 허용하지 않습니다.
Drop은 제네릭 타입 구현의 하위 집합에만 적용되도록 특수화될 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add trait bound to struct definition",
                "Добавить ограничение трейта в определение структуры",
                "구조체 정의에 트레이트 바운드 추가"
            ),
            code:        "struct MyStruct<T: Foo> {\n    t: T\n}\n\nimpl<T: Foo> Drop for MyStruct<T> {\n    fn drop(&mut self) {}\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use wrapper struct with trait bounds",
                "Использовать обёртку с ограничениями трейтов",
                "트레이트 바운드가 있는 래퍼 구조체 사용"
            ),
            code:        "struct MyStructWrapper<T: Foo> {\n    t: MyStruct<T>\n}\n\nimpl<T: Foo> Drop for MyStructWrapper<T> {\n    fn drop(&mut self) {}\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Drop Trait",
            url:   "https://doc.rust-lang.org/std/ops/trait.Drop.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0367.html"
        }
    ]
};
