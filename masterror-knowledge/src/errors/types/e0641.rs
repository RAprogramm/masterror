// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0641: pointer with unknown kind

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0641",
    title:       LocalizedText::new(
        "Cannot cast to/from pointer with unknown kind",
        "Невозможно привести к/от указателя с неизвестным типом",
        "알 수 없는 종류의 포인터로/부터 캐스팅 불가"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to cast to or from a pointer type, but the compiler cannot infer
the complete type information needed for the pointer.

The error is triggered when casting to a pointer type using the wildcard `_`
without providing enough context for the compiler to determine what the
pointed-to type should be.",
        "\
Попытка привести к или от типа указателя, но компилятор не может вывести
полную информацию о типе, необходимую для указателя.

Ошибка возникает при приведении к типу указателя с использованием `_`
без предоставления достаточного контекста для определения целевого типа.",
        "\
포인터 타입으로/부터 캐스팅하려고 시도했지만 컴파일러가 포인터에 필요한
완전한 타입 정보를 추론할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly specify the pointed-to type",
                "Явно указать тип, на который указывает указатель",
                "가리키는 타입을 명시적으로 지정"
            ),
            code:        "let b = 0 as *const i32; // explicit type"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use type annotation on the variable",
                "Использовать аннотацию типа для переменной",
                "변수에 타입 주석 사용"
            ),
            code:        "let c: *const i32 = 0 as *const _; // type from annotation"
        }
    ],
    links:       &[
        DocLink {
            title: "Raw Pointers",
            url:   "https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0641.html"
        }
    ]
};
