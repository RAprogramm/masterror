// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0528: pattern requires at least N elements but array has M

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0528",
    title:       LocalizedText::new(
        "Pattern requires at least N elements but array has fewer",
        "Образец требует минимум N элементов, но массив содержит меньше",
        "패턴이 최소 N개의 요소를 요구하지만 배열이 더 적음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An array or slice pattern requires more elements than are present in the
matched array. Even with the `..` operator for remaining elements, the
pattern specifies a minimum number of required elements.

The matched array must have at least as many elements as explicitly required
by the pattern.",
        "\
Образец массива или среза требует больше элементов, чем присутствует
в сопоставляемом массиве. Даже с оператором `..` для оставшихся элементов
образец указывает минимальное количество требуемых элементов.",
        "\
배열 또는 슬라이스 패턴이 매칭된 배열에 있는 것보다 더 많은 요소를 요구합니다.
나머지 요소를 위한 `..` 연산자를 사용해도 패턴은 최소 요소 수를 지정합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Ensure array has enough elements",
            "Убедиться, что массив содержит достаточно элементов",
            "배열에 충분한 요소가 있는지 확인"
        ),
        code:        "let r = &[1, 2, 3, 4, 5];\nmatch r {\n    &[a, b, c, rest @ ..] => { /* ok */ }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0528.html"
    }]
};
