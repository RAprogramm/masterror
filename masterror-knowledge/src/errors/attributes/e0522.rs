// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0522: lang attribute used in invalid context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0522",
    title:       LocalizedText::new(
        "Unknown or invalid lang item",
        "Неизвестный или недопустимый lang элемент",
        "알 수 없거나 잘못된 lang 항목"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `#[lang]` attribute was used in an invalid context. This attribute is
intended exclusively for marking special items that are built-in to Rust,
including:

- Special traits that affect compiler behavior (e.g., `Copy`, `Sized`)
- Special functions that may be automatically invoked (e.g., panic handlers)

Using the `#[lang]` attribute with unknown or invalid lang items will result
in this error.",
        "\
Атрибут `#[lang]` был использован в недопустимом контексте. Этот атрибут
предназначен исключительно для пометки специальных элементов, встроенных
в Rust, включая:

- Специальные трейты, влияющие на поведение компилятора
- Специальные функции, которые могут автоматически вызываться",
        "\
`#[lang]` 속성이 잘못된 컨텍스트에서 사용되었습니다. 이 속성은 Rust에
내장된 특수 항목을 표시하기 위해서만 사용됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Only use valid compiler-recognized lang items",
            "Использовать только допустимые lang элементы",
            "유효한 컴파일러 인식 lang 항목만 사용"
        ),
        code:        "// Don't use #[lang] with custom names\n// This is for internal compiler use only"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0522.html"
    }]
};
