// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0203: duplicate relaxed bounds

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0203",
    title:       LocalizedText::new(
        "Duplicate relaxed default bounds",
        "Дублирующиеся ослабленные ограничения по умолчанию",
        "중복된 완화된 기본 바운드"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Having duplicate relaxed default bounds is unsupported.

The `?Sized` bound is a relaxed bound that removes the default `Sized`
requirement from a type parameter. Using `?Sized` more than once on
the same type parameter is redundant and not allowed.",
        "\
Наличие дублирующихся ослабленных ограничений по умолчанию не поддерживается.

Ограничение `?Sized` убирает требование `Sized` по умолчанию.
Использование `?Sized` более одного раза для одного параметра типа
избыточно и не допускается.",
        "\
중복된 완화된 기본 바운드는 지원되지 않습니다.
`?Sized` 바운드는 한 번만 사용해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove duplicate ?Sized bound",
            "Удалите дублирующееся ограничение ?Sized",
            "중복 ?Sized 바운드 제거"
        ),
        code:        "struct Good<T: ?Sized> {\n    inner: T,\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Sized Trait",
            url:   "https://doc.rust-lang.org/std/marker/trait.Sized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0203.html"
        }
    ]
};
