// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0537: unknown predicate in cfg attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0537",
    title:       LocalizedText::new(
        "Unknown predicate in `cfg` attribute",
        "Неизвестный предикат в атрибуте `cfg`",
        "`cfg` 속성에 알 수 없는 술어"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unknown predicate was used inside the `cfg` attribute. The `cfg` attribute
only supports three specific predicate types:
- `any`
- `all`
- `not`

Using any other predicate name will result in an error.",
        "\
Неизвестный предикат был использован внутри атрибута `cfg`. Атрибут `cfg`
поддерживает только три типа предикатов:
- `any`
- `all`
- `not`

Использование любого другого имени предиката приведёт к ошибке.",
        "\
`cfg` 속성 내부에서 알 수 없는 술어가 사용되었습니다. `cfg` 속성은
세 가지 특정 술어 타입만 지원합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use valid cfg predicates: any, all, not",
            "Использовать допустимые cfg предикаты: any, all, not",
            "유효한 cfg 술어 사용: any, all, not"
        ),
        code:        "#[cfg(not(target_os = \"linux\"))]\npub fn something() {}"
    }],
    links:       &[
        DocLink {
            title: "Conditional Compilation",
            url:   "https://doc.rust-lang.org/reference/conditional-compilation.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0537.html"
        }
    ]
};
