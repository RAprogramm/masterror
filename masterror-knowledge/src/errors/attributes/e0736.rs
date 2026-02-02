// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0736: naked function incompatible attributes

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0736",
    title:       LocalizedText::new(
        "Naked function with incompatible attribute",
        "Naked функция с несовместимым атрибутом",
        "호환되지 않는 속성이 있는 naked 함수"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
Functions marked with `#[naked]` are restricted in what other attributes
they may be marked with. Incompatible attributes include:
- `#[inline]`
- `#[track_caller]`
- `#[test]`, `#[ignore]`, `#[should_panic]`

These incompatibilities exist because naked functions deliberately impose
strict restrictions on the code that the compiler produces.",
        "\
Функции с `#[naked]` имеют ограничения на другие атрибуты.
Несовместимы: `#[inline]`, `#[track_caller]`, `#[test]`.",
        "\
`#[naked]`로 표시된 함수는 다른 속성에 제한이 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove incompatible attributes",
                "Удалите несовместимые атрибуты",
                "호환되지 않는 속성 제거"
            ),
            code:        "#[unsafe(naked)]\npub extern \"C\" fn foo() {\n    // naked_asm!(...)\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0736.html"
        }
    ]
};
