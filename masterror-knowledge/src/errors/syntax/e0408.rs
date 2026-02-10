// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0408: variable not bound in all patterns

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0408",
    title:       LocalizedText::new(
        "Variable not bound in all patterns of or-pattern",
        "Переменная не связана во всех вариантах или-паттерна",
        "or 패턴의 모든 분기에서 변수가 바인딩되지 않음"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
When using an or-pattern (|) in a match expression, any variables bound in one
branch must also be bound in all other branches. If a variable exists in some
patterns but not others, the compiler cannot guarantee it will have a value
in the match arm body.",
        "\
При использовании или-паттерна (|) в выражении match любые переменные,
связанные в одной ветви, должны быть связаны и во всех остальных.
Если переменная существует в одних паттернах, но не в других, компилятор
не может гарантировать, что она будет иметь значение.",
        "\
match 표현식에서 or 패턴(|)을 사용할 때, 한 분기에서 바인딩된 변수는
다른 모든 분기에서도 바인딩되어야 합니다. 변수가 일부 패턴에만
존재하면 컴파일러가 값의 존재를 보장할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Split into separate match arms",
                "Разделить на отдельные ветви match",
                "별도의 match 분기로 분리"
            ),
            code:        "match x {\n    Some(y) => { /* use y */ }\n    None => { /* ... */ }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Bind variables consistently in all patterns",
                "Связать переменные одинаково во всех паттернах",
                "모든 패턴에서 일관되게 변수 바인딩"
            ),
            code:        "match x {\n    (0, y) | (y, 0) => { /* y bound in both */ }\n    _ => {}\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0408.html"
    }]
};
