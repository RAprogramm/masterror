// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0133: call to unsafe function requires unsafe function or block

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0133",
    title:       LocalizedText::new(
        "Call to unsafe function requires unsafe function or block",
        "Вызов unsafe функции требует unsafe функции или блока",
        "unsafe 함수 호출에는 unsafe 함수 또는 블록이 필요"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Unsafe code was used outside of an unsafe block. Unsafe operations in Rust
are potentially dangerous and require explicit safe boundaries.

Examples of unsafe code that require unsafe blocks:
- Dereferencing raw pointers
- Calling functions via FFI
- Calling functions marked unsafe",
        "\
Небезопасный код использован вне блока unsafe. Небезопасные операции в Rust
потенциально опасны и требуют явных границ безопасности.

Примеры небезопасного кода, требующего блока unsafe:
- Разыменование сырых указателей
- Вызов функций через FFI
- Вызов функций, помеченных как unsafe",
        "\
unsafe 블록 외부에서 unsafe 코드가 사용되었습니다. Rust에서 unsafe 작업은
잠재적으로 위험하며 명시적인 안전 경계가 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Wrap unsafe code in an unsafe block",
            "Обернуть небезопасный код в блок unsafe",
            "unsafe 코드를 unsafe 블록으로 감싸기"
        ),
        code:        "unsafe fn f() { }\n\nfn main() {\n    unsafe { f(); } // ok!\n}"
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
