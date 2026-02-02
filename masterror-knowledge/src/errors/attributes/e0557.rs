// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0557: feature has been removed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0557",
    title:       LocalizedText::new(
        "Feature has been removed",
        "Функция была удалена",
        "기능이 제거됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A feature attribute named a feature that has been removed from Rust. Feature
gates are unstable Rust features that may be removed in future versions.

When a feature is removed, any code using `#![feature(...)]` with that feature
name will fail to compile.",
        "\
Атрибут feature указал функцию, которая была удалена из Rust. Feature gates -
это нестабильные функции Rust, которые могут быть удалены в будущих версиях.

Когда функция удаляется, любой код, использующий `#![feature(...)]` с этим
именем функции, не скомпилируется.",
        "\
기능 속성이 Rust에서 제거된 기능을 명명했습니다. 기능 게이트는 향후 버전에서
제거될 수 있는 불안정한 Rust 기능입니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove the obsolete feature attribute",
            "Удалить устаревший атрибут feature",
            "사용되지 않는 기능 속성 제거"
        ),
        code:        "// Remove: #![feature(managed_boxes)]\n// This feature no longer exists"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0557.html"
    }]
};
