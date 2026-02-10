// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0448: unnecessary pub on enum variant

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0448",
    title:       LocalizedText::new(
        "Unnecessary pub visibility on enum variant",
        "Лишняя публичная видимость варианта перечисления",
        "열거형 변형에 불필요한 pub 가시성"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
The pub keyword was used on enum variants inside a public enum, which is
redundant. Since the enum is already public, all of its variants are
automatically public.

Note: This error is no longer emitted by modern compiler versions.",
        "\
Ключевое слово pub использовано для вариантов перечисления внутри
публичного перечисления, что избыточно. Поскольку перечисление уже
публично, все его варианты автоматически публичны.

Примечание: эта ошибка больше не выдаётся современными версиями компилятора.",
        "\
공개 열거형 내의 열거형 변형에 pub 키워드가 사용되었으며, 이는
중복입니다. 열거형이 이미 공개이므로 모든 변형은 자동으로 공개됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove pub from enum variants",
            "Удалить pub из вариантов перечисления",
            "열거형 변형에서 pub 제거"
        ),
        code:        "pub enum Foo {\n    Bar, // Variants inherit enum's visibility\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0448.html"
    }]
};
