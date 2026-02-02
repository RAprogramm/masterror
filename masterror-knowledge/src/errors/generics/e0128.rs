// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0128: generic parameters with a default cannot use forward declared
//! identifiers

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0128",
    title:       LocalizedText::new(
        "Generic parameters with a default cannot use forward declared identifiers",
        "Обобщённые параметры со значением по умолчанию не могут использовать ещё не объявленные идентификаторы",
        "기본값이 있는 제네릭 매개변수는 전방 선언된 식별자를 사용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type parameter with default value is using a forward declared identifier.
Type parameter defaults can only reference parameters that are declared
before them. Since type parameters are evaluated in order, attempting to
use a not-yet-defined identifier in a default value causes this error.",
        "\
Параметр типа со значением по умолчанию использует ещё не объявленный
идентификатор. Значения по умолчанию для параметров типа могут ссылаться
только на параметры, объявленные до них. Поскольку параметры типа
вычисляются по порядку, использование ещё не определённого идентификатора
вызывает эту ошибку.",
        "\
기본값이 있는 타입 매개변수가 전방 선언된 식별자를 사용하고 있습니다.
타입 매개변수 기본값은 이전에 선언된 매개변수만 참조할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Reorder type parameters so referenced ones come first",
            "Переупорядочить параметры типа так, чтобы используемые шли первыми",
            "참조되는 매개변수가 먼저 오도록 타입 매개변수 재정렬"
        ),
        code:        "struct Foo<U = (), T = U> {\n    field1: T,\n    field2: U,\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Type Parameters",
            url:   "https://doc.rust-lang.org/reference/items/generics.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0128.html"
        }
    ]
};
