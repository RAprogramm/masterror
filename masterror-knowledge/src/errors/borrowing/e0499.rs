// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0499: cannot borrow as mutable more than once

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0499",
    title:       LocalizedText::new(
        "Cannot borrow as mutable more than once",
        "Нельзя заимствовать как изменяемое более одного раза",
        "가변으로 두 번 이상 빌릴 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
Rust allows only ONE mutable reference to data at a time. This is stricter
than the immutable borrowing rule and prevents all aliased mutation.

Why? Two mutable references to the same data could lead to:
- Data races in concurrent code
- Iterator invalidation
- Dangling pointers after reallocation

This rule is checked at compile time, giving you fearless concurrency.",
        "\
Rust разрешает только ОДНУ изменяемую ссылку на данные одновременно.
Это строже правила неизменяемого заимствования.

Почему? Две изменяемые ссылки на одни данные могут привести к:
- Гонкам данных в конкурентном коде
- Инвалидации итераторов
- Висячим указателям после реаллокации",
        "\
Rust는 데이터에 대해 한 번에 하나의 가변 참조만 허용합니다.
이는 불변 빌림 규칙보다 엄격합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use scopes to limit borrow lifetime",
                "Использовать области видимости",
                "스코프를 사용하여 빌림 수명 제한"
            ),
            code:        "{ let r1 = &mut x; *r1 += 1; } // r1 dropped\nlet r2 = &mut x;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use RefCell for interior mutability",
                "Использовать RefCell",
                "내부 가변성을 위해 RefCell 사용"
            ),
            code:        "use std::cell::RefCell;\nlet x = RefCell::new(value);"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Mutable References",
            url:   "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0499.html"
        }
    ]
};
