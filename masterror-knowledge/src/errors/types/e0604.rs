// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0604: only `u8` can be cast as `char`

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0604",
    title:       LocalizedText::new(
        "Only u8 can be cast as char",
        "Только u8 можно привести к char",
        "u8만 char로 캐스팅 가능"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type other than `u8` was cast to `char`. Only `u8` can be directly cast
to `char` in Rust.

`char` represents a Unicode Scalar Value, which is an integer value in
ranges 0 to 0xD7FF and 0xE000 to 0x10FFFF. Since only `u8` (0-255) always
fits within valid Unicode ranges, it's the only integer type that can be
directly cast to `char`.",
        "\
Тип, отличный от `u8`, был приведён к `char`. Только `u8` может быть
напрямую приведён к `char` в Rust.

`char` представляет скалярное значение Unicode. Поскольку только `u8` (0-255)
всегда попадает в допустимые диапазоны Unicode, это единственный целочисленный
тип, который можно напрямую привести к `char`.",
        "\
`u8`가 아닌 타입이 `char`로 캐스팅되었습니다. Rust에서는 `u8`만
직접 `char`로 캐스팅할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use char::from_u32() for safe conversion",
                "Использовать char::from_u32() для безопасного преобразования",
                "안전한 변환을 위해 char::from_u32() 사용"
            ),
            code:        "let c = char::from_u32(0x3B1); // Some('α')"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Cast u8 directly",
                "Привести u8 напрямую",
                "u8 직접 캐스팅"
            ),
            code:        "let c = 86u8 as char; // 'V'"
        }
    ],
    links:       &[
        DocLink {
            title: "Type Cast Expressions",
            url:   "https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0604.html"
        }
    ]
};
