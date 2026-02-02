// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0760: async fn return type with Self referencing parent lifetime (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0760",
    title:       LocalizedText::new(
        "Async fn return type references parent lifetime via Self",
        "Возвращаемый тип async fn ссылается на родительское время жизни через Self",
        "async fn 반환 타입이 Self를 통해 부모 수명 참조"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler.

Previously, an `async fn` or `impl Trait` return type could not contain
a projection or `Self` that references lifetimes from a parent scope.",
        "\
Примечание: Эта ошибка больше не выдаётся компилятором.

Ранее возвращаемый тип `async fn` не мог содержать проекцию или `Self`,
ссылающиеся на времена жизни из родительской области.",
        "\
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Spell out Self explicitly (legacy fix)",
                "Явно укажите Self",
                "Self를 명시적으로 작성"
            ),
            code:        "impl<'a> S<'a> {\n    async fn new(i: &'a i32) -> S<'a> {\n        S(&22)\n    }\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0760.html"
        }
    ]
};
