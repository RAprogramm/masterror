// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0170: pattern binding uses same name as one of the variants

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0170",
    title:       LocalizedText::new(
        "Pattern binding uses same name as one of the variants",
        "Привязка паттерна использует то же имя, что и вариант",
        "패턴 바인딩이 변형 이름과 동일한 이름 사용"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A pattern binding is using the same name as one of the variants of a type.
Rust interprets unqualified variant names as new variable bindings rather
than references to the enum variants, which is likely not the intended behavior.",
        "\
Привязка паттерна использует то же имя, что и один из вариантов типа.
Rust интерпретирует неквалифицированные имена вариантов как новые
привязки переменных, а не как ссылки на варианты перечисления.",
        "\
패턴 바인딩이 타입의 변형 중 하나와 동일한 이름을 사용하고 있습니다.
Rust는 정규화되지 않은 변형 이름을 열거형 변형에 대한 참조가 아닌
새 변수 바인딩으로 해석합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Qualify the variant names",
                "Квалифицировать имена вариантов",
                "변형 이름 정규화"
            ),
            code:        "enum Method { GET, POST }\n\nmatch m {\n    Method::GET => {},  // properly qualified\n    Method::POST => {},\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Import variants into scope",
                "Импортировать варианты в область видимости",
                "스코프로 변형 가져오기"
            ),
            code:        "use Method::*;\nenum Method { GET, POST }\n\nmatch m {\n    GET => {},  // now unqualified names work\n    POST => {},\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Enum Patterns",
            url:   "https://doc.rust-lang.org/book/ch06-02-match.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0170.html"
        }
    ]
};
