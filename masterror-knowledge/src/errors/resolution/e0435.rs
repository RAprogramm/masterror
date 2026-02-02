// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0435: non-constant value in constant expression

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0435",
    title:       LocalizedText::new(
        "Non-constant value in constant expression",
        "Неконстантное значение в константном выражении",
        "상수 표현식에 비상수 값"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A non-constant (runtime) value was used in a context that requires a
compile-time constant expression. Certain constructs like array lengths
require constant expressions and cannot accept runtime variables.

Variables (let bindings) are runtime values, while constants (const) are
compile-time values.",
        "\
Неконстантное (времени выполнения) значение использовано там, где
требуется константное выражение времени компиляции. Некоторые
конструкции, такие как длина массива, требуют константных выражений.

Переменные (let) - значения времени выполнения, а константы (const) -
значения времени компиляции.",
        "\
컴파일 타임 상수 표현식이 필요한 컨텍스트에서 비상수(런타임) 값이
사용되었습니다. 배열 길이와 같은 특정 구조는 상수 표현식이 필요하며
런타임 변수를 허용하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a const instead of let",
                "Использовать const вместо let",
                "let 대신 const 사용"
            ),
            code:        "const FOO: usize = 42;\nlet a: [u8; FOO]; // ok!"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a literal directly",
                "Использовать литерал напрямую",
                "리터럴 직접 사용"
            ),
            code:        "let a: [u8; 42]; // ok!"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Variables and Constants",
            url:   "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0435.html"
        }
    ]
};
