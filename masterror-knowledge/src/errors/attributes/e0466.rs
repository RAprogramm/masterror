// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0466: malformed macro_use declaration

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0466",
    title:       LocalizedText::new(
        "Malformed macro import declaration",
        "Неправильное объявление импорта макросов",
        "잘못된 형식의 매크로 임포트 선언"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
The syntax used in a #[macro_use] attribute declaration is incorrect.
The attribute expects a comma-separated list of macro names inside
parentheses, not function calls or key-value assignments.

Note: This error is no longer emitted by modern compilers.",
        "\
Синтаксис в объявлении атрибута #[macro_use] неверен. Атрибут ожидает
список имён макросов через запятую в скобках, а не вызовы функций
или присваивания ключ-значение.

Примечание: эта ошибка больше не выдаётся современными компиляторами.",
        "\
#[macro_use] 속성 선언에 사용된 구문이 올바르지 않습니다. 속성은
괄호 안에 쉼표로 구분된 매크로 이름 목록을 기대하며, 함수 호출이나
키-값 할당은 허용되지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use comma-separated macro names",
                "Использовать имена макросов через запятую",
                "쉼표로 구분된 매크로 이름 사용"
            ),
            code:        "#[macro_use(macro1, macro2)]\nextern crate some_crate;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Import all macros from crate",
                "Импортировать все макросы из крейта",
                "크레이트에서 모든 매크로 임포트"
            ),
            code:        "#[macro_use]\nextern crate some_crate;"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0466.html"
    }]
};
