// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0742: visibility restricted to non-ancestor module

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0742",
    title:       LocalizedText::new(
        "Visibility restricted to non-ancestor module",
        "Видимость ограничена модулем, не являющимся предком",
        "조상이 아닌 모듈로 가시성 제한"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
Visibility is restricted to a module which isn't an ancestor of the current item.

The `pub(in path)` syntax can only restrict visibility to an ancestor module,
not to sibling or unrelated modules.",
        "\
Видимость ограничена модулем, который не является предком текущего элемента.

Синтаксис `pub(in path)` может ограничивать видимость только предком.",
        "\
현재 항목의 조상이 아닌 모듈로 가시성이 제한되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Move item inside the target module",
            "Переместите элемент внутрь целевого модуля",
            "대상 모듈 내부로 항목 이동"
        ),
        code:        "pub mod earth {\n    pub mod sea {\n        pub(in crate::earth) struct Shark; // ok\n    }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0742.html"
    }]
};
