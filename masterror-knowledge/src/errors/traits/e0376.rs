// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0376: CoerceUnsized implemented between non-struct types

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0376",
    title:       LocalizedText::new(
        "CoerceUnsized implemented between non-struct types",
        "CoerceUnsized реализован между не-struct типами",
        "비구조체 타입 간에 CoerceUnsized가 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
CoerceUnsized or DispatchFromDyn was implemented between types that are not
structs. These traits can only be implemented between structs.

Both the source and target types of the implementation must be structs.
This error occurs when trying to implement these traits for generic type
parameters, references, or other non-struct types.",
        "\
CoerceUnsized или DispatchFromDyn был реализован между типами, которые не
являются структурами. Эти трейты могут быть реализованы только между структурами.

И исходный, и целевой типы реализации должны быть структурами.",
        "\
CoerceUnsized 또는 DispatchFromDyn이 구조체가 아닌 타입 간에 구현되었습니다.
이러한 트레이트는 구조체 간에만 구현될 수 있습니다.

구현의 소스 타입과 대상 타입 모두 구조체여야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement only between struct types",
                "Реализовать только между struct типами",
                "구조체 타입 간에만 구현"
            ),
            code:        "#![feature(coerce_unsized)]\nuse std::ops::CoerceUnsized;\n\nstruct Foo<T: ?Sized> { a: T }\nstruct Bar<T: ?Sized> { a: T }\n\n// impl<T, U> CoerceUnsized<U> for Foo<T> {} // Error: U is not a struct\nimpl<T, U> CoerceUnsized<Foo<U>> for Foo<T>\n    where T: CoerceUnsized<U> {} // OK: both are structs"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops::CoerceUnsized",
            url:   "https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0376.html"
        }
    ]
};
