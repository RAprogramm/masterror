// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0297: refutable pattern in for loop

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0297",
    title:       LocalizedText::new(
        "Refutable pattern in for loop",
        "Опровержимый паттерн в цикле for",
        "for 루프에서 반박 가능한 패턴"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
Patterns used to bind names in loops are refutable (don't guarantee
a match in all cases). Patterns in loops must be irrefutable, meaning
they must successfully extract a name in every iteration.

For example, `for Some(x) in xs` is refutable because it only matches
`Some` variants but not `None`. When the loop encounters `None`, the
pattern fails to bind anything.

Note: This error code is no longer emitted by the compiler.",
        "\
Паттерны, используемые для связывания имён в циклах, являются опровержимыми
(не гарантируют совпадение во всех случаях). Паттерны в циклах должны
быть неопровержимыми.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
루프에서 이름을 바인딩하는 데 사용된 패턴이 반박 가능합니다.
루프의 패턴은 반박 불가능해야 합니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use if let inside the loop",
                "Используйте if let внутри цикла",
                "루프 내에서 if let 사용"
            ),
            code:        "for item in xs {\n    if let Some(x) = item {\n        // use x\n    }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use match inside the loop",
                "Используйте match внутри цикла",
                "루프 내에서 match 사용"
            ),
            code:        "for item in xs {\n    match item {\n        Some(x) => {},\n        None => {},\n    }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Refutability",
            url:   "https://doc.rust-lang.org/book/ch18-02-refutability.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0297.html"
        }
    ]
};
