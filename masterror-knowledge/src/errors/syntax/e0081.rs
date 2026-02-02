// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0081: duplicate enum discriminant value

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0081",
    title:       LocalizedText::new(
        "Duplicate enum discriminant value",
        "Дублирующееся значение дискриминанта enum",
        "중복된 열거형 판별자 값"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when two or more enum variants have the same discriminant
value, making it impossible to distinguish between them.

Example:
    enum Enum {
        P = 3,
        X = 3,  // Error: duplicate discriminant value
    }

Variants without explicit values are auto-numbered starting from 0.",
        "\
Эта ошибка возникает, когда два или более варианта enum имеют одинаковое
значение дискриминанта, что делает невозможным их различение.",
        "\
이 오류는 두 개 이상의 열거형 변형이 같은 판별자 값을 가질 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use unique discriminant values",
            "Использовать уникальные значения дискриминантов",
            "고유한 판별자 값 사용"
        ),
        code:        "enum Enum {\n    P,\n    X = 3,\n    Y = 5,\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Discriminants",
            url:   "https://doc.rust-lang.org/reference/items/enumerations.html#discriminants"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0081.html"
        }
    ]
};
