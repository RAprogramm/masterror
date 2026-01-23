// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0700: hidden type captures lifetime

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0700",
    title:       LocalizedText::new(
        "Hidden type captures lifetime",
        "Скрытый тип захватывает время жизни",
        "숨겨진 타입이 라이프타임을 캡처함"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
When using `impl Trait` return type, the hidden concrete type captures
a lifetime that isn't declared in the function signature.",
        "\
При использовании типа возврата `impl Trait` скрытый конкретный тип
захватывает время жизни, не объявленное в сигнатуре.",
        "\
`impl Trait` 반환 타입을 사용할 때, 숨겨진 구체적 타입이 선언되지 않은 라이프타임을 캡처합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Declare the captured lifetime",
            "Объявить захваченное время жизни",
            "캡처된 라이프타임 선언"
        ),
        code:        "fn foo<'a>(x: &'a str) -> impl Iterator<Item = char> + 'a"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0700.html"
    }]
};
