// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0639: cannot instantiate non-exhaustive type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0639",
    title:       LocalizedText::new(
        "Cannot instantiate non-exhaustive type from outside crate",
        "Невозможно создать экземпляр неисчерпывающего типа извне крейта",
        "크레이트 외부에서 비완전 타입을 인스턴스화할 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Attempted to instantiate a struct, enum, or enum variant from outside its
defining crate when it has been marked with `#[non_exhaustive]`.

The `#[non_exhaustive]` attribute signals that a type may have additional
fields or variants added in future versions. To preserve backward
compatibility, Rust prevents external code from directly instantiating
these types using struct literals.",
        "\
Попытка создать экземпляр структуры, enum или варианта enum извне его
определяющего крейта, когда он помечен `#[non_exhaustive]`.

Атрибут `#[non_exhaustive]` сигнализирует, что тип может иметь
дополнительные поля или варианты, добавленные в будущих версиях. Для
сохранения обратной совместимости Rust запрещает внешнему коду напрямую
создавать экземпляры этих типов с помощью литералов структур.",
        "\
`#[non_exhaustive]`로 표시된 구조체, enum 또는 enum 변형을
정의하는 크레이트 외부에서 인스턴스화하려고 시도했습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use the constructor function provided by the crate",
            "Использовать функцию-конструктор, предоставленную крейтом",
            "크레이트에서 제공하는 생성자 함수 사용"
        ),
        code:        "// Check the crate's documentation for a `new` or similar constructor\nlet instance = SomeType::new();"
    }],
    links:       &[
        DocLink {
            title: "Non-exhaustive Attribute",
            url:   "https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0639.html"
        }
    ]
};
