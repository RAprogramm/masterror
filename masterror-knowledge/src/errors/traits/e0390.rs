// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0390: cannot define inherent impl for primitive types

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0390",
    title:       LocalizedText::new(
        "Cannot define inherent impl for primitive types",
        "Нельзя определить inherent impl для примитивных типов",
        "기본 타입에 대한 고유 impl을 정의할 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A method or constant was implemented on a primitive type using an inherent
implementation (direct impl block). This is not allowed in Rust.

You cannot create inherent implementations directly on primitive types like
raw pointers (*mut Foo). Rust restricts this to prevent conflicts and
maintain type system coherence.

Use a trait implementation instead, or for references, move the reference
into the method signature.",
        "\
Метод или константа была реализована на примитивном типе с использованием
inherent реализации (прямой impl блок). Это не разрешено в Rust.

Нельзя создавать inherent реализации на примитивных типах как сырые указатели.
Используйте реализацию трейта вместо этого.",
        "\
메서드나 상수가 고유 구현(직접 impl 블록)을 사용하여 기본 타입에 구현되었습니다.
이는 Rust에서 허용되지 않습니다.

원시 포인터(*mut Foo)와 같은 기본 타입에 직접 고유 구현을 만들 수 없습니다.
대신 트레이트 구현을 사용하세요."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a trait implementation instead",
                "Использовать реализацию трейта вместо этого",
                "대신 트레이트 구현 사용"
            ),
            code:        "struct Foo { x: i32 }\n\ntrait Bar {\n    fn bar();\n}\n\nimpl Bar for *mut Foo {\n    fn bar() {} // ok!\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Move reference into method signature",
                "Переместить ссылку в сигнатуру метода",
                "참조를 메서드 시그니처로 이동"
            ),
            code:        "struct Foo;\n\nimpl Foo {\n    fn bar(&self, other: &Self) {} // not impl &Foo\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Implementations",
            url:   "https://doc.rust-lang.org/reference/items/implementations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0390.html"
        }
    ]
};
