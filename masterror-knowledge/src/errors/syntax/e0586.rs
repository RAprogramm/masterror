// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0586: inclusive range with no end

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0586",
    title:       LocalizedText::new(
        "Inclusive range with no end",
        "Включающий диапазон без конца",
        "끝이 없는 포함 범위"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An inclusive range was used with no end. An inclusive range (using `..=`)
requires both a start and an end value. The `..=` syntax is specifically
designed to include the end value in the range.

If you need a range without a specified end, you must use a non-inclusive
range with `..` instead.",
        "\
Был использован включающий диапазон без конца. Включающий диапазон (с `..=`)
требует как начального, так и конечного значения. Синтаксис `..=` специально
предназначен для включения конечного значения в диапазон.

Если вам нужен диапазон без указанного конца, используйте `..`.",
        "\
끝이 없는 포함 범위가 사용되었습니다. 포함 범위(`..=` 사용)는 시작과 끝 값이
모두 필요합니다. `..=` 구문은 특별히 범위에 끝 값을 포함하도록 설계되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use non-inclusive range for open-ended",
                "Использовать невключающий диапазон для открытого конца",
                "열린 끝에 비포함 범위 사용"
            ),
            code:        "let x = &tmp[1..];  // not &tmp[1..=]"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Provide an end value for inclusive range",
                "Указать конечное значение для включающего диапазона",
                "포함 범위에 끝 값 제공"
            ),
            code:        "let x = &tmp[1..=3];  // include index 3"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0586.html"
    }]
};
