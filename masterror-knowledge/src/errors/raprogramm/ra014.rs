// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA014: Pre-allocate with Vec::with_capacity

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA014",
    title:        LocalizedText::new(
        "Pre-allocate with Vec::with_capacity",
        "Предвыделяйте память с Vec::with_capacity",
        "Vec::with_capacity로 사전 할당"
    ),
    category:     PracticeCategory::Performance,
    explanation:  LocalizedText::new(
        "\
When you know the approximate size of a Vec, pre-allocate to avoid reallocations.
Each reallocation copies all existing elements to new memory.

This is especially important in hot paths and loops.",
        "\
Когда знаете примерный размер Vec, предвыделяйте чтобы избежать реаллокаций.
Каждая реаллокация копирует все элементы в новую память.",
        "\
Vec의 대략적인 크기를 알 때, 재할당을 피하기 위해 사전 할당하세요."
    ),
    good_example: r#"let mut results = Vec::with_capacity(items.len());
for item in items {
    results.push(process(item));
}"#,
    bad_example:  r#"let mut results = Vec::new(); // starts with 0 capacity
for item in items {
    results.push(process(item)); // reallocates multiple times!
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#9-code-review-methodology"
};
