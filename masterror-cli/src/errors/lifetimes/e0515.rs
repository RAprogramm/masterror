// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0515: cannot return reference to temporary value

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0515",
    title:       LocalizedText::new(
        "Cannot return reference to temporary value",
        "Нельзя вернуть ссылку на временное значение",
        "임시 값에 대한 참조를 반환할 수 없음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
You're trying to return a reference to a value that was created inside
the function. When the function returns, that value is dropped.

The reference would point to freed memory - a dangling pointer.",
        "\
Вы пытаетесь вернуть ссылку на значение, созданное внутри функции.
При возврате из функции это значение будет уничтожено.",
        "\
함수 내에서 생성된 값에 대한 참조를 반환하려고 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Return owned value instead of reference",
                "Вернуть владеющее значение вместо ссылки",
                "참조 대신 소유 값 반환"
            ),
            code:        "fn create() -> String { String::from(\"hello\") }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a parameter lifetime",
                "Использовать время жизни параметра",
                "매개변수 라이프타임 사용"
            ),
            code:        "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0515.html"
    }]
};
