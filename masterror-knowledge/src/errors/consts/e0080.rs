// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0080: constant value evaluation failed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0080",
    title:       LocalizedText::new(
        "Constant evaluation failed",
        "Ошибка вычисления константы",
        "상수 평가 실패"
    ),
    category:    Category::Consts,
    explanation: LocalizedText::new(
        "\
This error occurs when the compiler cannot evaluate a constant expression.
This typically happens with operations that are mathematically or
computationally invalid:

- Division by zero
- Integer overflow
- Invalid bit shifts

Example:
    enum E { X = (1 / 0) }  // Error: division by zero",
        "\
Эта ошибка возникает, когда компилятор не может вычислить константное
выражение. Обычно это происходит при недопустимых операциях:
деление на ноль, целочисленное переполнение.",
        "\
이 오류는 컴파일러가 상수 표현식을 평가할 수 없을 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Ensure valid arithmetic operations",
            "Убедиться в корректности арифметических операций",
            "유효한 산술 연산 확인"
        ),
        code:        "enum E {\n    X = 1,\n    Y = 2,\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Constant Evaluation",
            url:   "https://doc.rust-lang.org/reference/const_eval.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0080.html"
        }
    ]
};
