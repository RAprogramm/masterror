// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0075: SIMD struct must have single array field

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0075",
    title:       LocalizedText::new(
        "SIMD struct must have single array field",
        "SIMD структура должна иметь одно поле-массив",
        "SIMD 구조체는 단일 배열 필드를 가져야 함"
    ),
    category:    Category::Simd,
    explanation: LocalizedText::new(
        "\
The `#[repr(simd)]` attribute can only be applied to structs with exactly one
field. Empty structs and structs with multiple fields are not allowed.

Example:
    #[repr(simd)]
    struct Bad;  // Error: empty struct

    #[repr(simd)]
    struct Bad2([u32; 1], [u32; 1]);  // Error: multiple fields",
        "\
Атрибут `#[repr(simd)]` можно применять только к структурам с ровно одним
полем. Пустые структуры и структуры с несколькими полями не допускаются.",
        "\
`#[repr(simd)]` 속성은 정확히 하나의 필드를 가진 구조체에만 적용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a single array field",
            "Использовать одно поле-массив",
            "단일 배열 필드 사용"
        ),
        code:        "#[repr(simd)]\nstruct Good([u32; 4]);"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0075.html"
    }]
};
