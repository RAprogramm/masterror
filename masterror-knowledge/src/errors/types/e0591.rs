// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0591: transmuting function items vs function pointers

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0591",
    title:       LocalizedText::new(
        "Cannot transmute between function items and pointers",
        "Нельзя преобразовать между элементами функций и указателями",
        "함수 항목과 포인터 간에 변환할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Per RFC 401, function items have a unique, zero-sized marker type that is
distinct from function pointers. While the language typically coerces function
items to function pointers automatically, this distinction becomes problematic
when using `transmute`.

The key issue is:
- Function items (e.g., `typeof(foo)`) are zero-sized types
- Function pointers (e.g., `fn(S)`) are not zero-sized

Transmuting directly between these types is incorrect because their sizes
don't match.",
        "\
Согласно RFC 401, элементы функций имеют уникальный тип-маркер нулевого
размера, отличный от указателей на функции. Хотя язык обычно автоматически
приводит элементы функций к указателям, это различие становится проблемой
при использовании `transmute`.",
        "\
RFC 401에 따르면 함수 항목은 함수 포인터와 구별되는 고유한 크기가 0인
마커 타입을 갖습니다. 언어는 일반적으로 함수 항목을 함수 포인터로
자동 변환하지만, `transmute` 사용 시 이 구분이 문제가 됩니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Cast to function pointer before transmute",
                "Привести к указателю на функцию перед transmute",
                "transmute 전에 함수 포인터로 캐스트"
            ),
            code:        "let f: extern \"C\" fn(*mut i32) = transmute(foo as extern \"C\" fn(_));"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Cast to usize before transmute",
                "Привести к usize перед transmute",
                "transmute 전에 usize로 캐스트"
            ),
            code:        "let f: extern \"C\" fn(*mut i32) = transmute(foo as usize);"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 401: Coercions",
            url:   "https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0591.html"
        }
    ]
};
