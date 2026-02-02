// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0705: feature stable in current edition (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0705",
    title:       LocalizedText::new(
        "Feature stable in current edition",
        "Функция стабильна в текущей редакции",
        "현재 에디션에서 안정화된 기능"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

A `#![feature]` attribute was used for a feature that is stable in the
current edition. The feature gate is unnecessary since the feature is
already available.",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Атрибут `#![feature]` использован для функции, стабильной в текущей редакции.",
        "\
참고: 이 오류 코드는 더 이상 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove unnecessary feature gate",
            "Удалите ненужный feature gate",
            "불필요한 기능 게이트 제거"
        ),
        code:        "// Remove: #![feature(already_stable_feature)]"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0705.html"
    }]
};
