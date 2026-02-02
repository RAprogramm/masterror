// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0055: auto-deref recursion limit exceeded

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0055",
    title:       LocalizedText::new(
        "Auto-deref recursion limit exceeded",
        "Превышен лимит рекурсии автоматического разыменования",
        "자동 역참조 재귀 한도 초과"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when automatic dereferencing during method calls exceeds the
compiler's recursion limit. Rust automatically dereferences values to match
method receivers, but has a limit on how deep this can go.

Example:
    let ref_foo = &&&&&Foo;
    ref_foo.method();  // Error if recursion limit is less than 5",
        "\
Эта ошибка возникает, когда автоматическое разыменование при вызове методов
превышает лимит рекурсии компилятора.",
        "\
이 오류는 메서드 호출 중 자동 역참조가 컴파일러의 재귀 한도를 초과할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Increase recursion limit",
                "Увеличить лимит рекурсии",
                "재귀 한도 증가"
            ),
            code:        "#![recursion_limit=\"128\"]"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Manually dereference",
                "Разыменовать вручную",
                "수동으로 역참조"
            ),
            code:        "let ref_foo = &&&&&Foo;\n(*****ref_foo).method();"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0055.html"
    }]
};
