// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0253: attempt to import unimportable type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0253",
    title:       LocalizedText::new(
        "Attempt to import an unimportable type",
        "Попытка импортировать неимпортируемый тип",
        "임포트할 수 없는 타입을 임포트하려는 시도"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An attempt was made to import a type that belongs to a trait directly.
You cannot directly import associated types from a trait.

Associated types belong to their trait and should be accessed through
the trait itself or through concrete implementations of the trait,
rather than through direct import.

Note: This error code is no longer emitted by the compiler.",
        "\
Была попытка напрямую импортировать тип, принадлежащий трейту.
Нельзя напрямую импортировать ассоциированные типы из трейта.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
트레이트에 속한 타입을 직접 임포트하려고 했습니다.
연관 타입은 직접 임포트할 수 없습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Access associated type through the trait",
            "Обращайтесь к ассоциированному типу через трейт",
            "트레이트를 통해 연관 타입에 접근"
        ),
        code:        "use foo::MyTrait;\n\nfn example<T: MyTrait>() -> T::SomeType { todo!() }"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Associated Types",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0253.html"
        }
    ]
};
