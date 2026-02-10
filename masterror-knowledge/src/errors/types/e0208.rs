// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0208: variance display (internal)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0208",
    title:       LocalizedText::new(
        "Type variance display (internal)",
        "Отображение вариантности типа (внутреннее)",
        "타입 변성 표시 (내부용)"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This is an internal compiler error that is no longer emitted in normal
Rust code. It was used to display the variance of a type's generic
parameters via the unstable `#[rustc_variance]` attribute.

Variance notation:
- `-` indicates contravariance
- `o` indicates invariance
- `+` indicates covariance

This attribute is only used internally for compiler testing.",
        "\
Это внутренняя ошибка компилятора, которая больше не выдаётся в
обычном коде Rust. Она использовалась для отображения вариантности
параметров типа через нестабильный атрибут `#[rustc_variance]`.",
        "\
이것은 더 이상 일반 Rust 코드에서 발생하지 않는 내부 컴파일러 오류입니다.
`#[rustc_variance]` 속성을 통해 타입의 변성을 표시하는 데 사용되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove #[rustc_variance] attribute",
            "Удалите атрибут #[rustc_variance]",
            "#[rustc_variance] 속성 제거"
        ),
        code:        "struct Foo<'a, T> {\n    t: &'a mut T,\n}"
    }],
    links:       &[
        DocLink {
            title: "Rustonomicon: Variance",
            url:   "https://doc.rust-lang.org/nomicon/subtyping.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0208.html"
        }
    ]
};
