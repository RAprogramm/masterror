// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0482: lifetime of returned value doesn't outlive function call

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0482",
    title:       LocalizedText::new(
        "Lifetime of returned value doesn't outlive function call",
        "Время жизни возвращаемого значения не переживает вызов функции",
        "반환된 값의 라이프타임이 함수 호출보다 오래 살지 않음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
This error occurs when using impl Trait in return types with lifetimes that
don't properly align. The issue arises because impl Trait implicitly applies
a 'static lifetime restriction, but the actual data may only live for a
shorter lifetime.

Note: This error is no longer emitted by modern compilers.",
        "\
Эта ошибка возникает при использовании impl Trait в возвращаемых типах
с неправильно согласованными временами жизни. Проблема в том, что
impl Trait неявно применяет ограничение 'static, но данные могут жить
только более короткое время.

Примечание: эта ошибка больше не выдаётся современными компиляторами.",
        "\
이 오류는 라이프타임이 제대로 정렬되지 않는 반환 타입에서 impl Trait를
사용할 때 발생합니다. impl Trait가 암묵적으로 'static 라이프타임
제한을 적용하기 때문입니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add lifetime bounds to impl Trait",
                "Добавить ограничения времени жизни к impl Trait",
                "impl Trait에 라이프타임 바운드 추가"
            ),
            code:        "fn prefix<'a>(\n    words: impl Iterator<Item = &'a str> + 'a\n) -> impl Iterator<Item = String> + 'a {\n    words.map(|v| format!(\"foo-{}\", v))\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use 'static lifetime",
                "Использовать время жизни 'static",
                "'static 라이프타임 사용"
            ),
            code:        "fn prefix(\n    words: impl Iterator<Item = &'static str>\n) -> impl Iterator<Item = String> {\n    words.map(|v| format!(\"foo-{}\", v))\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0482.html"
    }]
};
