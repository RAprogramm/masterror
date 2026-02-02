// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0554: feature attributes require nightly

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0554",
    title:       LocalizedText::new(
        "Feature attributes require nightly compiler",
        "Атрибуты feature требуют nightly компилятор",
        "기능 속성은 나이틀리 컴파일러 필요"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Feature attributes (using `#![feature(...)]`) can only be used when compiling
with the Rust nightly compiler. Stable or beta compilers will reject code
containing feature attributes, as they are experimental features that may
change or be removed in future releases.

This error enforces Rust's stability guarantee by preventing unstable features
from being used in code compiled with stable or beta toolchains.",
        "\
Атрибуты feature (с использованием `#![feature(...)]`) могут использоваться
только при компиляции с nightly компилятором Rust. Стабильные или бета
компиляторы отклонят код с атрибутами feature, так как это экспериментальные
функции, которые могут измениться или быть удалены.",
        "\
기능 속성(`#![feature(...)]` 사용)은 Rust 나이틀리 컴파일러로 컴파일할 때만
사용할 수 있습니다. 안정 또는 베타 컴파일러는 기능 속성이 포함된 코드를
거부합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Switch to nightly Rust to use unstable features",
                "Переключиться на nightly Rust для использования нестабильных функций",
                "불안정 기능을 사용하려면 나이틀리 Rust로 전환"
            ),
            code:        "// Run: rustup default nightly\n// Or: rustup run nightly cargo build"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the feature attribute for stable Rust",
                "Удалить атрибут feature для стабильного Rust",
                "안정 Rust를 위해 기능 속성 제거"
            ),
            code:        "// Remove: #![feature(lang_items)]\n// Use stable alternatives instead"
        }
    ],
    links:       &[
        DocLink {
            title: "Nightly Rust",
            url:   "https://doc.rust-lang.org/book/appendix-07-nightly-rust.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0554.html"
        }
    ]
};
