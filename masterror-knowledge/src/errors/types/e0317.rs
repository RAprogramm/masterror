// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0317: if expression is missing an else block

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0317",
    title:       LocalizedText::new(
        "If expression is missing an else block",
        "Выражение if не имеет блока else",
        "if 표현식에 else 블록이 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An if expression without an else block is used in a context where a type
other than () is expected. An if expression without else has type (),
which causes a type mismatch when the surrounding code expects a different
type value.

Example: let a = if x == 5 { 1 }; fails because without else, the if
returns () when the condition is false, not an integer.",
        "\
Выражение if без блока else используется в контексте, где ожидается тип
отличный от (). Выражение if без else имеет тип (), что вызывает
несоответствие типов, когда окружающий код ожидает другое значение.",
        "\
else 블록이 없는 if 표현식이 () 이외의 타입이 예상되는 컨텍스트에서 사용되었습니다.
else가 없는 if 표현식은 () 타입을 가지며, 주변 코드가 다른 타입 값을 기대할 때
타입 불일치가 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add else block with same return type",
                "Добавить блок else с тем же типом возврата",
                "동일한 반환 타입의 else 블록 추가"
            ),
            code:        "let a = if x == 5 {\n    1\n} else {\n    2\n};"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: if Expressions",
            url:   "https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0317.html"
        }
    ]
};
