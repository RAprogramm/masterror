// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0706: async fn in trait (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0706",
    title:       LocalizedText::new(
        "Async fn not supported in traits",
        "Async fn не поддерживается в трейтах",
        "트레이트에서 async fn 지원되지 않음"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

Previously, `async fn`s were not supported in traits because they return
`impl Future`, which requires Generic Associated Types (GATs).

Modern Rust now supports async functions in traits natively.",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Ранее `async fn` не поддерживались в трейтах, так как они возвращают
`impl Future`, что требовало GATs. Современный Rust поддерживает это.",
        "\
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다.

이전에는 `async fn`이 트레이트에서 지원되지 않았습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use native async trait (modern Rust)",
                "Используйте встроенную поддержку async trait",
                "네이티브 async 트레이트 사용"
            ),
            code:        "trait MyTrait {\n    async fn foo(&self);\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use async-trait crate (legacy)",
                "Используйте крейт async-trait",
                "async-trait 크레이트 사용"
            ),
            code:        "#[async_trait]\ntrait MyTrait {\n    async fn foo(&self);\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0706.html"
        }
    ]
};
