// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0377: CoerceUnsized may only be implemented between same struct

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0377",
    title:       LocalizedText::new(
        "CoerceUnsized only between same struct type",
        "CoerceUnsized только между одинаковыми struct типами",
        "CoerceUnsized는 동일한 구조체 타입 간에만 가능"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
CoerceUnsized or DispatchFromDyn was implemented between different struct types.
These traits can only be validly implemented between the same struct with
different generic parameters, not between entirely different structs.

Example: impl CoerceUnsized<Bar<U>> for Foo<T> is invalid because Foo and Bar
are different struct types. It should be Foo<T> to Foo<U>.",
        "\
CoerceUnsized или DispatchFromDyn был реализован между разными типами структур.
Эти трейты могут быть корректно реализованы только между одной и той же
структурой с разными параметрами, не между разными структурами.

Пример: impl CoerceUnsized<Bar<U>> for Foo<T> недопустим.",
        "\
CoerceUnsized 또는 DispatchFromDyn이 서로 다른 구조체 타입 간에 구현되었습니다.
이러한 트레이트는 완전히 다른 구조체 간이 아니라 다른 제네릭 매개변수를 가진
동일한 구조체 간에만 유효하게 구현될 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement between same struct with different type params",
                "Реализовать между одной структурой с разными параметрами",
                "다른 타입 매개변수를 가진 동일한 구조체 간에 구현"
            ),
            code:        "#![feature(coerce_unsized)]\nuse std::ops::CoerceUnsized;\n\nstruct Foo<T: ?Sized> { field: T }\n\nimpl<T, U> CoerceUnsized<Foo<U>> for Foo<T>\n    where T: CoerceUnsized<U> {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops::CoerceUnsized",
            url:   "https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0377.html"
        }
    ]
};
