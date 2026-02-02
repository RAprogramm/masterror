// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0594: cannot assign to immutable value

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0594",
    title:       LocalizedText::new(
        "Cannot assign to immutable value",
        "Нельзя присвоить неизменяемому значению",
        "불변 값에 할당할 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
A non-mutable value was assigned a value. In Rust, variables are immutable
by default, so you must explicitly use the `mut` keyword to allow modifications.

This error occurs when attempting to modify a variable or field that was not
declared as mutable.",
        "\
Неизменяемому значению было присвоено значение. В Rust переменные
по умолчанию неизменяемы, поэтому необходимо явно использовать
ключевое слово `mut` для разрешения изменений.",
        "\
불변 값에 값이 할당되었습니다. Rust에서 변수는 기본적으로 불변이므로
수정을 허용하려면 `mut` 키워드를 명시적으로 사용해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Declare the variable as mutable",
            "Объявить переменную как изменяемую",
            "변수를 가변으로 선언"
        ),
        code:        "let mut x = SolarSystem { earth: 3 };\nx.earth = 2; // ok!"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0594.html"
    }]
};
