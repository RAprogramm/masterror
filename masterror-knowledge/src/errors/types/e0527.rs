// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0527: pattern requires N elements but array has M

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0527",
    title:       LocalizedText::new(
        "Pattern requires different number of elements than array",
        "Образец требует другое количество элементов, чем в массиве",
        "패턴이 배열과 다른 수의 요소를 요구함"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The number of elements in an array or slice pattern differs from the number
of elements in the array being matched. When matching against arrays, the
pattern must account for the correct number of elements.

Use the `..` pattern to capture remaining elements when you don't need to
match all elements explicitly.",
        "\
Количество элементов в образце массива или среза отличается от количества
элементов в сопоставляемом массиве. При сопоставлении с массивами образец
должен учитывать правильное количество элементов.

Используйте `..` для захвата оставшихся элементов.",
        "\
배열 또는 슬라이스 패턴의 요소 수가 매칭되는 배열의 요소 수와 다릅니다.
배열과 매칭할 때 패턴은 올바른 수의 요소를 설명해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use `..` to match remaining elements",
            "Использовать `..` для сопоставления остальных элементов",
            "`..`를 사용하여 나머지 요소 매칭"
        ),
        code:        "match r {\n    &[a, b, ..] => println!(\"a={}, b={}\", a, b),\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0527.html"
    }]
};
