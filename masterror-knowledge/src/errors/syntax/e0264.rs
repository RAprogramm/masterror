// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0264: unknown external lang item

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0264",
    title:       LocalizedText::new(
        "Unknown external lang item",
        "Неизвестный внешний элемент языка",
        "알 수 없는 외부 lang 항목"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
An unknown external lang item was used.

The Rust compiler has a predefined set of valid external language items
that can be declared with the `#[lang]` attribute. If you try to use
a lang item name that doesn't exist in this set, the compiler will
reject it.

The complete list of available external lang items can be found in
`compiler/rustc_hir/src/weak_lang_items.rs` in the Rust source code.",
        "\
Использован неизвестный внешний элемент языка.

Компилятор Rust имеет предопределённый набор допустимых внешних
элементов языка, которые можно объявить с атрибутом `#[lang]`.",
        "\
알 수 없는 외부 lang 항목이 사용되었습니다.
`#[lang]` 속성으로 선언할 수 있는 유효한 항목만 사용해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use valid external lang item",
            "Используйте допустимый внешний элемент языка",
            "유효한 외부 lang 항목 사용"
        ),
        code:        "#[lang = \"panic_impl\"]  // valid lang item\nfn panic() {}"
    }],
    links:       &[
        DocLink {
            title: "Rust Internals: Lang Items",
            url:   "https://doc.rust-lang.org/unstable-book/language-features/lang-items.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0264.html"
        }
    ]
};
