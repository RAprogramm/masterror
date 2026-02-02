// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0284: ambiguous return type inference

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0284",
    title:       LocalizedText::new(
        "Ambiguous return type cannot be inferred",
        "Неоднозначный возвращаемый тип не может быть выведен",
        "모호한 반환 타입을 추론할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The compiler cannot unambiguously infer the return type of a generic
function or method.

This commonly happens with methods like `into()` for type conversions,
where the return type is not explicitly specified and the compiler
cannot determine it from context due to multiple valid possibilities.",
        "\
Компилятор не может однозначно вывести возвращаемый тип обобщённой
функции или метода.

Это часто происходит с методами вроде `into()` для преобразования типов,
где возвращаемый тип явно не указан и компилятор не может определить
его из контекста.",
        "\
컴파일러가 제네릭 함수나 메서드의 반환 타입을 명확하게 추론할 수 없습니다.
`into()` 같은 타입 변환 메서드에서 자주 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly type the intermediate expression",
                "Явно укажите тип промежуточного выражения",
                "중간 표현식의 타입을 명시적으로 지정"
            ),
            code:        "let n: u32 = 1;\nlet mut d: u64 = 2;\nlet m: u64 = n.into();  // explicitly typed\nd = d + m;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use turbofish syntax on the method",
                "Используйте синтаксис turbofish для метода",
                "메서드에 터보피시 구문 사용"
            ),
            code:        "let n: u32 = 1;\nlet d: u64 = n.into::<u64>();"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Type Annotations",
            url:   "https://doc.rust-lang.org/book/ch03-02-data-types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0284.html"
        }
    ]
};
