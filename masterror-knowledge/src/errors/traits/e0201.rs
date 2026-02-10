// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0201: duplicate associated items in impl block

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0201",
    title:       LocalizedText::new(
        "Duplicate associated items in impl block",
        "Дублирующиеся ассоциированные элементы в блоке impl",
        "impl 블록에 중복된 연관 항목"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Two associated items (methods, associated types, or associated functions)
are defined with the same identifier within the same `impl` block or trait
implementation.

You cannot have duplicate names for associated items in a single
implementation. This applies to methods, associated types, and
associated functions.

Note: Items with the same name ARE allowed in separate `impl` blocks
for different types.",
        "\
Два ассоциированных элемента (методы, ассоциированные типы или функции)
определены с одинаковым идентификатором в одном блоке `impl` или
реализации трейта.

Нельзя иметь дублирующиеся имена для ассоциированных элементов
в одной реализации.",
        "\
동일한 `impl` 블록 또는 트레이트 구현 내에서 두 개의 연관 항목
(메서드, 연관 타입 또는 연관 함수)이 같은 식별자로 정의되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove duplicate or rename one of the items",
            "Удалить дубликат или переименовать один из элементов",
            "중복을 제거하거나 항목 중 하나의 이름 변경"
        ),
        code:        "impl Foo {\n    fn bar(&self) -> bool { self.0 > 5 }\n    fn baz() {} // renamed from bar\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Implementations",
            url:   "https://doc.rust-lang.org/reference/items/implementations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0201.html"
        }
    ]
};
