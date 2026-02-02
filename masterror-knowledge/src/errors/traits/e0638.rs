// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0638: non-exhaustive type matched exhaustively

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0638",
    title:       LocalizedText::new(
        "Non-exhaustive type must be matched non-exhaustively",
        "Неисчерпывающий тип должен сопоставляться неисчерпывающе",
        "비완전 타입은 비완전하게 매치해야 함"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A struct, enum, or enum variant marked with `#[non_exhaustive]` was
matched exhaustively. The `#[non_exhaustive]` attribute allows library
authors to add new variants or fields in future versions without breaking
downstream code.

Downstream crates must:
- Use a wildcard `_` pattern when matching non-exhaustive enums
- Use the `..` pattern when matching non-exhaustive structs",
        "\
Структура, enum или вариант enum, помеченный `#[non_exhaustive]`, был
сопоставлен исчерпывающе. Атрибут `#[non_exhaustive]` позволяет авторам
библиотек добавлять новые варианты или поля в будущих версиях, не ломая
зависимый код.

Зависимые крейты должны:
- Использовать шаблон подстановки `_` при сопоставлении non-exhaustive enums
- Использовать шаблон `..` при сопоставлении non-exhaustive структур",
        "\
`#[non_exhaustive]`로 표시된 구조체, enum 또는 enum 변형이
완전하게 매치되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use wildcard pattern for enums",
                "Использовать шаблон подстановки для enums",
                "enum에 와일드카드 패턴 사용"
            ),
            code:        "match error {\n    Error::Message(s) => {},\n    Error::Other => {},\n    _ => {}, // required for non_exhaustive\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use .. pattern for structs",
                "Использовать шаблон .. для структур",
                "구조체에 .. 패턴 사용"
            ),
            code:        "match my_struct {\n    MyStruct { field1, .. } => {},\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Non-exhaustive Attribute",
            url:   "https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0638.html"
        }
    ]
};
