// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0568: auto traits cannot have super traits

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0568",
    title:       LocalizedText::new(
        "Auto traits cannot have super traits",
        "Автоматические трейты не могут иметь супертрейты",
        "자동 트레이트는 슈퍼트레이트를 가질 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A super trait has been added to an auto trait. Auto traits are automatically
implemented for all existing types. Adding a super trait (a required trait
bound) would restrict which types can implement it, defeating the purpose of
an auto trait.

This creates a contradiction since auto traits should apply universally to
all types, but a super trait bound would restrict that.",
        "\
Супертрейт был добавлен к автоматическому трейту. Автоматические трейты
реализуются для всех существующих типов автоматически. Добавление супертрейта
(требуемого ограничения трейта) ограничит типы, которые могут его реализовать,
что противоречит цели автоматического трейта.",
        "\
자동 트레이트에 슈퍼트레이트가 추가되었습니다. 자동 트레이트는 모든 기존 타입에
자동으로 구현됩니다. 슈퍼트레이트(필수 트레이트 바운드)를 추가하면 구현할 수
있는 타입이 제한되어 자동 트레이트의 목적에 어긋납니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove super trait from auto trait",
            "Удалить супертрейт из автоматического трейта",
            "자동 트레이트에서 슈퍼트레이트 제거"
        ),
        code:        "#![feature(auto_traits)]\n\nauto trait Bound {} // no : Copy"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0568.html"
    }]
};
