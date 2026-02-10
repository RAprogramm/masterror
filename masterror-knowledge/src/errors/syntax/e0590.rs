// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0590: break/continue in while condition without label

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0590",
    title:       LocalizedText::new(
        "`break`/`continue` in while condition without label",
        "`break`/`continue` в условии while без метки",
        "레이블 없이 while 조건에서 `break`/`continue`"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
`break` or `continue` keywords were used in a condition of a `while` loop
without a label. When using these keywords inside the condition of a while
loop, you must provide a label to specify which loop is being targeted.

Using `break` or `continue` without a label in a while condition is ambiguous.",
        "\
Ключевые слова `break` или `continue` были использованы в условии цикла
`while` без метки. При использовании этих ключевых слов внутри условия
цикла while необходимо указать метку, чтобы определить целевой цикл.",
        "\
`break` 또는 `continue` 키워드가 레이블 없이 `while` 루프의 조건에서
사용되었습니다. while 루프의 조건 내에서 이러한 키워드를 사용할 때
대상 루프를 지정하는 레이블을 제공해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add a label to the while loop",
            "Добавить метку к циклу while",
            "while 루프에 레이블 추가"
        ),
        code:        "'foo: while break 'foo {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0590.html"
    }]
};
