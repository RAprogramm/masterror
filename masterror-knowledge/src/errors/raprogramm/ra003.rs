// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA003: Avoid unnecessary clone() calls

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA003",
    title:        LocalizedText::new(
        "Avoid unnecessary clone() calls",
        "Избегайте ненужных вызовов clone()",
        "불필요한 clone() 호출 피하기"
    ),
    category:     PracticeCategory::Performance,
    explanation:  LocalizedText::new(
        "\
Cloning allocates memory and copies data. Often you can use references instead.
Only clone when you actually need ownership of the data.

Common anti-patterns:
- Cloning just to satisfy the borrow checker (restructure instead)
- Cloning in a loop (clone once before the loop)
- Cloning when a reference would work",
        "\
Клонирование выделяет память и копирует данные. Часто можно использовать ссылки.
Клонируйте только когда действительно нужно владение данными.",
        "\
클론은 메모리를 할당하고 데이터를 복사합니다. 종종 참조를 대신 사용할 수 있습니다."
    ),
    good_example: r#"fn process(data: &str) { /* use reference */ }
let owned = expensive_data.clone(); // clone once
for item in &items { process(item); }"#,
    bad_example:  r#"for item in items {
    process(item.clone()); // clones every iteration!
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#3-code-quality"
};
