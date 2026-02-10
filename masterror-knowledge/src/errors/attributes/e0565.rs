// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0565: literal used in attribute that doesn't support literals

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0565",
    title:       LocalizedText::new(
        "Literal used in attribute that doesn't support literals",
        "Литерал использован в атрибуте, не поддерживающем литералы",
        "리터럴을 지원하지 않는 속성에서 리터럴 사용"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A literal was used in a built-in attribute that doesn't support literals.
Not all attributes support literals in their input - some specifically require
identifiers.

For example, `#[repr(\"C\")]` is incorrect because `repr` expects an
identifier, not a string literal.",
        "\
Литерал был использован во встроенном атрибуте, который не поддерживает
литералы. Не все атрибуты поддерживают литералы во входных данных - некоторые
требуют именно идентификаторы.",
        "\
리터럴을 지원하지 않는 내장 속성에서 리터럴이 사용되었습니다. 모든 속성이
입력에서 리터럴을 지원하는 것은 아니며, 일부는 특별히 식별자를 요구합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use an identifier instead of a string literal",
            "Использовать идентификатор вместо строкового литерала",
            "문자열 리터럴 대신 식별자 사용"
        ),
        code:        "#[repr(C)]  // not #[repr(\"C\")]\nstruct Repr {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0565.html"
    }]
};
