// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0571: break with value in non-loop

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0571",
    title:       LocalizedText::new(
        "`break` with value from non-`loop` loop",
        "`break` со значением в не-`loop` цикле",
        "비`loop` 루프에서 값이 있는 `break`"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A `break` statement with an argument appeared in a non-`loop` loop. The
`break` statement can only take an argument (a value to return from the loop)
when used inside `loop` loops.

It cannot be used with a value in `for`, `while`, or `while let` loops.",
        "\
Оператор `break` с аргументом появился в цикле, отличном от `loop`.
Оператор `break` может принимать аргумент (значение для возврата из цикла)
только внутри циклов `loop`.

Его нельзя использовать со значением в циклах `for`, `while` или `while let`.",
        "\
`loop`가 아닌 루프에서 인수가 있는 `break` 문이 나타났습니다. `break` 문은
`loop` 루프 내에서만 인수(루프에서 반환할 값)를 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Change while to loop",
            "Изменить while на loop",
            "while를 loop로 변경"
        ),
        code:        "let result = loop {\n    if satisfied(i) {\n        break 2 * i; // ok in loop\n    }\n    i += 1;\n};"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0571.html"
    }]
};
