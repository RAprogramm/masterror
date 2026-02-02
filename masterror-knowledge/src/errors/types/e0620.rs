// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0620: cast to unsized type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0620",
    title:       LocalizedText::new(
        "Cast to unsized type not allowed",
        "Приведение к нетипизированному размеру не допускается",
        "크기가 정해지지 않은 타입으로 캐스팅 불가"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to cast a value directly to an unsized type. Unsized types are
types whose size cannot be determined at compile-time.

Since the size cannot be computed, unsized types cannot exist as standalone
values - they can only be accessed through pointers or references.",
        "\
Попытка привести значение напрямую к типу с неизвестным размером.
Типы с неизвестным размером - это типы, размер которых не может быть
определён во время компиляции.

Поскольку размер не может быть вычислен, такие типы не могут существовать
как самостоятельные значения - к ним можно обращаться только через
указатели или ссылки.",
        "\
크기가 정해지지 않은 타입으로 직접 캐스팅하려고 시도했습니다.
이러한 타입은 포인터나 참조를 통해서만 접근할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Cast to a reference of the unsized type",
            "Привести к ссылке на тип с неизвестным размером",
            "크기가 정해지지 않은 타입의 참조로 캐스팅"
        ),
        code:        "let x = &[1_usize, 2] as &[usize]; // cast to reference"
    }],
    links:       &[
        DocLink {
            title: "Dynamically Sized Types",
            url:   "https://doc.rust-lang.org/reference/dynamically-sized-types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0620.html"
        }
    ]
};
