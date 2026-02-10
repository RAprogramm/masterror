// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0670: async fn not permitted in Rust 2015

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0670",
    title:       LocalizedText::new(
        "async fn not permitted in Rust 2015",
        "async fn не разрешён в Rust 2015",
        "Rust 2015에서는 async fn이 허용되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Rust 2015 does not permit the use of `async fn`. The `async fn` feature
was not available in the 2015 edition of Rust.

The `async fn` syntax and async/await functionality became available
starting with Rust 2018 edition.",
        "\
Rust 2015 не допускает использование `async fn`. Функция `async fn` не
была доступна в редакции Rust 2015.

Синтаксис `async fn` и функциональность async/await стали доступны
начиная с редакции Rust 2018.",
        "\
Rust 2015에서는 `async fn` 사용이 허용되지 않습니다. `async fn`
기능은 Rust 2015 에디션에서 사용할 수 없었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Switch to Rust 2018 or later edition",
            "Переключиться на Rust 2018 или более позднюю редакцию",
            "Rust 2018 이상 에디션으로 전환"
        ),
        code:        "# In Cargo.toml:\n[package]\nedition = \"2021\""
    }],
    links:       &[
        DocLink {
            title: "Editions",
            url:   "https://doc.rust-lang.org/edition-guide/"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0670.html"
        }
    ]
};
