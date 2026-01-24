// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0382: borrow of moved value

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0382",
    title:       LocalizedText::new(
        "Borrow of moved value",
        "Заимствование перемещённого значения",
        "이동된 값의 빌림"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
In Rust, each value has exactly one owner at a time. This is the foundation
of Rust's memory safety guarantees without garbage collection.

When you assign a value to another variable or pass it to a function,
ownership MOVES to the new location. The original variable becomes invalid
and cannot be used anymore.

This happens because Rust needs to know exactly when to free memory.
With one owner, there's no ambiguity about who is responsible for cleanup.",
        "\
В Rust каждое значение имеет ровно одного владельца. Это основа
гарантий безопасности памяти без сборщика мусора.

Когда вы присваиваете значение другой переменной или передаёте в функцию,
владение ПЕРЕМЕЩАЕТСЯ. Исходная переменная становится недействительной.

Rust должен точно знать, когда освобождать память.
С одним владельцем нет неоднозначности в том, кто отвечает за очистку.",
        "\
Rust에서 각 값은 정확히 하나의 소유자를 가집니다. 이것이 가비지 컬렉터 없이
메모리 안전성을 보장하는 기반입니다.

값을 다른 변수에 할당하거나 함수에 전달하면 소유권이 새 위치로 이동합니다.
원래 변수는 무효화되어 더 이상 사용할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Clone the value (creates a deep copy)",
                "Клонировать значение (глубокая копия)",
                "값을 복제 (깊은 복사)"
            ),
            code:        "let s2 = s.clone();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Borrow with a reference (no copy)",
                "Заимствовать по ссылке (без копии)",
                "참조로 빌림 (복사 없음)"
            ),
            code:        "let s2 = &s;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Implement Copy trait (for small types)",
                "Реализовать Copy (для маленьких типов)",
                "Copy 트레이트 구현 (작은 타입용)"
            ),
            code:        "#[derive(Copy, Clone)]"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Ownership",
            url:   "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0382.html"
        }
    ]
};
