// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0067: invalid left-hand side in compound assignment

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0067",
    title:       LocalizedText::new(
        "Invalid left-hand side in compound assignment",
        "Недопустимое левое значение в составном присваивании",
        "복합 할당에서 잘못된 좌변"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when the left-hand side of a compound assignment operator
(+=, -=, etc.) is not a valid assignable expression.

Example:
    12 += 1;  // Error: cannot assign to literal",
        "\
Эта ошибка возникает, когда левая часть оператора составного присваивания
(+=, -= и т.д.) не является допустимым присваиваемым выражением.",
        "\
이 오류는 복합 할당 연산자의 좌변이 유효한 할당 가능 표현식이 아닐 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a mutable variable",
            "Использовать изменяемую переменную",
            "가변 변수 사용"
        ),
        code:        "let mut x: i8 = 12;\nx += 1;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0067.html"
    }]
};
