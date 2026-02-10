// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0517: repr attribute on unsupported item

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0517",
    title:       LocalizedText::new(
        "`#[repr(..)]` attribute on unsupported item",
        "Атрибут `#[repr(..)]` на неподдерживаемом элементе",
        "지원되지 않는 항목에 `#[repr(..)]` 속성"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `#[repr(..)]` attribute was placed on an unsupported item. Each
representation attribute only works with specific item types:

- `#[repr(C)]`: Only structs and enums
- `#[repr(packed)]` and `#[repr(simd)]`: Only structs
- `#[repr(u8)]`, `#[repr(i16)]`, etc.: Only field-less enums

These attributes cannot be applied to type aliases or impl blocks.",
        "\
Атрибут `#[repr(..)]` был помещён на неподдерживаемый элемент. Каждый
атрибут представления работает только с определёнными типами элементов:

- `#[repr(C)]`: Только структуры и перечисления
- `#[repr(packed)]` и `#[repr(simd)]`: Только структуры
- `#[repr(u8)]`, `#[repr(i16)]` и т.д.: Только перечисления без полей",
        "\
`#[repr(..)]` 속성이 지원되지 않는 항목에 배치되었습니다. 각 표현 속성은
특정 항목 타입에서만 작동합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Apply repr(C) to struct or enum",
                "Применить repr(C) к структуре или перечислению",
                "repr(C)를 구조체 또는 열거형에 적용"
            ),
            code:        "#[repr(C)]\nstruct Foo { bar: bool }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Apply repr(u8) to field-less enum",
                "Применить repr(u8) к перечислению без полей",
                "repr(u8)를 필드 없는 열거형에 적용"
            ),
            code:        "#[repr(u8)]\nenum Color { Red, Green, Blue }"
        }
    ],
    links:       &[
        DocLink {
            title: "Alternative Representations",
            url:   "https://doc.rust-lang.org/nomicon/other-reprs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0517.html"
        }
    ]
};
