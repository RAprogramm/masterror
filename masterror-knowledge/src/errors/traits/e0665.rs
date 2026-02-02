// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0665: Default derive on enum without default variant

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0665",
    title:       LocalizedText::new(
        "Default derived on enum without specifying default variant",
        "Default производный для enum без указания варианта по умолчанию",
        "기본 변형을 지정하지 않고 enum에 Default 파생"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
The `Default` trait was derived on an enum without explicitly designating
which variant should be used as the default value.

The Rust compiler cannot automatically determine which enum variant should
be the default because enums can have multiple variants. Unlike structs
(where `Default` can be derived if all fields implement `Default`), there's
no obvious choice for which variant to use.",
        "\
Трейт `Default` был выведен для enum без явного указания, какой вариант
должен использоваться как значение по умолчанию.

Компилятор Rust не может автоматически определить, какой вариант enum
должен быть по умолчанию, потому что у enum может быть несколько вариантов.
В отличие от структур (где `Default` может быть выведен, если все поля
реализуют `Default`), нет очевидного выбора для варианта.",
        "\
어떤 변형이 기본값으로 사용되어야 하는지 명시적으로 지정하지 않고
enum에서 `Default` 트레이트가 파생되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Annotate the default variant",
                "Аннотировать вариант по умолчанию",
                "기본 변형 주석 추가"
            ),
            code:        "#[derive(Default)]\nenum Food {\n    #[default]\n    Sweet,\n    Salty,\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Manually implement Default",
                "Реализовать Default вручную",
                "Default 수동 구현"
            ),
            code:        "impl Default for Food {\n    fn default() -> Food {\n        Food::Sweet\n    }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Default Trait",
            url:   "https://doc.rust-lang.org/std/default/trait.Default.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0665.html"
        }
    ]
};
