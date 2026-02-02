// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0778: malformed instruction_set attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0778",
    title:       LocalizedText::new(
        "Malformed instruction_set attribute",
        "Неправильный атрибут instruction_set",
        "잘못된 instruction_set 속성"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
The `instruction_set` attribute was malformed. This attribute requires
exactly one argument specifying the instruction set architecture.",
        "\
Атрибут `instruction_set` неправильно сформирован.
Он требует ровно один аргумент, указывающий архитектуру.",
        "\
`instruction_set` 속성이 잘못되었습니다.
명령어 집합 아키텍처를 지정하는 인수가 필요합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Provide instruction set argument",
                "Укажите аргумент набора инструкций",
                "명령어 집합 인수 제공"
            ),
            code:        "#[cfg_attr(target_arch=\"arm\", instruction_set(arm::a32))]\nfn something() {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Codegen Attributes",
            url:   "https://doc.rust-lang.org/reference/attributes/codegen.html"
        }
    ]
};
