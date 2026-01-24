// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0308",
    title:       LocalizedText::new("Mismatched types", "Несовпадение типов", "타입 불일치"),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "Rust is statically typed and does NOT perform implicit type conversions.",
        "Rust статически типизирован и НЕ выполняет неявные преобразования типов.",
        "Rust는 정적 타입 언어이며 암시적 타입 변환을 수행하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new("Use parse()", "Использовать parse()", "parse() 사용"),
            code:        "let n: i32 = s.parse().unwrap();"
        },
        FixSuggestion {
            description: LocalizedText::new("Use 'as'", "Использовать 'as'", "'as' 사용"),
            code:        "let n = x as i32;"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0308.html"
    }]
};
