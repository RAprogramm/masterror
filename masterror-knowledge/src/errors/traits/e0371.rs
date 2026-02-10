// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0371: trait implemented on another that already automatically implements it

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0371",
    title:       LocalizedText::new(
        "Trait already automatically implements supertrait",
        "Трейт уже автоматически реализует супертрейт",
        "트레이트가 이미 슈퍼트레이트를 자동으로 구현함"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
When Trait2 is a subtrait of Trait1 (defined as trait Trait2: Trait1 { ... }),
it is not allowed to implement Trait1 for Trait2. This is because Trait2
already automatically implements Trait1 by definition through inheritance,
making an explicit implementation redundant and not useful.

Example: impl Bar for Baz {} is invalid if trait Baz: Bar {}.",
        "\
Когда Trait2 является подтрейтом Trait1 (trait Trait2: Trait1 { ... }),
нельзя реализовать Trait1 для Trait2. Trait2 уже автоматически реализует
Trait1 по определению через наследование, делая явную реализацию избыточной.

Пример: impl Bar for Baz {} недопустим, если trait Baz: Bar {}.",
        "\
Trait2가 Trait1의 서브트레이트일 때(trait Trait2: Trait1 { ... }로 정의),
Trait2에 대해 Trait1을 구현하는 것은 허용되지 않습니다. Trait2는 이미
상속을 통해 정의상 Trait1을 자동으로 구현하므로 명시적 구현은 중복됩니다.

예: trait Baz: Bar {}이면 impl Bar for Baz {}는 유효하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove redundant implementation",
                "Удалить избыточную реализацию",
                "중복 구현 제거"
            ),
            code:        "trait Foo { fn foo(&self) {} }\ntrait Bar: Foo {}\ntrait Baz: Bar {}\n\n// impl Bar for Baz {} // Remove - already implemented\n// impl Foo for Baz {} // Remove - already implemented via Bar"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Supertraits",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0371.html"
        }
    ]
};
