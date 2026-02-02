// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0541: unknown meta item in attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0541",
    title:       LocalizedText::new(
        "Unknown meta item in attribute",
        "Неизвестный мета-элемент в атрибуте",
        "속성에 알 수 없는 메타 항목"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unknown meta item was used in an attribute. Meta items are the key-value
pairs inside of attributes, and the keys provided must be one of the valid
keys for the specified attribute.

Either remove the unknown meta item, or rename it to a correct one.",
        "\
Неизвестный мета-элемент был использован в атрибуте. Мета-элементы - это
пары ключ-значение внутри атрибутов, и предоставленные ключи должны быть
одним из допустимых ключей для указанного атрибута.",
        "\
속성에서 알 수 없는 메타 항목이 사용되었습니다. 메타 항목은 속성 내부의
키-값 쌍이며, 제공된 키는 지정된 속성에 대한 유효한 키 중 하나여야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use correct meta item key (e.g., `note` not `reason`)",
            "Использовать правильный ключ (например, `note` вместо `reason`)",
            "올바른 메타 항목 키 사용 (예: `reason`이 아닌 `note`)"
        ),
        code:        "#[deprecated(\n    since=\"1.0.0\",\n    note=\"explanation\" // not 'reason'\n)]\nfn deprecated_function() {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0541.html"
    }]
};
