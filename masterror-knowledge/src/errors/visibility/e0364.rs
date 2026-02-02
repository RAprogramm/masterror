// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0364: private items cannot be publicly re-exported

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0364",
    title:       LocalizedText::new(
        "Private items cannot be publicly re-exported",
        "Приватные элементы нельзя публично реэкспортировать",
        "비공개 항목은 공개적으로 재내보내기할 수 없음"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
You attempted to use `pub use` to re-export an item that is not itself marked
as `pub`. You cannot publicly expose private items through re-exports.

Re-exports cannot elevate the visibility of private items to public scope.",
        "\
Вы попытались использовать `pub use` для реэкспорта элемента, который сам
не помечен как `pub`. Нельзя публично раскрывать приватные элементы через
реэкспорт.

Реэкспорт не может повысить видимость приватных элементов до публичной области.",
        "\
`pub use`를 사용하여 `pub`로 표시되지 않은 항목을 재내보내기하려고 했습니다.
재내보내기를 통해 비공개 항목을 공개적으로 노출할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Mark the item as pub before re-exporting",
            "Пометить элемент как pub перед реэкспортом",
            "재내보내기 전에 항목을 pub로 표시"
        ),
        code:        "mod a {\n    pub fn foo() {}  // now public\n    \n    mod a {\n        pub use super::foo;  // ok!\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Use Declarations",
            url:   "https://doc.rust-lang.org/reference/items/use-declarations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0364.html"
        }
    ]
};
