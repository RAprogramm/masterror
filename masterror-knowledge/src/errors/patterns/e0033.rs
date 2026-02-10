// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0033: trait type dereferenced in pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0033",
    title:       LocalizedText::new(
        "Trait type dereferenced in pattern",
        "Разыменование трейт-объекта в паттерне",
        "패턴에서 트레이트 타입 역참조"
    ),
    category:    Category::Patterns,
    explanation: LocalizedText::new(
        "\
This error occurs when you try to dereference a trait object in a pattern.
Trait types don't have a known size at compile time, so they cannot be
dereferenced to create a local variable.

Example:
    let trait_obj: &dyn MyTrait = &value;
    let &x = trait_obj;  // Error: cannot dereference trait object",
        "\
Эта ошибка возникает при попытке разыменовать трейт-объект в паттерне.
Трейт-типы не имеют известного размера во время компиляции.",
        "\
이 오류는 패턴에서 트레이트 객체를 역참조하려고 할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Call methods directly on trait object",
            "Вызывать методы напрямую на трейт-объекте",
            "트레이트 객체에서 직접 메서드 호출"
        ),
        code:        "trait_obj.method();"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Trait Objects",
            url:   "https://doc.rust-lang.org/book/ch17-02-trait-objects.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0033.html"
        }
    ]
};
