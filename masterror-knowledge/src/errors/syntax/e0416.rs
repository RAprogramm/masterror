// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0416: identifier bound more than once in pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0416",
    title:       LocalizedText::new(
        "Identifier bound more than once in the same pattern",
        "Идентификатор связан более одного раза в паттерне",
        "같은 패턴에서 식별자가 두 번 이상 바인딩됨"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
The same identifier was used more than once in a single pattern. Each variable
name can only be bound once within the same pattern. Using the same identifier
multiple times creates ambiguity.",
        "\
Один и тот же идентификатор использован более одного раза в одном паттерне.
Каждое имя переменной может быть связано только один раз в пределах одного
паттерна.",
        "\
같은 패턴에서 동일한 식별자가 두 번 이상 사용되었습니다. 각 변수 이름은
같은 패턴 내에서 한 번만 바인딩될 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use different variable names",
                "Использовать разные имена переменных",
                "다른 변수 이름 사용"
            ),
            code:        "match (1, 2) {\n    (x, y) => {} // Different names\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use guards to compare values",
                "Использовать охранные выражения для сравнения",
                "값 비교를 위해 가드 사용"
            ),
            code:        "match (a, b) {\n    (x, y) if x == y => { /* equal */ }\n    (x, y) => { /* not equal */ }\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0416.html"
    }]
};
