// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0691: zero-sized field with non-trivial alignment in transparent struct

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0691",
    title:       LocalizedText::new(
        "Zero-sized field in transparent struct has non-trivial alignment",
        "Поле нулевого размера в прозрачной структуре имеет нетривиальное выравнивание",
        "투명 구조체의 크기가 0인 필드가 비정상적인 정렬을 가짐"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A struct, enum, or union with `repr(transparent)` contains a zero-sized
field that requires non-trivial alignment (alignment greater than 1).

The `repr(transparent)` representation guarantees that a type is represented
exactly like the single piece of data it wraps. Zero-sized fields with
larger alignment requirements would force the struct to have greater
alignment than its data field, violating the transparency guarantee.",
        "\
Структура, enum или union с `repr(transparent)` содержит поле нулевого
размера, которое требует нетривиального выравнивания (больше 1).

Представление `repr(transparent)` гарантирует, что тип представлен
точно так же, как единственный элемент данных, который он оборачивает.
Поля нулевого размера с большими требованиями к выравниванию заставят
структуру иметь большее выравнивание, чем её поле данных.",
        "\
`repr(transparent)` 구조체, enum 또는 union에 크기가 0이지만
정렬이 1보다 큰 필드가 포함되어 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use PhantomData instead of aligned zero-sized type",
            "Использовать PhantomData вместо выровненного типа нулевого размера",
            "정렬된 크기 0 타입 대신 PhantomData 사용"
        ),
        code:        "use std::marker::PhantomData;\n\n#[repr(transparent)]\nstruct Wrapper(f32, PhantomData<ForceAlign32>);"
    }],
    links:       &[
        DocLink {
            title: "repr(transparent)",
            url:   "https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0691.html"
        }
    ]
};
