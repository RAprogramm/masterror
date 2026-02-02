// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0581: lifetime appears only in return type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0581",
    title:       LocalizedText::new(
        "Lifetime appears only in return type",
        "Время жизни появляется только в возвращаемом типе",
        "라이프타임이 반환 타입에서만 나타남"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
In a `fn` type, a lifetime parameter must appear in both the argument types
and return type. If a lifetime appears only in the return type and not in
the arguments, it's impossible to determine how long the lifetime should
actually be, since there's nothing constraining it.

This restriction ensures the compiler can properly track lifetime relationships.",
        "\
В типе `fn` параметр времени жизни должен появляться как в типах аргументов,
так и в возвращаемом типе. Если время жизни появляется только в возвращаемом
типе, невозможно определить, каким оно должно быть на самом деле.",
        "\
`fn` 타입에서 라이프타임 매개변수는 인수 타입과 반환 타입 모두에 나타나야 합니다.
라이프타임이 인수가 아닌 반환 타입에만 나타나면 라이프타임이 실제로 얼마나
길어야 하는지 결정할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use lifetime in both arguments and return type",
                "Использовать время жизни и в аргументах, и в возвращаемом типе",
                "인수와 반환 타입 모두에서 라이프타임 사용"
            ),
            code:        "let x: for<'a> fn(&'a i32) -> &'a i32;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use 'static lifetime for return-only case",
                "Использовать 'static для случая только возврата",
                "반환 전용 케이스에 'static 라이프타임 사용"
            ),
            code:        "let y: fn() -> &'static i32;"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0581.html"
    }]
};
