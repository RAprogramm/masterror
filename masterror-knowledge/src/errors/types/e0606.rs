// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0606: incompatible cast

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0606",
    title:       LocalizedText::new(
        "Incompatible cast attempted",
        "Попытка несовместимого приведения типа",
        "호환되지 않는 캐스팅 시도"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An incompatible cast was attempted. Only primitive types can be cast into
each other. Attempting to cast non-primitive types (like references)
directly will result in this error.

For example, casting `&u8` directly to `u32` is invalid. You must
dereference first.",
        "\
Была предпринята попытка несовместимого приведения типа. Только примитивные
типы могут быть приведены друг к другу. Попытка привести непримитивные типы
(например, ссылки) напрямую приведёт к этой ошибке.

Например, приведение `&u8` напрямую к `u32` недопустимо. Сначала нужно
разыменовать.",
        "\
호환되지 않는 캐스팅이 시도되었습니다. 원시 타입만 서로 캐스팅할 수 있습니다.
참조와 같은 비원시 타입을 직접 캐스팅하면 이 오류가 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Dereference before casting",
            "Разыменовать перед приведением",
            "캐스팅 전에 역참조"
        ),
        code:        "let x = &0u8;\nlet y: u32 = *x as u32; // dereference first"
    }],
    links:       &[
        DocLink {
            title: "Type Cast Expressions",
            url:   "https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0606.html"
        }
    ]
};
