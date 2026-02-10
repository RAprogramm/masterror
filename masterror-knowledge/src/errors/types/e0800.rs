// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0800: type or const parameter not in scope

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0800",
    title:       LocalizedText::new(
        "Type or const parameter not in scope",
        "Типовой или const параметр не в области видимости",
        "타입 또는 const 매개변수가 스코프에 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type or const parameter of the given name is not in scope.

This error occurs when you reference a type or const parameter that hasn't
been declared or is not available in the current scope.

Common causes:
- Misspelled type parameter name
- Missing generic declaration in function or type
- Using `use<T>` clause without declaring T",
        "\
Типовой или const параметр с данным именем не находится в области видимости.

Эта ошибка возникает, когда вы ссылаетесь на параметр типа или const,
который не был объявлен или недоступен в текущей области видимости.

Частые причины:
- Опечатка в имени параметра типа
- Отсутствует объявление дженерика в функции или типе
- Использование `use<T>` без объявления T",
        "\
주어진 이름의 타입 또는 const 매개변수가 스코프에 없습니다.
선언되지 않았거나 현재 스코프에서 사용할 수 없는 매개변수를 참조할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Declare the type parameter",
                "Объявить параметр типа",
                "타입 매개변수 선언"
            ),
            code:        "fn missing<T>() -> impl Sized + use<T> {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Check spelling of parameter name",
                "Проверить написание имени параметра",
                "매개변수 이름 철자 확인"
            ),
            code:        "fn example<Item>(x: Item) -> Item { x }"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Generic Types",
            url:   "https://doc.rust-lang.org/book/ch10-01-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0800.html"
        }
    ]
};
