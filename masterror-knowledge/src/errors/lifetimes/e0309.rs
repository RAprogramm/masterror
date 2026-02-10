// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0309: parameter type is missing an explicit lifetime bound

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0309",
    title:       LocalizedText::new(
        "Parameter type is missing an explicit lifetime bound",
        "Параметр типа не имеет явного ограничения времени жизни",
        "매개변수 타입에 명시적 라이프타임 바운드가 없음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A type parameter lacks an explicit lifetime bound required by an associated
type constraint. The compiler cannot verify that all data in the type T is
valid for the required lifetime 'a.

This commonly happens when:
- A struct field uses an associated type like <T as SomeTrait<'a>>::Output
- The trait implementation requires a lifetime bound (e.g., T: 'a)
- The struct definition doesn't declare this same bound in its where-clause",
        "\
Параметр типа не имеет явного ограничения времени жизни, требуемого
ассоциированным типом. Компилятор не может проверить, что все данные
в типе T действительны для требуемого времени жизни 'a.

Это часто происходит когда:
- Поле структуры использует ассоциированный тип
- Реализация трейта требует ограничения времени жизни
- Определение структуры не объявляет это ограничение",
        "\
타입 매개변수에 연관 타입 제약에 필요한 명시적 라이프타임 바운드가 없습니다.
컴파일러는 타입 T의 모든 데이터가 필요한 라이프타임 'a에 유효한지 확인할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add lifetime bound to type parameter",
                "Добавить ограничение времени жизни к параметру типа",
                "타입 매개변수에 라이프타임 바운드 추가"
            ),
            code:        "struct Foo<'a, T>\nwhere\n    T: 'a,\n{\n    foo: <T as SomeTrait<'a>>::Output\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Lifetime Bounds",
            url:   "https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0309.html"
        }
    ]
};
