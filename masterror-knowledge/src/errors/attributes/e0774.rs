// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0774: derive on invalid target

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0774",
    title:       LocalizedText::new(
        "Derive applied to invalid target",
        "Derive применён к неподходящему элементу",
        "잘못된 대상에 derive 적용"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
The `derive` attribute was applied on something which is not a struct, union,
or enum. The `derive` attribute is only allowed on these three item types.",
        "\
Атрибут `derive` применён к элементу, который не является структурой,
объединением или перечислением. `derive` допустим только для них.",
        "\
`derive` 속성이 struct, union, enum이 아닌 것에 적용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Apply derive to struct, enum, or union",
                "Применяйте derive к struct, enum или union",
                "struct, enum, union에 derive 적용"
            ),
            code:        "#[derive(Clone)]\nstruct Bar {\n    field: u32,\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Derivable Traits",
            url:   "https://doc.rust-lang.org/book/appendix-03-derivable-traits.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0774.html"
        }
    ]
};
