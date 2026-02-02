// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0516: typeof keyword is reserved but unimplemented

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0516",
    title:       LocalizedText::new(
        "The `typeof` keyword is reserved but unimplemented",
        "Ключевое слово `typeof` зарезервировано, но не реализовано",
        "`typeof` 키워드는 예약되었지만 구현되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `typeof` keyword was used in code, but this keyword is currently reserved
for future use and not implemented in the Rust compiler.

Note: This error code is no longer emitted by the compiler.",
        "\
Ключевое слово `typeof` было использовано в коде, но оно зарезервировано
для будущего использования и не реализовано в компиляторе Rust.

Примечание: этот код ошибки больше не выдаётся компилятором.",
        "\
`typeof` 키워드가 코드에서 사용되었지만, 이 키워드는 현재 향후 사용을 위해
예약되어 있으며 Rust 컴파일러에서 구현되지 않았습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use type inference instead",
            "Использовать вывод типов",
            "타입 추론 사용"
        ),
        code:        "let x = 92; // compiler infers i32"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0516.html"
    }]
};
