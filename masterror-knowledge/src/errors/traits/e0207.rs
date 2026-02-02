// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0207: unconstrained type parameter in impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0207",
    title:       LocalizedText::new(
        "Unconstrained type parameter in impl",
        "Неограниченный параметр типа в impl",
        "impl에서 제약되지 않은 타입 매개변수"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A type, const, or lifetime parameter specified for `impl` is not constrained.
Rust requires that all parameters in an `impl` must be 'constrained' by:

1. Appearing in the implementing type (e.g., `impl<T> Foo<T>`)
2. Appearing in the implemented trait (e.g., `impl<T> SomeTrait<T> for Foo`)
3. Being bound as an associated type

If a type parameter appears only in method signatures, not in the impl
parameters or the type being implemented for, you get this error.",
        "\
Параметр типа, константы или времени жизни, указанный для `impl`,
не ограничен. Rust требует, чтобы все параметры в `impl` были
ограничены появлением в реализуемом типе или трейте.",
        "\
`impl`에 지정된 타입, const 또는 수명 매개변수가 제약되지 않았습니다.
모든 매개변수는 구현 타입이나 트레이트에 나타나야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Move type parameter to method",
                "Переместить параметр типа в метод",
                "타입 매개변수를 메서드로 이동"
            ),
            code:        "impl Foo {\n    fn get<T: Default>(&self) -> T {\n        T::default()\n    }\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use PhantomData to carry the type",
                "Используйте PhantomData для переноса типа",
                "PhantomData를 사용하여 타입 전달"
            ),
            code:        "use std::marker::PhantomData;\n\nstruct Foo<T>(PhantomData<T>);\n\nimpl<T: Default> Foo<T> {\n    fn get(&self) -> T { T::default() }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Implementations",
            url:   "https://doc.rust-lang.org/reference/items/implementations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0207.html"
        }
    ]
};
