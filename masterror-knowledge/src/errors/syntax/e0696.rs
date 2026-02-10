// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0696: continue outside loop

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0696",
    title:       LocalizedText::new(
        "continue used incorrectly",
        "continue использован неправильно",
        "continue가 잘못 사용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A function is using `continue` keyword incorrectly. The `continue` keyword
can only be used inside loops (`loop`, `while`, `for`).

Using `continue` in a labeled block that is NOT a loop causes this error.
Using `continue` to jump to a labeled block (rather than a loop) is also
invalid.",
        "\
Функция использует ключевое слово `continue` неправильно. Ключевое слово
`continue` может использоваться только внутри циклов (`loop`, `while`, `for`).

Использование `continue` в маркированном блоке, который НЕ является циклом,
вызывает эту ошибку. Использование `continue` для перехода к маркированному
блоку (а не к циклу) также недопустимо.",
        "\
함수가 `continue` 키워드를 잘못 사용하고 있습니다. `continue` 키워드는
루프(`loop`, `while`, `for`) 내에서만 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use continue inside a loop",
            "Использовать continue внутри цикла",
            "루프 내에서 continue 사용"
        ),
        code:        "'b: loop {\n    continue 'b; // ok - 'b is a loop\n}"
    }],
    links:       &[
        DocLink {
            title: "continue Expression",
            url:   "https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0696.html"
        }
    ]
};
