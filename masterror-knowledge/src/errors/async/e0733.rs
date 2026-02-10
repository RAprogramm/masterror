// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0733: async recursion without boxing

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0733",
    title:       LocalizedText::new(
        "Async recursion without boxing",
        "Асинхронная рекурсия без упаковки",
        "박싱 없는 async 재귀"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
An `async` function called itself recursively without boxing.

When an async function calls itself recursively, the compiler cannot determine
the size of the future at compile time, because each recursive call creates
a new future. Without boxing, the compiler cannot allocate the necessary memory.",
        "\
Асинхронная функция вызывает себя рекурсивно без упаковки.

Компилятор не может определить размер future во время компиляции,
так как каждый рекурсивный вызов создаёт новый future.",
        "\
`async` 함수가 박싱 없이 자신을 재귀적으로 호출했습니다.

컴파일러가 컴파일 시점에 future 크기를 결정할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Box the recursive call",
                "Упакуйте рекурсивный вызов",
                "재귀 호출을 박싱"
            ),
            code:        "async fn foo(n: usize) {\n    if n > 0 {\n        Box::pin(foo(n - 1)).await;\n    }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Return boxed future",
                "Верните упакованный future",
                "박싱된 future 반환"
            ),
            code:        "fn foo(n: usize) -> Pin<Box<dyn Future<Output = ()>>> {\n    Box::pin(async move {\n        if n > 0 { foo(n - 1).await; }\n    })\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0733.html"
        }
    ]
};
