// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0695: unlabeled break inside labeled block

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0695",
    title:       LocalizedText::new(
        "Unlabeled break inside labeled block",
        "Немаркированный break внутри маркированного блока",
        "레이블이 지정된 블록 내 레이블 없는 break"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A `break` statement without a label appeared inside a labeled block.
The compiler cannot determine which loop or block you intend to break
out of.

When you have a labeled block with a `break` statement inside it, Rust
requires the `break` to be explicitly labeled to clarify which construct
should be exited.",
        "\
Оператор `break` без метки появился внутри маркированного блока.
Компилятор не может определить, из какого цикла или блока вы намерены
выйти.

Когда у вас есть маркированный блок с оператором `break` внутри, Rust
требует, чтобы `break` был явно помечен, чтобы уточнить, из какой
конструкции следует выйти.",
        "\
레이블이 없는 `break` 문이 레이블이 지정된 블록 내에 나타났습니다.
컴파일러는 어떤 루프나 블록에서 벗어나려는지 결정할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Label the break statement",
                "Добавить метку к оператору break",
                "break 문에 레이블 지정"
            ),
            code:        "'outer: loop {\n    'inner: {\n        break 'outer; // explicit label\n    }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Break the labeled block",
                "Выйти из маркированного блока",
                "레이블이 지정된 블록에서 break"
            ),
            code:        "loop {\n    'a: {\n        break 'a; // break labeled block\n    }\n    break;\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Loop Labels",
            url:   "https://doc.rust-lang.org/reference/expressions/loop-expr.html#loop-labels"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0695.html"
        }
    ]
};
