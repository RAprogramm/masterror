// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0728: await used outside async context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0728",
    title:       LocalizedText::new(
        "await used outside async context",
        "await используется вне асинхронного контекста",
        "async 컨텍스트 외부에서 await 사용"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
The `await` keyword was used outside of an `async` function or `async` block.

The `await` keyword is used to suspend the current computation until the
given future is ready to produce a value. It is only legal within an async
context, such as an `async fn` or an `async` block.",
        "\
Ключевое слово `await` используется вне `async` функции или `async` блока.

`await` приостанавливает текущее вычисление до готовности future.
Оно допустимо только в async контексте.",
        "\
`await` 키워드가 `async` 함수나 `async` 블록 외부에서 사용되었습니다.

`await`는 현재 계산을 일시 중지하며 async 컨텍스트 내에서만 유효합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use await inside async function",
                "Используйте await внутри async функции",
                "async 함수 내에서 await 사용"
            ),
            code:        "async fn foo() {\n    some_future().await;\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use await inside async block",
                "Используйте await внутри async блока",
                "async 블록 내에서 await 사용"
            ),
            code:        "fn bar() -> impl Future<Output = u8> {\n    async {\n        some_future().await\n    }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0728.html"
        }
    ]
};
