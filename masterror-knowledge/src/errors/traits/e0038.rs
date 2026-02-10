// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0038",
    title:       LocalizedText::new(
        "Trait cannot be made into an object",
        "Трейт не может быть превращён в объект",
        "트레이트를 객체로 만들 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "This trait is not object-safe - it can't be used as `dyn Trait`.",
        "Этот трейт не объектно-безопасен - его нельзя использовать как `dyn Trait`.",
        "이 트레이트는 객체 안전하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new("Use generics", "Использовать обобщения", "제네릭 사용"),
        code:        "fn process<T: MyTrait>(item: T) { ... }"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0038.html"
    }]
};
