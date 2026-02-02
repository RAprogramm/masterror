// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0805: invalid number of attribute arguments

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0805",
    title:       LocalizedText::new(
        "Invalid number of attribute arguments",
        "Неверное количество аргументов атрибута",
        "잘못된 속성 인수 개수"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An attribute was given an invalid number of arguments.

Rust attributes have specific requirements for the number of arguments
they accept. For example, the `#[inline]` attribute can either have
no arguments or exactly one argument.

Invalid examples:
- `#[inline()]` - empty parentheses not allowed
- `#[inline(always, never)]` - too many arguments

Valid examples:
- `#[inline]` - no arguments
- `#[inline(always)]` - single argument",
        "\
Атрибуту было передано неверное количество аргументов.

Атрибуты Rust имеют определённые требования к количеству аргументов.
Например, атрибут `#[inline]` может либо не иметь аргументов,
либо иметь ровно один аргумент.

Неверные примеры:
- `#[inline()]` - пустые скобки не разрешены
- `#[inline(always, never)]` - слишком много аргументов

Верные примеры:
- `#[inline]` - без аргументов
- `#[inline(always)]` - один аргумент",
        "\
속성에 잘못된 수의 인수가 전달되었습니다.
Rust 속성은 허용하는 인수 수에 대해 특정 요구 사항이 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use attribute without parentheses",
                "Использовать атрибут без скобок",
                "괄호 없이 속성 사용"
            ),
            code:        "#[inline]\nfn foo() {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use attribute with single argument",
                "Использовать атрибут с одним аргументом",
                "단일 인수로 속성 사용"
            ),
            code:        "#[inline(always)]\nfn foo() {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Attributes",
            url:   "https://doc.rust-lang.org/reference/attributes.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0805.html"
        }
    ]
};
