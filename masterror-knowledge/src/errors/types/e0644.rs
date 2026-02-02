// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0644: closure references its own type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0644",
    title:       LocalizedText::new(
        "Closure cannot reference its own type",
        "Замыкание не может ссылаться на свой собственный тип",
        "클로저는 자신의 타입을 참조할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A closure or generator was constructed that references its own type.
This typically happens when a closure accepts a parameter of its own
closure type.

Rust prohibits closures from directly referencing their own type to keep
closure type inference tractable and prevent circular type dependencies.",
        "\
Было создано замыкание или генератор, который ссылается на свой
собственный тип. Это обычно происходит, когда замыкание принимает
параметр своего собственного типа замыкания.

Rust запрещает замыканиям напрямую ссылаться на свой собственный тип,
чтобы упростить вывод типов замыканий и предотвратить циклические
зависимости типов.",
        "\
자신의 타입을 참조하는 클로저나 제너레이터가 생성되었습니다.
Rust는 클로저 타입 추론을 가능하게 하고 순환 타입 종속성을 방지하기 위해
클로저가 자신의 타입을 직접 참조하는 것을 금지합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a top-level function instead",
                "Использовать функцию верхнего уровня вместо этого",
                "대신 최상위 함수 사용"
            ),
            code:        "fn my_fn() { /* ... */ }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use indirect recursion via function pointers",
                "Использовать косвенную рекурсию через указатели на функции",
                "함수 포인터를 통한 간접 재귀 사용"
            ),
            code:        "fn foo(f: &dyn Fn()) { f(); }"
        }
    ],
    links:       &[
        DocLink {
            title: "Closures",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0644.html"
        }
    ]
};
