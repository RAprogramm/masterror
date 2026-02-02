// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0536: malformed not cfg-predicate

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0536",
    title:       LocalizedText::new(
        "The `not` cfg-predicate was malformed",
        "cfg-предикат `not` был неправильно сформирован",
        "`not` cfg 술어가 잘못 형성됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `not` cfg-predicate was malformed. The `not` predicate is used in
conditional compilation attributes and must be properly formatted. It expects
exactly one cfg-pattern as its argument.

The `not()` cannot be empty - it requires a cfg-pattern argument.",
        "\
cfg-предикат `not` был неправильно сформирован. Предикат `not` используется
в атрибутах условной компиляции и должен быть правильно отформатирован.
Он ожидает ровно один cfg-шаблон в качестве аргумента.",
        "\
`not` cfg 술어가 잘못 형성되었습니다. `not` 술어는 조건부 컴파일 속성에서
사용되며 올바르게 형식화되어야 합니다. 인수로 정확히 하나의 cfg 패턴이
필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Provide a cfg-pattern inside not()",
            "Указать cfg-шаблон внутри not()",
            "not() 내부에 cfg 패턴 제공"
        ),
        code:        "#[cfg(not(target_os = \"linux\"))]\npub fn main() { }"
    }],
    links:       &[
        DocLink {
            title: "Conditional Compilation",
            url:   "https://doc.rust-lang.org/reference/conditional-compilation.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0536.html"
        }
    ]
};
