// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0502: cannot borrow as mutable because also borrowed as immutable

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0502",
    title:       LocalizedText::new(
        "Cannot borrow as mutable (already borrowed as immutable)",
        "Нельзя заимствовать как изменяемое (уже заимствовано как неизменяемое)",
        "가변으로 빌릴 수 없음 (이미 불변으로 빌림)"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
Rust enforces a strict borrowing rule: you can have EITHER one mutable
reference OR any number of immutable references, but never both at once.

This prevents data races at compile time. If you could mutate data while
someone else is reading it, the reader might see inconsistent state.

The immutable borrow is still \"active\" because it's used later in code.",
        "\
Rust применяет строгое правило: можно иметь ЛИБО одну изменяемую ссылку,
ЛИБО любое количество неизменяемых, но никогда обе одновременно.

Это предотвращает гонки данных.",
        "\
Rust는 엄격한 빌림 규칙을 적용합니다: 하나의 가변 참조 또는 여러 불변 참조를
가질 수 있지만, 동시에 둘 다 가질 수는 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "End the immutable borrow before mutating",
                "Завершить неизменяемое заимствование",
                "변경 전에 불변 빌림 종료"
            ),
            code:        "{ let r = &x; println!(\"{}\", r); } // r dropped\nx.push(1);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Clone before mutation",
                "Клонировать перед изменением",
                "변경 전에 복제"
            ),
            code:        "let copy = x[0].clone();\nx.push(copy);"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: References and Borrowing",
            url:   "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0502.html"
        }
    ]
};
