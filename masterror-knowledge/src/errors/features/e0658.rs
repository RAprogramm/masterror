// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0658: unstable feature used

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0658",
    title:       LocalizedText::new(
        "Unstable feature used",
        "Использована нестабильная функция",
        "불안정한 기능 사용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unstable feature was used without properly enabling it. Unstable features
are experimental functionality that may change or be removed in future Rust
versions.

Unstable features require:
- The nightly version of Rust
- Explicit opt-in via `#![feature(...)]` attribute",
        "\
Использована нестабильная функция без её правильного включения. Нестабильные
функции — это экспериментальная функциональность, которая может измениться
или быть удалена в будущих версиях Rust.

Нестабильные функции требуют:
- Ночную версию Rust
- Явное включение через атрибут `#![feature(...)]`",
        "\
불안정한 기능이 적절한 활성화 없이 사용되었습니다. 불안정한 기능은
향후 Rust 버전에서 변경되거나 제거될 수 있는 실험적 기능입니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Enable the feature with #![feature(...)]",
                "Включить функцию с помощью #![feature(...)]",
                "#![feature(...)]로 기능 활성화"
            ),
            code:        "#![feature(core_intrinsics)]\n\nuse std::intrinsics; // ok!"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Switch to nightly Rust",
                "Переключиться на ночную версию Rust",
                "nightly Rust로 전환"
            ),
            code:        "rustup default nightly"
        }
    ],
    links:       &[
        DocLink {
            title: "The Unstable Book",
            url:   "https://doc.rust-lang.org/unstable-book/"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0658.html"
        }
    ]
};
