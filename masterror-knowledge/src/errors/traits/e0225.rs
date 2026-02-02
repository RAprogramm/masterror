// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0225: multiple non-auto trait bounds

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0225",
    title:       LocalizedText::new(
        "Multiple non-auto traits in trait object",
        "Несколько не-auto трейтов в трейт-объекте",
        "트레이트 객체에 여러 non-auto 트레이트"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Multiple types were used as bounds for a closure or trait object.
Rust does not currently support using multiple non-auto trait types
as bounds for a trait object.

You can only have ONE non-builtin trait as a bound for a trait object.
However, you CAN add any number of auto traits (Send, Sync, etc.)
in addition to that single trait.",
        "\
Несколько типов были использованы как ограничения для замыкания или
трейт-объекта. Rust не поддерживает множественные не-auto трейты
в качестве ограничений для трейт-объекта.

Можно использовать только ОДИН не встроенный трейт, но можно добавить
любое количество auto-трейтов (Send, Sync и т.д.).",
        "\
클로저나 트레이트 객체에 여러 타입이 바운드로 사용되었습니다.
트레이트 객체에는 하나의 non-auto 트레이트만 바운드로 사용할 수 있습니다.
단, auto 트레이트(Send, Sync 등)는 추가로 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use one non-auto trait with auto traits",
            "Используйте один не-auto трейт с auto трейтами",
            "하나의 non-auto 트레이트와 auto 트레이트 사용"
        ),
        code:        "let _: Box<dyn std::io::Read + Send + Sync>;"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Trait Objects",
            url:   "https://doc.rust-lang.org/book/ch17-02-trait-objects.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0225.html"
        }
    ]
};
