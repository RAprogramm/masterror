// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0588: packed type contains aligned field

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0588",
    title:       LocalizedText::new(
        "Packed type contains a field with align repr",
        "Упакованный тип содержит поле с атрибутом align",
        "패킹된 타입에 align repr이 있는 필드 포함"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type with `packed` representation hint has a field with the `align`
representation hint. These are incompatible when nested in this direction.

However, the reverse is allowed: an `align` type can contain a `packed` type.",
        "\
Тип с атрибутом представления `packed` содержит поле с атрибутом `align`.
Эти атрибуты несовместимы при таком вложении.

Однако обратное допустимо: тип с `align` может содержать тип с `packed`.",
        "\
`packed` 표현 힌트가 있는 타입에 `align` 표현 힌트가 있는 필드가 있습니다.
이 방향의 중첩에서는 호환되지 않습니다.

그러나 반대는 허용됩니다: `align` 타입은 `packed` 타입을 포함할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Reverse the nesting: align can contain packed",
            "Изменить вложение: align может содержать packed",
            "중첩 반전: align이 packed를 포함할 수 있음"
        ),
        code:        "#[repr(packed)]\nstruct Packed(i32);\n\n#[repr(align(16))] // align can wrap packed\nstruct Aligned(Packed);"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0588.html"
    }]
};
