// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0575: expected type or associated type, found something else

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0575",
    title:       LocalizedText::new(
        "Expected type or associated type, found something else",
        "Ожидался тип или ассоциированный тип",
        "타입 또는 연관 타입이 예상되었지만 다른 것 발견"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Something other than a type or an associated type was given where one was
expected. This error occurs when you attempt to use a non-type construct
(such as an enum variant or trait method) in a context that requires a type
annotation.

Use the enum itself as the type, not the variant. For traits, use the
associated type, not methods.",
        "\
Вместо типа или ассоциированного типа было указано что-то другое.
Эта ошибка возникает при попытке использовать не-типовую конструкцию
(например, вариант перечисления или метод трейта) в контексте,
требующем аннотации типа.",
        "\
타입 또는 연관 타입이 예상되는 곳에 다른 것이 주어졌습니다.
이 오류는 열거형 변형이나 트레이트 메서드와 같은 비타입 구성을
타입 어노테이션이 필요한 컨텍스트에서 사용하려고 할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use the enum type, not its variant",
                "Использовать тип перечисления, а не вариант",
                "변형이 아닌 열거형 타입 사용"
            ),
            code:        "enum Rick { Morty }\nlet _: Rick; // not <u8 as Rick>::Morty"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use associated type, not method",
                "Использовать ассоциированный тип, а не метод",
                "메서드가 아닌 연관 타입 사용"
            ),
            code:        "let _: <u8 as Age>::Empire; // not ::Mythology"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0575.html"
    }]
};
