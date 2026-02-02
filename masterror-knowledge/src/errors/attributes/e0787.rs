// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0787: unsupported naked function definition

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0787",
    title:       LocalizedText::new(
        "Unsupported naked function definition",
        "Неподдерживаемое определение naked функции",
        "지원되지 않는 naked 함수 정의"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A naked function was defined incorrectly. Naked functions must follow these rules:
1. Body must contain a single `naked_asm!` block
2. Execution must never fall through past the assembly code
3. Only `att_syntax` and `raw` asm options are allowed
4. Only `const` and `sym` operands are permitted",
        "\
Naked функция определена неправильно. Правила:
1. Тело должно содержать один блок `naked_asm!`
2. Выполнение не должно выходить за пределы ассемблерного кода
3. Только опции `att_syntax` и `raw`
4. Только операнды `const` и `sym`",
        "\
naked 함수가 잘못 정의되었습니다. naked 함수는 다음 규칙을 따라야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use naked_asm! in function body",
                "Используйте naked_asm! в теле функции",
                "함수 본문에 naked_asm! 사용"
            ),
            code:        "#[unsafe(naked)]\npub extern \"C\" fn foo() {\n    naked_asm!(\"ret\");\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 2972",
            url:   "https://github.com/rust-lang/rfcs/blob/master/text/2972-constrained-naked.md"
        }
    ]
};
