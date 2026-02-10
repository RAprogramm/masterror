// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0077: SIMD element must be a machine type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0077",
    title:       LocalizedText::new(
        "SIMD element must be a machine type",
        "Элемент SIMD должен быть машинным типом",
        "SIMD 요소는 기계 타입이어야 함"
    ),
    category:    Category::Simd,
    explanation: LocalizedText::new(
        "\
When using `#[repr(simd)]`, all array elements must be machine types that
can be directly operated on by SIMD instructions (like u32, f64, i16).

Example:
    #[repr(simd)]
    struct Bad([String; 2]);  // Error: String is not a machine type",
        "\
При использовании `#[repr(simd)]` все элементы массива должны быть машинными
типами, которые могут напрямую обрабатываться SIMD-инструкциями.",
        "\
`#[repr(simd)]`를 사용할 때 모든 배열 요소는 SIMD 명령어로 직접 작동할 수 있는 기계 타입이어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use primitive machine types",
            "Использовать примитивные машинные типы",
            "원시 기계 타입 사용"
        ),
        code:        "#[repr(simd)]\nstruct Good([u32; 4]);  // u32 is a machine type"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0077.html"
    }]
};
