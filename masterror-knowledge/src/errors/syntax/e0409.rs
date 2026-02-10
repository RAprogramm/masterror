// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0409: inconsistent binding modes in or-pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0409",
    title:       LocalizedText::new(
        "Inconsistent binding modes in or-pattern",
        "Несовместимые режимы связывания в или-паттерне",
        "or 패턴에서 일관되지 않은 바인딩 모드"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
In an or-pattern, a variable cannot be bound by-value in one pattern and
by-reference in another. All bindings of the same variable must use the
same binding mode (ref, ref mut, or by-value).",
        "\
В или-паттерне переменная не может быть связана по значению в одном
паттерне и по ссылке в другом. Все связывания одной переменной должны
использовать один режим (ref, ref mut или по значению).",
        "\
or 패턴에서 변수는 한 패턴에서는 값으로, 다른 패턴에서는 참조로
바인딩될 수 없습니다. 같은 변수의 모든 바인딩은 동일한 바인딩 모드를
사용해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use consistent binding modes",
                "Использовать одинаковый режим связывания",
                "일관된 바인딩 모드 사용"
            ),
            code:        "match x {\n    (0, ref y) | (ref y, 0) => { /* both ref */ }\n    _ => ()\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Split into separate patterns",
                "Разделить на отдельные паттерны",
                "별도의 패턴으로 분리"
            ),
            code:        "match x {\n    (y, 0) => { /* by value */ }\n    (0, ref y) => { /* by ref */ }\n    _ => ()\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0409.html"
    }]
};
