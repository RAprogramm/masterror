// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0779: unknown instruction_set argument

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0779",
    title:       LocalizedText::new(
        "Unknown instruction_set argument",
        "Неизвестный аргумент instruction_set",
        "알 수 없는 instruction_set 인수"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An unknown argument was given to the `instruction_set` attribute.

Currently supported arguments are:
- `arm::a32`
- `arm::t32`",
        "\
В атрибут `instruction_set` передан неизвестный аргумент.

Поддерживаемые аргументы: `arm::a32`, `arm::t32`.",
        "\
`instruction_set` 속성에 알 수 없는 인수가 전달되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use valid instruction set",
                "Используйте допустимый набор инструкций",
                "유효한 명령어 집합 사용"
            ),
            code:        "#[cfg_attr(target_arch=\"arm\", instruction_set(arm::a32))]\npub fn something() {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Codegen Attributes",
            url:   "https://doc.rust-lang.org/reference/attributes/codegen.html"
        }
    ]
};
