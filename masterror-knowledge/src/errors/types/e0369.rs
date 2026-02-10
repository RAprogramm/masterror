// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0369: binary operation cannot be applied to type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0369",
    title:       LocalizedText::new(
        "Binary operation cannot be applied to type",
        "Бинарная операция не применима к типу",
        "이항 연산을 타입에 적용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A binary operation was attempted on a type which doesn't support it.
This error occurs when trying to use an operator (like <<, +, -, etc.)
with a type that hasn't implemented the corresponding trait from std::ops.

Example: f32 doesn't implement left shift (<<), so 12f32 << 2 fails.",
        "\
Бинарная операция была применена к типу, который её не поддерживает.
Эта ошибка возникает при попытке использовать оператор (<<, +, - и т.д.)
с типом, который не реализовал соответствующий трейт из std::ops.

Пример: f32 не реализует сдвиг влево (<<), поэтому 12f32 << 2 не работает.",
        "\
지원하지 않는 타입에 이항 연산이 시도되었습니다.
std::ops의 해당 트레이트를 구현하지 않은 타입에 연산자(<<, +, - 등)를
사용하려고 할 때 이 오류가 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a compatible type",
                "Использовать совместимый тип",
                "호환되는 타입 사용"
            ),
            code:        "let x = 12u32; // u32 supports <<\nx << 2;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Implement the operator trait for custom types",
                "Реализовать трейт оператора для своего типа",
                "사용자 정의 타입에 연산자 트레이트 구현"
            ),
            code:        "use std::ops::Add;\n\nimpl Add for MyType {\n    type Output = MyType;\n    fn add(self, rhs: Self) -> Self::Output {\n        // ...\n    }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops Module",
            url:   "https://doc.rust-lang.org/std/ops/index.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0369.html"
        }
    ]
};
