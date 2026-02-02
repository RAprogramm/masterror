// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0184: the Copy trait was implemented on a type with a Drop implementation

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0184",
    title:       LocalizedText::new(
        "The Copy trait was implemented on a type with a Drop implementation",
        "Трейт Copy реализован для типа с реализацией Drop",
        "Drop 구현이 있는 타입에 Copy 트레이트가 구현됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Explicitly implementing both Drop and Copy traits on a type is currently
disallowed. While this might make sense in theory, the current implementation
is incorrect and can lead to memory unsafety, so it has been disabled.

A type that implements Copy is bitwise-copied, meaning Drop would never be
called, leading to resource leaks or double-frees.",
        "\
Явная реализация обоих трейтов Drop и Copy для одного типа запрещена.
Хотя это может иметь смысл в теории, текущая реализация некорректна и
может привести к небезопасности памяти.

Тип, реализующий Copy, копируется побитово, что означает, что Drop
никогда не будет вызван.",
        "\
현재 타입에 Drop과 Copy 트레이트를 모두 명시적으로 구현하는 것은
허용되지 않습니다. 이론적으로는 의미가 있을 수 있지만 현재 구현은
올바르지 않으며 메모리 안전성 문제를 일으킬 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove either Copy or Drop implementation",
            "Удалить реализацию Copy или Drop",
            "Copy 또는 Drop 구현 제거"
        ),
        code:        "// Choose one:\n\n// Option 1: Keep Copy, remove Drop\n#[derive(Copy, Clone)]\nstruct Foo;\n\n// Option 2: Keep Drop, remove Copy\nstruct Bar;\n\nimpl Drop for Bar {\n    fn drop(&mut self) {}\n}"
    }],
    links:       &[
        DocLink {
            title: "Issue #20126",
            url:   "https://github.com/rust-lang/rust/issues/20126"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0184.html"
        }
    ]
};
