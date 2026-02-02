// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0206: Copy trait on invalid type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0206",
    title:       LocalizedText::new(
        "Copy trait implemented on invalid type",
        "Трейт Copy реализован для недопустимого типа",
        "잘못된 타입에 Copy 트레이트 구현"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
The `Copy` trait was implemented on a type which is neither a struct,
an enum, nor a union.

The `Copy` trait can only be derived or manually implemented for these
three composite types. You cannot implement `Copy` for references,
primitives, or other types.",
        "\
Трейт `Copy` был реализован для типа, который не является ни структурой,
ни перечислением, ни объединением.

Трейт `Copy` может быть реализован только для этих трёх составных типов.",
        "\
`Copy` 트레이트가 struct, enum, union이 아닌 타입에 구현되었습니다.
`Copy`는 이 세 가지 복합 타입에만 구현할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Implement Copy only on struct, enum, or union",
            "Реализуйте Copy только для struct, enum или union",
            "struct, enum 또는 union에만 Copy 구현"
        ),
        code:        "#[derive(Copy, Clone)]\nstruct Bar;\n\n// Don't do: impl Copy for &'static mut Bar {}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Copy",
            url:   "https://doc.rust-lang.org/std/marker/trait.Copy.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0206.html"
        }
    ]
};
