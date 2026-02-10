// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0211: type mismatch in function/type usage

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0211",
    title:       LocalizedText::new(
        "Type mismatch in function or type usage",
        "Несоответствие типов в использовании функции или типа",
        "함수 또는 타입 사용에서 타입 불일치"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A function or type doesn't fit the requirements for where it was used.
This is a type mismatch error with several common scenarios.

Note: This error code is no longer emitted by the compiler.

Common cases:
- Intrinsic function with wrong signature
- Main function with wrong return type
- Range pattern type mismatch in match
- Invalid self type in methods",
        "\
Функция или тип не соответствует требованиям для места использования.
Это ошибка несоответствия типов.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
함수나 타입이 사용된 위치의 요구사항과 맞지 않습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Ensure types match the expected signature",
                "Убедитесь, что типы соответствуют ожидаемой сигнатуре",
                "타입이 예상 시그니처와 일치하는지 확인"
            ),
            code:        "fn main() {}  // correct main signature"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use matching types in range patterns",
                "Используйте совпадающие типы в диапазонных паттернах",
                "범위 패턴에서 일치하는 타입 사용"
            ),
            code:        "let x = 1u8;\nmatch x {\n    0u8..=3u8 => (),\n    _ => ()\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Type System",
            url:   "https://doc.rust-lang.org/reference/types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0211.html"
        }
    ]
};
