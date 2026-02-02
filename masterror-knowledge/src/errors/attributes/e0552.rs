// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0552: unrecognized representation hint

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0552",
    title:       LocalizedText::new(
        "Unrecognized representation hint",
        "Нераспознанный атрибут представления",
        "인식할 수 없는 표현 힌트"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unrecognized representation attribute was used. The `#[repr(...)]`
attribute is used to specify how the compiler should lay out a struct or enum
in memory. Only certain values are recognized.

The `repr` attribute supports options like `C`, `transparent`, `packed`,
`align(N)`, and integer types like `u8`, `i32`, etc.",
        "\
Был использован нераспознанный атрибут представления. Атрибут `#[repr(...)]`
используется для указания, как компилятор должен размещать структуру или
перечисление в памяти. Распознаются только определённые значения.",
        "\
인식할 수 없는 표현 속성이 사용되었습니다. `#[repr(...)]` 속성은 컴파일러가
구조체나 열거형을 메모리에 어떻게 배치해야 하는지 지정하는 데 사용됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a valid repr option",
            "Использовать допустимую опцию repr",
            "유효한 repr 옵션 사용"
        ),
        code:        "#[repr(C)]  // valid options: C, transparent, packed, align(N)\nstruct MyStruct {\n    my_field: usize\n}"
    }],
    links:       &[
        DocLink {
            title: "Alternative Representations",
            url:   "https://doc.rust-lang.org/nomicon/other-reprs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0552.html"
        }
    ]
};
