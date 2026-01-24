// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0106: missing lifetime specifier

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0106",
    title:       LocalizedText::new(
        "Missing lifetime specifier",
        "Отсутствует спецификатор времени жизни",
        "라이프타임 지정자 누락"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
References in Rust have lifetimes - they describe how long the reference
is valid. Usually the compiler infers lifetimes, but sometimes you must
be explicit.

Lifetime annotations don't change how long values live. They describe
relationships between references so the compiler can verify safety.",
        "\
Ссылки в Rust имеют времена жизни — они описывают, как долго ссылка
действительна. Обычно компилятор выводит времена жизни, но иногда нужно
указать явно.",
        "\
Rust의 참조에는 라이프타임이 있습니다 - 참조가 얼마나 오래 유효한지 설명합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add explicit lifetime parameter",
                "Добавить явный параметр времени жизни",
                "명시적 라이프타임 매개변수 추가"
            ),
            code:        "struct Foo<'a> { x: &'a str }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use owned type instead",
                "Использовать владеющий тип",
                "소유 타입 사용"
            ),
            code:        "struct Foo { x: String }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use 'static for compile-time constants",
                "Использовать 'static для констант",
                "컴파일 시간 상수에 'static 사용"
            ),
            code:        "fn get_str() -> &'static str { \"hello\" }"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Lifetimes",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0106.html"
        }
    ]
};
