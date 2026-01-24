// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0597: value does not live long enough

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0597",
    title:       LocalizedText::new(
        "Value does not live long enough",
        "Значение живёт недостаточно долго",
        "값이 충분히 오래 살지 않음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
You're creating a reference to something that will be destroyed before
the reference is used. This would create a dangling pointer.

Rust prevents this at compile time. The referenced value must live at
least as long as the reference itself.",
        "\
Вы создаёте ссылку на что-то, что будет уничтожено до использования ссылки.",
        "\
참조가 사용되기 전에 파괴될 것에 대한 참조를 만들고 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Move value to outer scope",
                "Переместить значение во внешнюю область",
                "값을 외부 스코프로 이동"
            ),
            code:        "let s = String::from(\"hello\"); // declare before use"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Return owned value instead",
                "Вернуть владеющее значение",
                "소유 값 반환"
            ),
            code:        "fn get() -> String { s.to_string() }"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Lifetimes",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0597.html"
        }
    ]
};
