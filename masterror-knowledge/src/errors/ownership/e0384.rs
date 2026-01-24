// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0384: cannot assign twice to immutable variable

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0384",
    title:       LocalizedText::new(
        "Cannot assign twice to immutable variable",
        "Нельзя присвоить дважды неизменяемой переменной",
        "불변 변수에 두 번 할당할 수 없음"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
Variables in Rust are immutable by default. Once a value is bound to a name,
you cannot change it unless you explicitly mark it as mutable with `mut`.

This is a deliberate design choice that makes code easier to reason about.
When you see a variable without `mut`, you know it won't change.",
        "\
Переменные в Rust неизменяемы по умолчанию. После привязки значения
к имени вы не можете его изменить без явного указания `mut`.

Это осознанное решение, упрощающее понимание кода.
Если переменная без `mut`, она не изменится.",
        "\
Rust의 변수는 기본적으로 불변입니다. 값이 이름에 바인딩되면
`mut`로 명시적으로 표시하지 않는 한 변경할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Make the variable mutable",
                "Сделать переменную изменяемой",
                "변수를 가변으로 만들기"
            ),
            code:        "let mut x = 5;\nx = 10;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use shadowing (create new binding)",
                "Использовать затенение (новая привязка)",
                "섀도잉 사용 (새 바인딩 생성)"
            ),
            code:        "let x = 5;\nlet x = 10; // shadows the first x"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Variables and Mutability",
            url:   "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0384.html"
        }
    ]
};
