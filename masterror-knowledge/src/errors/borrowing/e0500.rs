// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0500: closure requires unique access but X is already borrowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0500",
    title:       LocalizedText::new(
        "Closure requires unique access but value is already borrowed",
        "Замыкание требует уникальный доступ, но значение уже заимствовано",
        "클로저가 고유 접근을 필요로 하지만 값이 이미 빌려짐"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
A closure that mutates a captured variable needs exclusive access to it.
But you've already borrowed the value elsewhere, creating a conflict.

Closures that capture by mutable reference act like mutable borrows.",
        "\
Замыкание, изменяющее захваченную переменную, требует эксклюзивного доступа.",
        "\
캡처된 변수를 변경하는 클로저는 독점적인 접근이 필요합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "End the borrow before the closure",
                "Завершить заимствование перед замыканием",
                "클로저 전에 빌림 종료"
            ),
            code:        "{ let r = &x; use(r); }\nlet c = || x += 1;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Move the value into the closure",
                "Переместить значение в замыкание",
                "클로저로 값 이동"
            ),
            code:        "let c = move || { x += 1; };"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0500.html"
    }]
};
