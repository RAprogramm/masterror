// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0405: trait not in scope

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0405",
    title:       LocalizedText::new(
        "Trait not in scope",
        "Трейт не в области видимости",
        "트레이트가 스코프에 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An attempt was made to implement a trait that hasn't been imported or
defined in the current scope. The compiler cannot find the trait you're
referencing.

This can happen due to:
- Misspelled trait name
- Missing import statement
- Trait defined in a different module",
        "\
Попытка реализовать трейт, который не импортирован или не определён
в текущей области видимости. Компилятор не может найти трейт.

Это может произойти из-за:
- Опечатки в имени трейта
- Отсутствия оператора use
- Трейт определён в другом модуле",
        "\
현재 스코프에 가져오거나 정의되지 않은 트레이트를 구현하려고
시도했습니다. 컴파일러가 참조하는 트레이트를 찾을 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Import the trait with use statement",
                "Импортировать трейт с помощью use",
                "use 문으로 트레이트 가져오기"
            ),
            code:        "use some_module::SomeTrait;\n\nimpl SomeTrait for Foo { }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Define the trait in current scope",
                "Определить трейт в текущей области видимости",
                "현재 스코프에 트레이트 정의"
            ),
            code:        "trait SomeTrait {\n    // methods\n}\n\nimpl SomeTrait for Foo { }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0405.html"
    }]
};
