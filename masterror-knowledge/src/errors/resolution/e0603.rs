// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0603: private item used outside its scope

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0603",
    title:       LocalizedText::new(
        "Private item used outside its scope",
        "Приватный элемент использован вне своей области",
        "비공개 항목이 범위 외부에서 사용됨"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A private item was used outside its scope. Rust's visibility rules prevent
external code from accessing private items by default.

Items (constants, functions, structs, etc.) are private by default and
can only be accessed from within their defining module unless marked `pub`.",
        "\
Приватный элемент был использован вне своей области видимости. Правила
видимости Rust по умолчанию запрещают внешнему коду доступ к приватным
элементам.

Элементы (константы, функции, структуры и т.д.) по умолчанию приватны и
могут быть доступны только из определяющего их модуля, если не помечены
`pub`.",
        "\
비공개 항목이 범위 외부에서 사용되었습니다. Rust의 가시성 규칙은 기본적으로
외부 코드가 비공개 항목에 접근하는 것을 방지합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Make the item public",
            "Сделать элемент публичным",
            "항목을 공개로 설정"
        ),
        code:        "mod foo {\n    pub const VALUE: u32 = 42; // now public\n}"
    }],
    links:       &[
        DocLink {
            title: "Visibility and Privacy",
            url:   "https://doc.rust-lang.org/reference/visibility-and-privacy.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0603.html"
        }
    ]
};
