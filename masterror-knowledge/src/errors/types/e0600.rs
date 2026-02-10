// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0600: cannot apply unary operator to type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0600",
    title:       LocalizedText::new(
        "Cannot apply unary operator to type",
        "Нельзя применить унарный оператор к типу",
        "타입에 단항 연산자를 적용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An unary operator was used on a type which doesn't implement it. Unary
operators like `!`, `-`, `*`, `&` require the type to implement the
corresponding trait from `std::ops`.

Different unary operators require different trait implementations:
- `!` requires `std::ops::Not`
- `-` requires `std::ops::Neg`
- `*` requires `std::ops::Deref`",
        "\
Унарный оператор был использован для типа, который его не реализует.
Унарные операторы вроде `!`, `-`, `*`, `&` требуют, чтобы тип реализовывал
соответствующий трейт из `std::ops`.",
        "\
타입이 구현하지 않은 단항 연산자가 사용되었습니다. `!`, `-`, `*`, `&`와 같은
단항 연산자는 해당 타입이 `std::ops`의 해당 트레이트를 구현해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Implement the appropriate operator trait",
            "Реализовать соответствующий трейт оператора",
            "적절한 연산자 트레이트 구현"
        ),
        code:        "use std::ops::Not;\n\nimpl Not for Question {\n    type Output = bool;\n    fn not(self) -> bool { matches!(self, Question::No) }\n}"
    }],
    links:       &[
        DocLink {
            title: "std::ops module",
            url:   "https://doc.rust-lang.org/std/ops/index.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0600.html"
        }
    ]
};
