// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0573: expected type, found something else

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0573",
    title:       LocalizedText::new(
        "Expected a type, found something else",
        "Ожидался тип, найдено что-то другое",
        "타입이 예상되었지만 다른 것이 발견됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Something other than a type has been used when one was expected. This commonly
happens when using an enum variant, constant, or other value in a context that
requires a type, such as:
- Return type annotations
- `impl` blocks
- Function parameter type annotations
- Trait method signatures

Always use actual types (structs, enums, traits) in type positions, not their
variants or values.",
        "\
Вместо типа было использовано что-то другое. Это часто происходит при
использовании варианта перечисления, константы или другого значения
в контексте, требующем тип, например:
- Аннотации возвращаемого типа
- Блоки `impl`
- Аннотации типов параметров функций",
        "\
타입이 예상되는 곳에 타입이 아닌 것이 사용되었습니다. 이는 열거형 변형,
상수 또는 다른 값이 다음과 같은 타입이 필요한 컨텍스트에서 사용될 때
자주 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use the enum type, not the variant",
                "Использовать тип перечисления, а не вариант",
                "변형이 아닌 열거형 타입 사용"
            ),
            code:        "fn oblivion() -> Dragon { // not Dragon::Born\n    Dragon::Born\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Create a newtype struct for impl blocks",
                "Создать новый тип структуры для блоков impl",
                "impl 블록을 위한 뉴타입 구조체 생성"
            ),
            code:        "struct Hobbit(u32);\nconst HOBBIT: Hobbit = Hobbit(2);\nimpl Hobbit {} // ok"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0573.html"
    }]
};
