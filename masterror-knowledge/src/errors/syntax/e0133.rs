// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0133: unsafe code outside unsafe block

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0133",
    title:       LocalizedText::new(
        "Unsafe code outside unsafe block",
        "Небезопасный код вне unsafe блока",
        "unsafe 블록 외부의 unsafe 코드"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
Unsafe operations must be wrapped in an `unsafe` block. This includes:
- Dereferencing raw pointers
- Calling unsafe functions
- Calling functions via FFI

Example:
    unsafe fn f() {}
    fn main() {
        f();  // Error: unsafe function call outside unsafe block
    }",
        "\
Небезопасные операции должны быть обёрнуты в блок `unsafe`. Это включает:
- Разыменование сырых указателей
- Вызов unsafe функций
- Вызов функций через FFI",
        "\
안전하지 않은 작업은 `unsafe` 블록으로 감싸야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Wrap in unsafe block",
            "Обернуть в unsafe блок",
            "unsafe 블록으로 감싸기"
        ),
        code:        "unsafe fn f() {}\n\nfn main() {\n    unsafe { f(); }\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Unsafe Rust",
            url:   "https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0133.html"
        }
    ]
};
