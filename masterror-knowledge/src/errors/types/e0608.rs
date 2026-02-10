// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0608: cannot index into a value

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0608",
    title:       LocalizedText::new(
        "Cannot index into a value of this type",
        "Невозможно индексировать значение этого типа",
        "이 타입의 값을 인덱싱할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to index a value whose type doesn't implement the `std::ops::Index`
trait.

Only types that implement `Index` can be indexed with square brackets.
Common indexable types include `Vec<T>`, arrays, and slices.

Note: Tuples and structs use dot notation (`.0`, `.field`), not brackets.",
        "\
Попытка индексировать значение, тип которого не реализует трейт
`std::ops::Index`.

Только типы, реализующие `Index`, могут индексироваться квадратными скобками.
Общие индексируемые типы включают `Vec<T>`, массивы и срезы.

Примечание: Кортежи и структуры используют точечную нотацию (`.0`, `.field`).",
        "\
`std::ops::Index` 트레이트를 구현하지 않는 타입의 값을 인덱싱하려고 시도했습니다.

`Index`를 구현하는 타입만 대괄호로 인덱싱할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use indexable types like Vec or arrays",
                "Использовать индексируемые типы, такие как Vec или массивы",
                "Vec나 배열 같은 인덱싱 가능한 타입 사용"
            ),
            code:        "let v: Vec<u8> = vec![0, 1, 2];\nprintln!(\"{}\", v[1]);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use dot notation for tuples",
                "Использовать точечную нотацию для кортежей",
                "튜플에는 점 표기법 사용"
            ),
            code:        "let tuple = (1, 2, 3);\nprintln!(\"{}\", tuple.0);"
        }
    ],
    links:       &[
        DocLink {
            title: "std::ops::Index",
            url:   "https://doc.rust-lang.org/std/ops/trait.Index.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0608.html"
        }
    ]
};
