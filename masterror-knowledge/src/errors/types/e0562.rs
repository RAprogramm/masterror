// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0562: impl Trait only allowed in function signatures

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0562",
    title:       LocalizedText::new(
        "`impl Trait` only allowed in function signatures",
        "`impl Trait` разрешён только в сигнатурах функций",
        "`impl Trait`는 함수 시그니처에서만 허용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The `impl Trait` syntax can only be used in specific contexts:
- As a function return type
- As a function argument type

It cannot be used in variable type annotations or other contexts like
struct fields or const declarations.",
        "\
Синтаксис `impl Trait` можно использовать только в определённых контекстах:
- Как возвращаемый тип функции
- Как тип аргумента функции

Его нельзя использовать в аннотациях типов переменных или других контекстах.",
        "\
`impl Trait` 구문은 특정 컨텍스트에서만 사용할 수 있습니다:
- 함수 반환 타입으로
- 함수 인수 타입으로

변수 타입 어노테이션이나 다른 컨텍스트에서는 사용할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Move `impl Trait` to function return type",
            "Переместить `impl Trait` в возвращаемый тип функции",
            "`impl Trait`를 함수 반환 타입으로 이동"
        ),
        code:        "fn count_to_n(n: usize) -> impl Iterator<Item=usize> {\n    0..n\n}"
    }],
    links:       &[
        DocLink {
            title: "impl Trait Reference",
            url:   "https://doc.rust-lang.org/stable/reference/types/impl-trait.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0562.html"
        }
    ]
};
