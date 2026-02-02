// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0636: duplicate feature

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0636",
    title:       LocalizedText::new(
        "Feature enabled multiple times",
        "Функция включена несколько раз",
        "기능이 여러 번 활성화됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The same feature is enabled multiple times with `#![feature]` attributes.
The Rust compiler does not allow duplicate `#![feature]` declarations for
the same feature. Each feature should only be enabled once per crate.",
        "\
Одна и та же функция включена несколько раз с помощью атрибутов `#![feature]`.
Компилятор Rust не допускает дублирующиеся объявления `#![feature]` для одной
и той же функции. Каждая функция должна быть включена только один раз в крейте.",
        "\
동일한 기능이 `#![feature]` 속성으로 여러 번 활성화되었습니다.
각 기능은 크레이트당 한 번만 활성화해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove duplicate feature attribute",
            "Удалить дублирующийся атрибут функции",
            "중복 기능 속성 제거"
        ),
        code:        "#![feature(rust1)] // keep only one"
    }],
    links:       &[
        DocLink {
            title: "The Unstable Book",
            url:   "https://doc.rust-lang.org/unstable-book/"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0636.html"
        }
    ]
};
