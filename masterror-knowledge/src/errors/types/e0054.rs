// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0054: cannot cast to bool

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0054",
    title:       LocalizedText::new(
        "Cannot cast to bool",
        "Нельзя преобразовать в bool",
        "bool로 캐스트할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Rust does not allow casting values directly to `bool` using the `as` operator.

Example:
    let x = 5;
    let b = x as bool;  // Error: cannot cast to bool",
        "\
Rust не позволяет напрямую преобразовывать значения в `bool` с помощью
оператора `as`.",
        "\
Rust는 `as` 연산자를 사용하여 값을 `bool`로 직접 캐스트하는 것을 허용하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a comparison instead",
            "Использовать сравнение вместо приведения",
            "대신 비교 사용"
        ),
        code:        "let x = 5;\nlet b = x != 0;  // true if x is nonzero"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0054.html"
    }]
};
