// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0539: invalid meta-item in attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0539",
    title:       LocalizedText::new(
        "Invalid meta-item in attribute",
        "Недопустимый мета-элемент в атрибуте",
        "속성에 잘못된 메타 항목"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An invalid meta-item was used inside an attribute. This can happen when:

1. Using `name = value` when a list is expected (e.g., `#[repr = \"C\"]`)
2. Using a list instead of `name = value` (e.g., `note(\"reason\")`)
3. Missing required values (e.g., `issue` without a value)
4. Providing unrecognized identifiers where specific keywords are required

Review the attribute's documentation to ensure correct syntax.",
        "\
Недопустимый мета-элемент был использован внутри атрибута. Это может
произойти, когда:

1. Используется `name = value` вместо списка
2. Используется список вместо `name = value`
3. Отсутствуют обязательные значения
4. Предоставлены нераспознанные идентификаторы",
        "\
속성 내부에서 잘못된 메타 항목이 사용되었습니다. 다음과 같은 경우에
발생할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use correct syntax for repr attribute",
                "Использовать правильный синтаксис для атрибута repr",
                "repr 속성에 올바른 구문 사용"
            ),
            code:        "#[repr(C)]  // not #[repr = \"C\"]\nstruct Foo {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use name = value for deprecated note",
                "Использовать name = value для note в deprecated",
                "deprecated note에 name = value 사용"
            ),
            code:        "#[deprecated(since = \"1.0.0\", note = \"reason\")]\nfn foo() {}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0539.html"
    }]
};
