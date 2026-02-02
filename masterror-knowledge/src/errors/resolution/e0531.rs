// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0531: unknown tuple struct or variant

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0531",
    title:       LocalizedText::new(
        "Unknown tuple struct or variant",
        "Неизвестная кортежная структура или вариант",
        "알 수 없는 튜플 구조체 또는 변형"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An unknown tuple struct or variant has been used. This error occurs when you
attempt to use a tuple struct or enum variant that either:
- Hasn't been imported into the current scope
- Is misspelled or doesn't exist
- Is referenced without proper qualification

You need to ensure tuple structs and enum variants are properly accessible.",
        "\
Была использована неизвестная кортежная структура или вариант. Эта ошибка
возникает при попытке использовать кортежную структуру или вариант
перечисления, который:
- Не импортирован в текущую область видимости
- Неправильно написан или не существует
- Указан без правильной квалификации",
        "\
알 수 없는 튜플 구조체 또는 변형이 사용되었습니다. 이 오류는 다음과 같은
튜플 구조체나 열거형 변형을 사용하려고 할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Import the enum variant into scope",
                "Импортировать вариант перечисления в область видимости",
                "열거형 변형을 스코프로 가져오기"
            ),
            code:        "enum Foo { Bar(u32) }\nuse Foo::*; // import variants\n\nmatch Type(12) {\n    Type(x) => {}\n    _ => {}\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use fully qualified path",
                "Использовать полный путь",
                "완전한 경로 사용"
            ),
            code:        "match val { Foo::Bar(x) => {} }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0531.html"
    }]
};
