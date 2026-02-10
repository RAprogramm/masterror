// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0316: nested quantification over lifetimes in where clause

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0316",
    title:       LocalizedText::new(
        "Nested quantification over lifetimes",
        "Вложенная квантификация по временам жизни",
        "라이프타임에 대한 중첩 한정"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A where clause contains a nested quantification over lifetimes, which is not
supported in Rust. Rust syntax allows lifetime quantifications in two places:
1. Quantifying over the trait bound only: Ty: for<'l> Trait<'l>
2. Quantifying over the whole clause: for<'l> &'l Ty: Trait<'l>

However, using both in the same clause leads to nested lifetime quantification.",
        "\
Where-клауза содержит вложенную квантификацию по временам жизни, которая
не поддерживается в Rust. Синтаксис Rust позволяет квантификацию в двух местах:
1. Квантификация только по trait bound: Ty: for<'l> Trait<'l>
2. Квантификация по всей клаузе: for<'l> &'l Ty: Trait<'l>

Использование обоих приводит к вложенной квантификации.",
        "\
where 절에 중첩된 라이프타임 한정이 포함되어 있으며, 이는 Rust에서 지원되지 않습니다.
Rust 문법은 두 위치에서 라이프타임 한정을 허용합니다:
1. 트레이트 바운드만 한정: Ty: for<'l> Trait<'l>
2. 전체 절 한정: for<'l> &'l Ty: Trait<'l>"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Combine lifetime parameters in single for<>",
                "Объединить параметры времени жизни в один for<>",
                "단일 for<>에서 라이프타임 매개변수 결합"
            ),
            code:        "fn foo<T>(t: T)\nwhere\n    for<'a, 'b> &'a T: Tr<'a, 'b>,\n{\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Higher-Ranked Trait Bounds",
            url:   "https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0316.html"
        }
    ]
};
