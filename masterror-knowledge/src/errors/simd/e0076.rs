// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0076: SIMD field must be an array

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0076",
    title:       LocalizedText::new(
        "SIMD field must be an array",
        "Поле SIMD должно быть массивом",
        "SIMD 필드는 배열이어야 함"
    ),
    category:    Category::Simd,
    explanation: LocalizedText::new(
        "\
When using `#[repr(simd)]` on a tuple struct, the field type must be an array.
This is required to represent SIMD vector lanes.

Example:
    #[repr(simd)]
    struct Bad(u16);  // Error: not an array",
        "\
При использовании `#[repr(simd)]` на tuple struct тип поля должен быть массивом.
Это необходимо для представления SIMD-векторных дорожек.",
        "\
튜플 구조체에 `#[repr(simd)]`를 사용할 때 필드 타입은 배열이어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Wrap in array notation",
            "Обернуть в нотацию массива",
            "배열 표기법으로 감싸기"
        ),
        code:        "#[repr(simd)]\nstruct Good([u16; 1]);  // Single-lane vector"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0076.html"
    }]
};
