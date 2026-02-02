// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0690: transparent struct with multiple non-zero-sized fields

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0690",
    title:       LocalizedText::new(
        "Transparent struct with multiple non-zero-sized fields",
        "Прозрачная структура с несколькими ненулевыми полями",
        "여러 개의 비영 크기 필드가 있는 투명 구조체"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A struct with `repr(transparent)` had two or more fields that were not
guaranteed to be zero-sized.

The `repr(transparent)` attribute requires that the struct be represented
exactly like one of its fields at runtime, so there can only be one
non-zero-sized field.",
        "\
Структура с `repr(transparent)` имела два или более полей, которые
не гарантированно имеют нулевой размер.

Атрибут `repr(transparent)` требует, чтобы структура была представлена
точно так же, как одно из её полей во время выполнения, поэтому может
быть только одно поле с ненулевым размером.",
        "\
`repr(transparent)` 구조체에 크기가 0이 아닌 것으로 보장되지 않는
필드가 두 개 이상 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use PhantomData for type parameters",
            "Использовать PhantomData для параметров типа",
            "타입 매개변수에 PhantomData 사용"
        ),
        code:        "use std::marker::PhantomData;\n\n#[repr(transparent)]\nstruct Wrapper<U> {\n    value: f32,\n    unit: PhantomData<U>, // zero-sized\n}"
    }],
    links:       &[
        DocLink {
            title: "repr(transparent)",
            url:   "https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0690.html"
        }
    ]
};
