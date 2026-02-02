// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0628: too many parameters for coroutine

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0628",
    title:       LocalizedText::new(
        "More than one parameter used for coroutine",
        "Более одного параметра использовано для сопрограммы",
        "코루틴에 둘 이상의 매개변수 사용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
More than one parameter was used for a coroutine. Rust coroutines are
restricted to accepting either 0 or 1 parameter.

If you need multiple values, consider using a tuple or struct as a single
parameter.",
        "\
Для сопрограммы было использовано более одного параметра. Сопрограммы Rust
ограничены приёмом 0 или 1 параметра.

Если вам нужно несколько значений, рассмотрите использование кортежа или
структуры в качестве единственного параметра.",
        "\
코루틴에 둘 이상의 매개변수가 사용되었습니다. Rust 코루틴은 0개 또는
1개의 매개변수만 받을 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use at most one parameter",
                "Использовать не более одного параметра",
                "최대 하나의 매개변수 사용"
            ),
            code:        "let coroutine = #[coroutine] |a: i32| {\n    yield a;\n};"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a tuple for multiple values",
                "Использовать кортеж для нескольких значений",
                "여러 값에 튜플 사용"
            ),
            code:        "let coroutine = #[coroutine] |params: (i32, i32)| {\n    let (a, b) = params;\n    yield a + b;\n};"
        }
    ],
    links:       &[
        DocLink {
            title: "Coroutines",
            url:   "https://doc.rust-lang.org/std/ops/trait.Coroutine.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0628.html"
        }
    ]
};
