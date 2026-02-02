// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0566: conflicting representation hints

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0566",
    title:       LocalizedText::new(
        "Conflicting representation hints",
        "Конфликтующие атрибуты представления",
        "충돌하는 표현 힌트"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Conflicting representation hints have been used on the same item. In most
cases, only one representation hint is needed.

For example, `#[repr(u32, u64)]` is invalid because you cannot specify
multiple conflicting integer representation hints on the same enum.",
        "\
Конфликтующие атрибуты представления были использованы для одного элемента.
В большинстве случаев нужен только один атрибут представления.

Например, `#[repr(u32, u64)]` недопустим, поскольку нельзя указать
несколько конфликтующих целочисленных представлений для одного перечисления.",
        "\
동일한 항목에 충돌하는 표현 힌트가 사용되었습니다. 대부분의 경우
하나의 표현 힌트만 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use cfg_attr for conditional repr",
            "Использовать cfg_attr для условного repr",
            "조건부 repr를 위해 cfg_attr 사용"
        ),
        code:        "#[cfg_attr(linux, repr(u32))]\n#[cfg_attr(not(linux), repr(u64))]\nenum Repr { A }"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0566.html"
    }]
};
