// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0268: break or continue outside of a loop

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0268",
    title:       LocalizedText::new(
        "Loop keyword outside of a loop",
        "Ключевое слово цикла вне цикла",
        "루프 외부에서 루프 키워드 사용"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A loop control keyword (`break` or `continue`) was used outside of a
loop context.

The `break` and `continue` keywords are only valid inside loop
constructs (such as `for`, `while`, or `loop`). Using them outside
a loop has no sensible meaning, so Rust rejects this code.",
        "\
Ключевое слово управления циклом (`break` или `continue`) было использовано
вне контекста цикла.

Ключевые слова `break` и `continue` допустимы только внутри конструкций
цикла (таких как `for`, `while` или `loop`).",
        "\
루프 제어 키워드(`break` 또는 `continue`)가 루프 컨텍스트 외부에서 사용되었습니다.
이 키워드들은 루프 구조 내에서만 유효합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use break/continue inside a loop",
            "Используйте break/continue внутри цикла",
            "루프 내에서 break/continue 사용"
        ),
        code:        "fn some_func() {\n    for _ in 0..10 {\n        break;  // valid inside loop\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Loops",
            url:   "https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0268.html"
        }
    ]
};
