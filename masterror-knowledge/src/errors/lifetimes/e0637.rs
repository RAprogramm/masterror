// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0637: underscore lifetime used illegally

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0637",
    title:       LocalizedText::new(
        "'_ lifetime used in illegal place",
        "Время жизни '_ использовано в недопустимом месте",
        "'_ 라이프타임이 잘못된 위치에서 사용됨"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The `'_` lifetime name or `&T` without an explicit lifetime name has been
used in an illegal place.

The `'_` lifetime is reserved for the anonymous lifetime and cannot be
used as a named lifetime identifier in some places. Similarly, `&T` without
an explicit lifetime name is not permitted in certain contexts like trait
bounds and where clauses.",
        "\
Имя времени жизни `'_` или `&T` без явного имени времени жизни было
использовано в недопустимом месте.

Время жизни `'_` зарезервировано для анонимного времени жизни и не может
использоваться как именованный идентификатор времени жизни в некоторых
местах. Аналогично, `&T` без явного имени времени жизни не допускается
в определённых контекстах, таких как ограничения трейтов и where-предложения.",
        "\
`'_` 라이프타임 이름 또는 명시적 라이프타임 이름이 없는 `&T`가
잘못된 위치에서 사용되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use explicit lifetime name",
                "Использовать явное имя времени жизни",
                "명시적 라이프타임 이름 사용"
            ),
            code:        "fn foo<'a>(str1: &'a str, str2: &'a str) -> &'a str { }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use higher-ranked trait bounds",
                "Использовать ограничения трейтов высшего ранга",
                "고차 트레이트 바운드 사용"
            ),
            code:        "fn foo<T>()\nwhere\n    T: for<'a> Into<&'a u32>,\n{}"
        }
    ],
    links:       &[
        DocLink {
            title: "Lifetime Elision",
            url:   "https://doc.rust-lang.org/reference/lifetime-elision.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0637.html"
        }
    ]
};
