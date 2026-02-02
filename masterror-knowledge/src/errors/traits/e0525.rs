// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0525: closure doesn't implement required Fn trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0525",
    title:       LocalizedText::new(
        "Closure doesn't implement required `Fn` trait",
        "Замыкание не реализует требуемый трейт `Fn`",
        "클로저가 필요한 `Fn` 트레이트를 구현하지 않음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A closure was used but didn't implement the expected trait. This error occurs
when a closure is passed to a function that expects a closure implementing a
specific trait (like `Fn`), but the closure only implements a less capable
trait (like `FnOnce`).

In Rust, there are three closure traits:
- `FnOnce` - can be called once, may consume captured values
- `FnMut` - can be called multiple times, may mutate captured values
- `Fn` - can be called multiple times, borrows captured values immutably

When a closure captures a value that isn't `Copy` or `Clone`, it becomes
`FnOnce` because it must consume the captured value when called.",
        "\
Замыкание было использовано, но не реализует ожидаемый трейт. Эта ошибка
возникает, когда замыкание передаётся функции, ожидающей замыкание с
определённым трейтом (например, `Fn`), но замыкание реализует только менее
способный трейт (например, `FnOnce`).",
        "\
클로저가 사용되었지만 예상되는 트레이트를 구현하지 않았습니다. 이 오류는
특정 트레이트(예: `Fn`)를 구현하는 클로저를 예상하는 함수에 클로저가
전달되었지만, 클로저가 더 제한적인 트레이트(예: `FnOnce`)만 구현할 때
발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Implement Copy and Clone on captured types",
            "Реализовать Copy и Clone для захваченных типов",
            "캡처된 타입에 Copy와 Clone 구현"
        ),
        code:        "#[derive(Clone, Copy)]\nstruct X;\n\nlet closure = |_| foo(x); // now Fn-compatible"
    }],
    links:       &[
        DocLink {
            title: "Closures Chapter",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0525.html"
        }
    ]
};
