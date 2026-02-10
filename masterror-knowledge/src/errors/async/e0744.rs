// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0744: await in const context (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0744",
    title:       LocalizedText::new(
        "Await used in const context",
        "Await используется в const контексте",
        "const 컨텍스트에서 await 사용"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

Previously, `.await` was forbidden inside a `const`, `static`, or `const fn`.
This restriction may be lifted in future Rust versions.",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Ранее `.await` был запрещён внутри `const`, `static` или `const fn`.",
        "\
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Move async code outside const context",
                "Переместите async код за пределы const контекста",
                "async 코드를 const 컨텍스트 밖으로 이동"
            ),
            code:        "async fn compute() -> i32 {\n    async { 0 }.await\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0744.html"
        }
    ]
};
