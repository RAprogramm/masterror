// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0727: `yield` used in async context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0727",
    title:       LocalizedText::new(
        "yield clause used in async context",
        "yield используется в асинхронном контексте",
        "async 컨텍스트에서 yield 사용됨"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
A `yield` clause was used in an `async` context. The `yield` keyword is used
for coroutines/generators, but mixing it with async blocks is not supported.

The async machinery handles its own suspension points via `.await`, and
coroutine yields are a separate mechanism that cannot be mixed.",
        "\
Ключевое слово `yield` было использовано в `async` контексте.
`yield` используется для корутин/генераторов, но смешивание с async блоками
не поддерживается.",
        "\
`async` 컨텍스트에서 `yield` 절이 사용되었습니다. `yield` 키워드는
코루틴/제너레이터용이며 async 블록과 혼합할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Move yield outside async block",
                "Переместите yield за пределы async блока",
                "yield를 async 블록 밖으로 이동"
            ),
            code:        "#[coroutine] || {\n    yield;\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0727.html"
        }
    ]
};
