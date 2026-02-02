// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0277",
    title:       LocalizedText::new(
        "Trait bound not satisfied",
        "Ограничение трейта не выполнено",
        "트레이트 바운드가 충족되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "A generic function or type requires a trait bound that your type doesn't satisfy.",
        "Обобщённая функция или тип требует ограничение трейта, которому ваш тип не удовлетворяет.",
        "제네릭 함수나 타입이 당신의 타입이 충족하지 않는 트레이트 바운드를 요구합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Derive the trait",
                "Получить через derive",
                "트레이트 derive"
            ),
            code:        "#[derive(Hash, Eq, PartialEq)]"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Implement manually",
                "Реализовать вручную",
                "수동 구현"
            ),
            code:        "impl MyTrait for MyType { ... }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0277.html"
    }]
};
