// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA015: Avoid O(n²) algorithms

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA015",
    title:        LocalizedText::new(
        "Avoid O(n²) algorithms",
        "Избегайте алгоритмов O(n²)",
        "O(n²) 알고리즘 피하기"
    ),
    category:     PracticeCategory::Performance,
    explanation:  LocalizedText::new(
        "\
Nested loops over the same data often indicate O(n²) complexity.
Use HashSet/HashMap for lookups, or sort + binary search.

What looks fine with 100 items becomes unusable with 10,000.",
        "\
Вложенные циклы по одним данным часто указывают на O(n²) сложность.
Используйте HashSet/HashMap для поиска или сортировку + бинарный поиск.",
        "\
같은 데이터에 대한 중첩 루프는 종종 O(n²) 복잡도를 나타냅니다."
    ),
    good_example: r#"let seen: HashSet<_> = items.iter().collect();
for item in other_items {
    if seen.contains(&item) { // O(1) lookup
        // ...
    }
}"#,
    bad_example:  r#"for item in other_items {
    for existing in &items { // O(n) for each = O(n²) total
        if item == existing {
            // ...
        }
    }
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#9-code-review-methodology"
};
