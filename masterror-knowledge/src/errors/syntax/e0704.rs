// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0704: incorrect visibility restriction

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0704",
    title:       LocalizedText::new(
        "Incorrect visibility restriction",
        "Неправильное ограничение видимости",
        "잘못된 가시성 제한"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
An incorrect visibility restriction syntax was used.

When trying to restrict visibility to a module, the `in` keyword must be used
with the `pub` keyword: `pub(in path)` instead of `pub(module_name)`.",
        "\
Использован неправильный синтаксис ограничения видимости.

При ограничении видимости модулем нужно использовать `in`:
`pub(in path)` вместо `pub(module_name)`.",
        "\
잘못된 가시성 제한 구문이 사용되었습니다.

모듈에 가시성을 제한할 때 `in` 키워드를 사용해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use pub(in path) syntax",
            "Используйте синтаксис pub(in path)",
            "pub(in path) 구문 사용"
        ),
        code:        "mod foo {\n    pub(in crate::foo) struct Bar {\n        x: i32\n    }\n}"
    }],
    links:       &[DocLink {
        title: "Rust Reference: Visibility",
        url:   "https://doc.rust-lang.org/reference/visibility-and-privacy.html"
    }]
};
