// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0281: type mismatch in Fn trait requirement

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0281",
    title:       LocalizedText::new(
        "Type mismatch in Fn trait requirement",
        "Несоответствие типов в требовании трейта Fn",
        "Fn 트레이트 요구사항에서 타입 불일치"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
You attempted to supply a type that doesn't implement a required trait
in a location that expects that trait. This typically occurs when
working with `Fn`-based types.

The closure or function parameter types must match the trait requirement
exactly. If a function expects `Fn(usize)` but you provide a closure
that takes `String`, you get a type mismatch.

Note: This error code is no longer emitted by the compiler.",
        "\
Вы попытались предоставить тип, который не реализует требуемый трейт.
Это обычно происходит при работе с типами на основе `Fn`.

Типы параметров замыкания должны точно соответствовать требованию трейта.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
요구된 트레이트를 구현하지 않는 타입을 제공하려고 했습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match closure parameter types to trait requirement",
            "Сопоставьте типы параметров замыкания с требованием трейта",
            "클로저 매개변수 타입을 트레이트 요구사항과 일치시킴"
        ),
        code:        "fn foo<F: Fn(usize)>(x: F) { }\n\nfn main() {\n    foo(|y: usize| { });  // match usize\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Closures",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0281.html"
        }
    ]
};
