// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0556: malformed feature attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0556",
    title:       LocalizedText::new(
        "The `feature` attribute was malformed",
        "Атрибут `feature` был неправильно сформирован",
        "`feature` 속성이 잘못 형성됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `feature` attribute must be properly formatted. It only accepts feature
flag identifiers and can only be used on nightly Rust.

Invalid syntax includes:
- Parenthesized expressions: `foo(bar)`
- Assignment syntax: `foo = \"baz\"`
- Duplicate flags
- Empty attribute: `#![feature]`
- String assignment: `#![feature = \"foo\"]`",
        "\
Атрибут `feature` должен быть правильно отформатирован. Он принимает только
идентификаторы флагов функций и может использоваться только в nightly Rust.

Недопустимый синтаксис включает:
- Выражения в скобках: `foo(bar)`
- Синтаксис присваивания: `foo = \"baz\"`
- Дублирующиеся флаги",
        "\
`feature` 속성은 올바르게 형식화되어야 합니다. 기능 플래그 식별자만 허용하며
나이틀리 Rust에서만 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use correct feature attribute syntax",
            "Использовать правильный синтаксис атрибута feature",
            "올바른 feature 속성 구문 사용"
        ),
        code:        "#![feature(flag)]\n#![feature(flag1, flag2)] // multiple flags"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0556.html"
    }]
};
