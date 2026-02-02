// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0164: expected tuple struct/variant, found method

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0164",
    title:       LocalizedText::new(
        "Expected tuple struct/variant, found method",
        "Ожидался кортежный struct/вариант, найден метод",
        "튜플 구조체/변형이 예상되었으나 메서드가 발견됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
You attempted to match something that is neither a tuple struct nor a tuple
variant as a pattern in a match expression. Only tuple structs and tuple
variants are allowed as patterns.

You can only use actual enum variants (unit variants, tuple variants, or
struct variants) as patterns in match expressions - not function calls or
methods, even if they return the same type.",
        "\
Вы попытались использовать в паттерне что-то, что не является ни кортежной
структурой, ни кортежным вариантом. Только кортежные структуры и варианты
можно использовать как паттерны.

В паттернах match можно использовать только варианты перечислений, а не
вызовы функций или методов, даже если они возвращают тот же тип.",
        "\
매치 표현식에서 튜플 구조체나 튜플 변형이 아닌 것을 패턴으로
매칭하려고 했습니다. 튜플 구조체와 튜플 변형만 패턴으로 허용됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use actual enum variants in patterns",
            "Использовать настоящие варианты перечисления в паттернах",
            "패턴에서 실제 열거형 변형 사용"
        ),
        code:        "enum A {\n    B,\n    C,\n}\n\nfn bar(foo: A) {\n    match foo {\n        A::B => (), // ok! B is a unit variant\n        A::C => (),\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Patterns",
            url:   "https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0164.html"
        }
    ]
};
