// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0322: built-in trait cannot be explicitly implemented

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0322",
    title:       LocalizedText::new(
        "Built-in trait cannot be explicitly implemented",
        "Встроенный трейт нельзя реализовать явно",
        "내장 트레이트는 명시적으로 구현할 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
All implementations of built-in traits are provided automatically by the
compiler. Built-in traits cannot be explicitly implemented by user code.

The Sized trait is a special trait built into the compiler for types with
a constant size known at compile-time. This trait is automatically implemented
for types as needed by the compiler.",
        "\
Все реализации встроенных трейтов предоставляются автоматически компилятором.
Встроенные трейты нельзя явно реализовать в пользовательском коде.

Трейт Sized - специальный встроенный трейт для типов с известным на этапе
компиляции постоянным размером. Он автоматически реализуется компилятором.",
        "\
내장 트레이트의 모든 구현은 컴파일러가 자동으로 제공합니다.
내장 트레이트는 사용자 코드에서 명시적으로 구현할 수 없습니다.

Sized 트레이트는 컴파일 시점에 상수 크기가 알려진 타입을 위한
컴파일러에 내장된 특별한 트레이트입니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the explicit implementation",
                "Удалить явную реализацию",
                "명시적 구현 제거"
            ),
            code:        "struct Foo;\n// impl Sized for Foo {} // Remove this - compiler handles it"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Sized Trait",
            url:   "https://doc.rust-lang.org/std/marker/trait.Sized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0322.html"
        }
    ]
};
