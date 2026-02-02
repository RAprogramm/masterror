// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0533: method used as pattern

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0533",
    title:       LocalizedText::new(
        "Non-pattern item used as match pattern",
        "Не-образцовый элемент использован как образец match",
        "비패턴 항목이 매치 패턴으로 사용됨"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An item which isn't a unit struct, a variant, nor a constant has been used as
a match pattern. In Rust, only unit structs, enum variants, and constants can
be used directly in match patterns.

Methods and other non-pattern items cannot be used as patterns.",
        "\
Элемент, который не является единичной структурой, вариантом или константой,
был использован как образец match. В Rust только единичные структуры,
варианты перечислений и константы могут использоваться напрямую в образцах.",
        "\
유닛 구조체, 변형 또는 상수가 아닌 항목이 매치 패턴으로 사용되었습니다.
Rust에서는 유닛 구조체, 열거형 변형 및 상수만 매치 패턴에서 직접 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a guard clause to compare against method result",
            "Использовать охранное выражение для сравнения с результатом метода",
            "메서드 결과와 비교하기 위해 가드 절 사용"
        ),
        code:        "match 0u32 {\n    x if x == Tortoise.turtle() => {} // bind then compare\n    _ => {}\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0533.html"
    }]
};
