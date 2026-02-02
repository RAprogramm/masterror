// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0368: binary assignment operator applied to unsupported type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0368",
    title:       LocalizedText::new(
        "Binary assignment operator cannot be applied to type",
        "Бинарный оператор присваивания не применим к типу",
        "이항 대입 연산자를 타입에 적용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A binary assignment operator (like +=, -=, <<=, ^=, etc.) is applied to a
type that doesn't implement the corresponding assignment trait.

Common causes:
1. The type doesn't support the operation (e.g., <<= on f32)
2. You implemented the operator trait (e.g., Add) but not the assignment
   variant (e.g., AddAssign)",
        "\
Бинарный оператор присваивания (например +=, -=, <<=, ^=) применён к типу,
который не реализует соответствующий трейт присваивания.

Частые причины:
1. Тип не поддерживает операцию (например <<= для f32)
2. Реализован трейт оператора (Add), но не трейт присваивания (AddAssign)",
        "\
이항 대입 연산자(+=, -=, <<=, ^= 등)가 해당 대입 트레이트를 구현하지 않는
타입에 적용되었습니다.

일반적인 원인:
1. 타입이 연산을 지원하지 않음 (예: f32에 <<=)
2. 연산자 트레이트(Add)는 구현했지만 대입 변형(AddAssign)은 구현하지 않음"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement the assignment trait",
                "Реализовать трейт присваивания",
                "대입 트레이트 구현"
            ),
            code:        "use std::ops::AddAssign;\n\nimpl AddAssign for Foo {\n    fn add_assign(&mut self, rhs: Foo) {\n        self.0 += rhs.0;\n    }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a type that supports the operation",
                "Использовать тип, поддерживающий операцию",
                "연산을 지원하는 타입 사용"
            ),
            code:        "let mut x = 12u32; // u32 supports <<=\nx <<= 2;"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops Module",
            url:   "https://doc.rust-lang.org/std/ops/index.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0368.html"
        }
    ]
};
