// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0538: duplicate meta item in attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0538",
    title:       LocalizedText::new(
        "Duplicate meta item in attribute",
        "Дублирующийся мета-элемент в атрибуте",
        "속성에 중복된 메타 항목"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An attribute contains the same meta item more than once. Meta items are the
key-value pairs inside of an attribute, and each key may only be used once
in each attribute.

If you specify the same meta item key multiple times, the compiler will raise
this error.",
        "\
Атрибут содержит один и тот же мета-элемент более одного раза. Мета-элементы -
это пары ключ-значение внутри атрибута, и каждый ключ может использоваться
только один раз в каждом атрибуте.",
        "\
속성에 동일한 메타 항목이 두 번 이상 포함되어 있습니다. 메타 항목은
속성 내부의 키-값 쌍이며, 각 키는 각 속성에서 한 번만 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove duplicate meta items",
            "Удалить дублирующиеся мета-элементы",
            "중복된 메타 항목 제거"
        ),
        code:        "#[deprecated(\n    since=\"1.0.0\",\n    note=\"First note only.\"\n)]\nfn deprecated_function() {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0538.html"
    }]
};
