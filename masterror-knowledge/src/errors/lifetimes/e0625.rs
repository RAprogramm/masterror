// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0625: const cannot refer to thread-local static

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0625",
    title:       LocalizedText::new(
        "Const cannot refer to thread-local static",
        "Константа не может ссылаться на thread-local static",
        "const는 thread-local static을 참조할 수 없음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A compile-time const variable is referring to a thread-local static variable.

Const variables cannot depend on thread-local statics because const values
must be evaluated at compile time, while thread-local statics are inherently
runtime-dependent.",
        "\
Константа времени компиляции ссылается на thread-local static переменную.

Константы не могут зависеть от thread-local statics, потому что значения
констант должны быть вычислены во время компиляции, тогда как thread-local
statics по своей природе зависят от времени выполнения.",
        "\
컴파일 시간 const 변수가 thread-local static 변수를 참조하고 있습니다.
const 값은 컴파일 시간에 평가되어야 하지만 thread-local statics는
본질적으로 런타임 의존적입니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Extract value as separate const",
            "Извлечь значение как отдельную константу",
            "값을 별도의 const로 추출"
        ),
        code:        "const C: usize = 12;\n\n#[thread_local]\nstatic X: usize = C;\n\nconst Y: usize = 2 * C; // both refer to const C"
    }],
    links:       &[
        DocLink {
            title: "Constants",
            url:   "https://doc.rust-lang.org/reference/items/constant-items.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0625.html"
        }
    ]
};
