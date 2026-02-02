// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0616: private field access

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0616",
    title:       LocalizedText::new(
        "Attempted to access a private field",
        "Попытка доступа к приватному полю",
        "비공개 필드에 접근 시도"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
Attempted to access a private field on a struct from outside its module.
In Rust, struct fields are private by default unless explicitly marked
with the `pub` keyword.",
        "\
Попытка доступа к приватному полю структуры извне её модуля. В Rust поля
структур по умолчанию приватны, если явно не помечены ключевым словом `pub`.",
        "\
모듈 외부에서 구조체의 비공개 필드에 접근하려고 시도했습니다.
Rust에서 구조체 필드는 `pub` 키워드로 명시적으로 표시하지 않으면
기본적으로 비공개입니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Make the field public",
                "Сделать поле публичным",
                "필드를 공개로 설정"
            ),
            code:        "pub struct Foo {\n    pub x: u32, // now public\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Provide a getter method (encapsulation)",
                "Предоставить метод-получатель (инкапсуляция)",
                "getter 메서드 제공 (캡슐화)"
            ),
            code:        "impl Foo {\n    pub fn get_x(&self) -> &u32 { &self.x }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Visibility and Privacy",
            url:   "https://doc.rust-lang.org/reference/visibility-and-privacy.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0616.html"
        }
    ]
};
