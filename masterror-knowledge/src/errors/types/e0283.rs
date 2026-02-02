// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0283: type annotation needed due to ambiguity

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0283",
    title:       LocalizedText::new(
        "Type annotation needed due to ambiguity",
        "Требуется аннотация типа из-за неоднозначности",
        "모호성으로 인해 타입 어노테이션 필요"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The compiler could not infer a type due to ambiguity - multiple types
could satisfy the requirements.

This typically happens with generic methods like `collect()` on iterators,
which have multiple valid implementations. The compiler cannot decide
between valid options without explicit type information.",
        "\
Компилятор не смог вывести тип из-за неоднозначности - несколько
типов могут удовлетворять требованиям.

Это часто происходит с методами вроде `collect()` на итераторах,
которые имеют несколько допустимых реализаций.",
        "\
컴파일러가 모호성으로 인해 타입을 추론할 수 없습니다.
여러 타입이 요구사항을 충족할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add a type annotation",
                "Добавьте аннотацию типа",
                "타입 어노테이션 추가"
            ),
            code:        "let x: Vec<char> = \"hello\".chars().rev().collect();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use turbofish syntax",
                "Используйте синтаксис turbofish",
                "터보피시 구문 사용"
            ),
            code:        "let x = \"hello\".chars().rev().collect::<Vec<char>>();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use partial type annotation with placeholder",
                "Используйте частичную аннотацию с заполнителем",
                "플레이스홀더와 부분 타입 어노테이션 사용"
            ),
            code:        "let x: Vec<_> = \"hello\".chars().rev().collect();"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Type Annotations",
            url:   "https://doc.rust-lang.org/book/ch03-02-data-types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0283.html"
        }
    ]
};
