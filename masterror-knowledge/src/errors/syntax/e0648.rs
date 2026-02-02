// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0648: export_name with null character

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0648",
    title:       LocalizedText::new(
        "export_name contains null characters",
        "export_name содержит нулевые символы",
        "export_name에 null 문자가 포함됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An `export_name` attribute was applied to a function with a null character
(`\\0`) in the export name string.

The Rust compiler does not permit null bytes in export name strings, as
they are invalid in C symbol names and would cause issues with external
linkage.",
        "\
Атрибут `export_name` был применён к функции с нулевым символом (`\\0`)
в строке имени экспорта.

Компилятор Rust не допускает нулевые байты в строках имён экспорта, так
как они недопустимы в именах символов C и вызовут проблемы с внешней
компоновкой.",
        "\
`export_name` 속성이 내보내기 이름 문자열에 null 문자(`\\0`)가 있는
함수에 적용되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove null characters from export_name",
            "Удалить нулевые символы из export_name",
            "export_name에서 null 문자 제거"
        ),
        code:        "#[export_name=\"foo\"] // no null characters\npub fn bar() {}"
    }],
    links:       &[
        DocLink {
            title: "FFI",
            url:   "https://doc.rust-lang.org/nomicon/ffi.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0648.html"
        }
    ]
};
