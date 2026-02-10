// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0512: transmute with differently sized types

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0512",
    title:       LocalizedText::new(
        "Cannot transmute between types of different sizes",
        "Нельзя преобразовать типы разных размеров",
        "크기가 다른 타입 간에 변환할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A transmute with two differently sized types was attempted. The `transmute`
function requires both the source and destination types to be the same size
in bytes, as it performs a bitwise reinterpretation of the data.

This is a fundamental requirement of transmute - you cannot change the size
of data during transmutation.",
        "\
Попытка преобразования между типами разного размера. Функция `transmute`
требует, чтобы исходный и целевой типы имели одинаковый размер в байтах,
так как она выполняет побитовую реинтерпретацию данных.",
        "\
크기가 다른 두 타입 간의 transmute가 시도되었습니다. `transmute` 함수는
소스와 대상 타입이 바이트 단위로 같은 크기여야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use types with the same size",
                "Использовать типы одинакового размера",
                "같은 크기의 타입 사용"
            ),
            code:        "unsafe { takes_u8(std::mem::transmute(0i8)); } // i8 and u8 same size"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use direct type conversion instead",
                "Использовать прямое преобразование типа",
                "대신 직접 타입 변환 사용"
            ),
            code:        "takes_u8(0u8); // direct conversion"
        }
    ],
    links:       &[
        DocLink {
            title: "std::mem::transmute",
            url:   "https://doc.rust-lang.org/std/mem/fn.transmute.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0512.html"
        }
    ]
};
