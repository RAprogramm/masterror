// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0010: cannot allocate in const/static context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0010",
    title:       LocalizedText::new(
        "Cannot allocate in const/static",
        "Нельзя выделять память в const/static",
        "const/static에서 할당 불가"
    ),
    category:    Category::Consts,
    explanation: LocalizedText::new(
        "\
The value of statics and constants must be known at compile time. Creating a
boxed value or using `vec![]` allocates memory on the heap at runtime, which
cannot be done in a const context.

Example:
    const CON: Vec<i32> = vec![1, 2, 3];  // Error: heap allocation",
        "\
Значения static и const должны быть известны во время компиляции.
Создание Box или использование vec![] выделяет память в куче во время
выполнения, что недопустимо в контексте const.",
        "\
static과 const의 값은 컴파일 시점에 알려져야 합니다. 힙 할당은 런타임에 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use an array instead",
                "Использовать массив вместо Vec",
                "대신 배열 사용"
            ),
            code:        "const CON: [i32; 3] = [1, 2, 3];"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use lazy_static or once_cell for runtime initialization",
                "Использовать lazy_static или once_cell",
                "런타임 초기화를 위해 lazy_static 또는 once_cell 사용"
            ),
            code:        "use std::sync::LazyLock;\nstatic CON: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 2, 3]);"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Constant Evaluation",
            url:   "https://doc.rust-lang.org/reference/const_eval.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0010.html"
        }
    ]
};
