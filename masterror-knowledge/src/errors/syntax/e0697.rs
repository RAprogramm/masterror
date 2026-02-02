// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0697: static closure

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0697",
    title:       LocalizedText::new(
        "Closure cannot be used as static",
        "Замыкание нельзя использовать как static",
        "클로저를 static으로 사용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A closure has been used as `static`. Closures cannot be declared as
`static` because of fundamental incompatibilities with how closures
and statics work.

Closures are designed to capture and save their environment. A `static`
item must have a fixed value determined at compile time and stored in
the binary. A static closure would need to save only a static environment,
which defeats the purpose of using a closure.",
        "\
Замыкание было использовано как `static`. Замыкания не могут быть
объявлены как `static` из-за фундаментальной несовместимости с тем,
как работают замыкания и статические элементы.

Замыкания предназначены для захвата и сохранения своего окружения.
Статический элемент должен иметь фиксированное значение, определённое
во время компиляции и хранящееся в бинарном файле.",
        "\
클로저가 `static`으로 사용되었습니다. 클로저와 static의 근본적인
비호환성으로 인해 클로저를 `static`으로 선언할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove the static keyword",
                "Удалить ключевое слово static",
                "static 키워드 제거"
            ),
            code:        "let closure = || {}; // regular closure"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a function instead",
                "Использовать функцию вместо этого",
                "대신 함수 사용"
            ),
            code:        "fn regular_function() {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Closures",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0697.html"
        }
    ]
};
