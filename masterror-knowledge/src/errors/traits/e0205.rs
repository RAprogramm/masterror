// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0205: Copy trait on enum with non-Copy variants

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0205",
    title:       LocalizedText::new(
        "Copy trait on enum with non-Copy variants",
        "Трейт Copy для enum с не-Copy вариантами",
        "non-Copy 변형을 가진 enum에 Copy 트레이트"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Cannot implement `Copy` trait for enum where one or more variants contain
types that do not implement `Copy`.

For an enum to be `Copy`, ALL of its variants must contain only `Copy` types.
If any variant holds a non-`Copy` type, the implementation will fail.

Note: This error code is no longer emitted by the compiler.",
        "\
Нельзя реализовать трейт `Copy` для enum, где один или более вариантов
содержат типы, не реализующие `Copy`.

Чтобы enum был `Copy`, ВСЕ его варианты должны содержать только
`Copy` типы.",
        "\
하나 이상의 변형이 `Copy`를 구현하지 않는 타입을 포함하는 enum에는
`Copy` 트레이트를 구현할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Ensure all variants contain Copy types",
                "Убедитесь, что все варианты содержат Copy типы",
                "모든 변형이 Copy 타입을 포함하는지 확인"
            ),
            code:        "#[derive(Copy, Clone)]\nenum Foo {\n    Bar(i32),\n    Baz(bool),\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use Clone instead of Copy",
                "Используйте Clone вместо Copy",
                "Copy 대신 Clone 사용"
            ),
            code:        "#[derive(Clone)]\nenum Foo {\n    Bar(Vec<u32>),\n    Baz,\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Copy",
            url:   "https://doc.rust-lang.org/std/marker/trait.Copy.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0205.html"
        }
    ]
};
