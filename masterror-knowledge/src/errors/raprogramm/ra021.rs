// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA021: Use String::with_capacity in loops

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA021",
    title:        LocalizedText::new(
        "Use String::with_capacity in loops",
        "Используйте String::with_capacity в циклах",
        "루프에서 String::with_capacity 사용"
    ),
    category:     PracticeCategory::Performance,
    explanation:  LocalizedText::new(
        "\
Building strings in loops with `push_str` or `+` causes repeated reallocations.
Each reallocation copies all existing data to a new buffer.

Pre-allocate with `String::with_capacity` when the final size is known or
can be estimated. For complex cases, consider using `std::fmt::Write` trait
or collecting into a `Vec<&str>` and joining at the end.",
        "\
Построение строк в циклах через `push_str` или `+` вызывает повторные
реаллокации. Каждая реаллокация копирует все данные в новый буфер.

Используйте `String::with_capacity` когда финальный размер известен или
может быть оценён. Для сложных случаев используйте `std::fmt::Write`
или собирайте в `Vec<&str>` и объединяйте в конце.",
        "\
`push_str` 또는 `+`로 루프에서 문자열을 구축하면 반복적인
재할당이 발생합니다. 각 재할당은 모든 기존 데이터를 새 버퍼로 복사합니다."
    ),
    good_example: r#"fn build_csv(rows: &[Row]) -> String {
    // Estimate: ~50 chars per row
    let mut result = String::with_capacity(rows.len() * 50);

    for row in rows {
        result.push_str(&row.to_csv_line());
        result.push('\n');
    }
    result
}

// Alternative: collect and join
let lines: Vec<_> = rows.iter().map(|r| r.to_csv_line()).collect();
let result = lines.join("\n");"#,
    bad_example:  r#"fn build_csv(rows: &[Row]) -> String {
    let mut result = String::new();

    for row in rows {
        result = result + &row.to_csv_line() + "\n"; // Reallocates every iteration!
    }
    result
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#performance"
};
