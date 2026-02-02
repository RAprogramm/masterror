// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0381: borrow of possibly-uninitialized variable

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0381",
    title:       LocalizedText::new(
        "Borrow of possibly-uninitialized variable",
        "Заимствование возможно неинициализированной переменной",
        "초기화되지 않았을 수 있는 변수의 빌림"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
Rust requires all variables to be initialized before use. You're trying
to use a variable that might not have been assigned a value yet.

This prevents reading garbage memory. The compiler tracks initialization
through all possible code paths.",
        "\
Rust требует инициализации всех переменных перед использованием.
Вы пытаетесь использовать переменную, которая может быть не инициализирована.

Это предотвращает чтение мусора из памяти.",
        "\
Rust는 사용 전에 모든 변수를 초기화해야 합니다.
아직 값이 할당되지 않았을 수 있는 변수를 사용하려고 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Initialize the variable",
                "Инициализировать переменную",
                "변수 초기화"
            ),
            code:        "let x = 0; // or any default value"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use Option for maybe-uninitialized",
                "Использовать Option для возможно неинициализированных",
                "초기화되지 않을 수 있는 경우 Option 사용"
            ),
            code:        "let x: Option<i32> = None;\nif condition { x = Some(42); }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0381.html"
    }]
};
