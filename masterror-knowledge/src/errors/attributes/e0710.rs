// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0710: unknown tool name in scoped lint

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0710",
    title:       LocalizedText::new(
        "Unknown tool name in scoped lint",
        "Неизвестное имя инструмента в lint",
        "스코프 lint에서 알 수 없는 도구 이름"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An unknown tool name was found in a scoped lint attribute.

This typically happens when you misspell a linter tool name (such as `clippy`)
or forget to import it in your project.",
        "\
В атрибуте lint найдено неизвестное имя инструмента.

Обычно это происходит при опечатке в имени линтера (например, `clippy`).",
        "\
스코프 lint 속성에서 알 수 없는 도구 이름이 발견되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Fix the tool name spelling",
                "Исправьте написание имени инструмента",
                "도구 이름 철자 수정"
            ),
            code:        "#[allow(clippy::filter_map)] // correct spelling"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0710.html"
        }
    ]
};
