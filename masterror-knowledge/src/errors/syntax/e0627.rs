// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0627: yield outside coroutine

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0627",
    title:       LocalizedText::new(
        "Yield expression used outside coroutine literal",
        "Выражение yield использовано вне литерала сопрограммы",
        "yield 표현식이 코루틴 리터럴 외부에서 사용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A yield expression was used outside of the coroutine literal. The `yield`
keyword is exclusively meant for use within coroutine literals and cannot
be used in regular functions.

The `yield` keyword allows coroutines to suspend execution and produce
intermediate values.",
        "\
Выражение yield было использовано вне литерала сопрограммы. Ключевое слово
`yield` предназначено исключительно для использования внутри литералов
сопрограмм и не может использоваться в обычных функциях.

Ключевое слово `yield` позволяет сопрограммам приостанавливать выполнение
и производить промежуточные значения.",
        "\
yield 표현식이 코루틴 리터럴 외부에서 사용되었습니다. `yield` 키워드는
코루틴 리터럴 내에서만 사용할 수 있으며 일반 함수에서는 사용할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Wrap yield in coroutine literal",
            "Обернуть yield в литерал сопрограммы",
            "yield를 코루틴 리터럴로 감싸기"
        ),
        code:        "let mut coroutine = #[coroutine] || {\n    yield 1;\n    return \"foo\"\n};"
    }],
    links:       &[
        DocLink {
            title: "Coroutines",
            url:   "https://doc.rust-lang.org/std/ops/trait.Coroutine.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0627.html"
        }
    ]
};
