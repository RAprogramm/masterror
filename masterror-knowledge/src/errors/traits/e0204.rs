// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0204: Copy trait on type with non-Copy fields

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0204",
    title:       LocalizedText::new(
        "Copy trait on type with non-Copy fields",
        "Трейт Copy для типа с не-Copy полями",
        "non-Copy 필드를 가진 타입에 Copy 트레이트"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Cannot implement `Copy` trait on types containing non-`Copy` fields.
The `Copy` trait can only be implemented on types whose ALL fields
implement `Copy`.

Common non-Copy types include:
- Vec<T>, String, Box<T>
- Mutable references (&mut T)
- Types containing the above",
        "\
Нельзя реализовать трейт `Copy` для типов, содержащих не-`Copy` поля.
Трейт `Copy` может быть реализован только для типов, ВСЕ поля которых
реализуют `Copy`.

Распространённые не-Copy типы:
- Vec<T>, String, Box<T>
- Изменяемые ссылки (&mut T)",
        "\
non-`Copy` 필드를 포함하는 타입에는 `Copy` 트레이트를 구현할 수 없습니다.
모든 필드가 `Copy`를 구현해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Ensure all fields implement Copy",
                "Убедитесь, что все поля реализуют Copy",
                "모든 필드가 Copy를 구현하는지 확인"
            ),
            code:        "struct Foo {\n    x: i32,  // Copy\n    y: bool, // Copy\n}\nimpl Copy for Foo {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use Clone instead of Copy",
                "Используйте Clone вместо Copy",
                "Copy 대신 Clone 사용"
            ),
            code:        "#[derive(Clone)]\nstruct Foo {\n    data: Vec<u32>,\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Copy Trait",
            url:   "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0204.html"
        }
    ]
};
