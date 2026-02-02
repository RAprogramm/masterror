// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0567: auto traits cannot have generic parameters

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0567",
    title:       LocalizedText::new(
        "Auto traits cannot have generic parameters",
        "Автоматические трейты не могут иметь обобщённые параметры",
        "자동 트레이트는 제네릭 매개변수를 가질 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Generics have been used on an auto trait. Auto traits cannot have generic
parameters because they are automatically implemented on all existing types.
The compiler cannot infer what types should be used for the trait's generic
parameters, creating ambiguity.

Auto traits are special traits that are automatically implemented for all
types that meet certain criteria.",
        "\
Обобщённые параметры были использованы в автоматическом трейте.
Автоматические трейты не могут иметь обобщённые параметры, поскольку они
автоматически реализуются для всех существующих типов. Компилятор не может
определить, какие типы использовать для параметров трейта.",
        "\
자동 트레이트에 제네릭이 사용되었습니다. 자동 트레이트는 모든 기존 타입에
자동으로 구현되기 때문에 제네릭 매개변수를 가질 수 없습니다. 컴파일러는
트레이트의 제네릭 매개변수에 어떤 타입을 사용해야 하는지 추론할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove generic parameters from auto trait",
            "Удалить обобщённые параметры из автоматического трейта",
            "자동 트레이트에서 제네릭 매개변수 제거"
        ),
        code:        "#![feature(auto_traits)]\n\nauto trait Generic {} // no type parameters"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0567.html"
    }]
};
