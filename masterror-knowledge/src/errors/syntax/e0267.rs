// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0267: break or continue inside closure but outside loop

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0267",
    title:       LocalizedText::new(
        "Loop keyword used inside closure but outside loop",
        "Ключевое слово цикла использовано внутри замыкания, но вне цикла",
        "클로저 내부에서 루프 외부에 루프 키워드 사용"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
A loop keyword (`break` or `continue`) was used inside a closure but
outside of any loop.

Loop control keywords are designed to control loop flow. Using them
in a closure without an enclosing loop has no valid target to break
from or continue to, making it a syntax error.",
        "\
Ключевое слово цикла (`break` или `continue`) было использовано внутри
замыкания, но вне какого-либо цикла.

Ключевые слова управления циклом предназначены для управления потоком
цикла. Использование их в замыкании без охватывающего цикла является
синтаксической ошибкой.",
        "\
루프 키워드(`break` 또는 `continue`)가 클로저 내부에서 루프 외부에 사용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use break/continue inside a loop in the closure",
                "Используйте break/continue внутри цикла в замыкании",
                "클로저 내 루프 안에서 break/continue 사용"
            ),
            code:        "let w = || {\n    for _ in 0..10 {\n        break;  // valid - inside loop\n    }\n};"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use return to exit closure early",
                "Используйте return для раннего выхода из замыкания",
                "클로저를 일찍 종료하려면 return 사용"
            ),
            code:        "let w = || {\n    return;  // halts closure execution\n};"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Loops",
            url:   "https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0267.html"
        }
    ]
};
