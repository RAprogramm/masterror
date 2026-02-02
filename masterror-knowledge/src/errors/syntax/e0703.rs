// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0703: invalid ABI

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0703",
    title:       LocalizedText::new("Invalid ABI", "Недопустимый ABI", "잘못된 ABI"),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
An invalid ABI (Application Binary Interface) was used in the code.

Only predefined ABIs can be used in Rust, such as:
- `Rust` (default)
- `C`
- `system`
- `cdecl`, `stdcall`, `fastcall`, `vectorcall` (Windows)
- `aapcs` (ARM)
- `win64`, `sysv64` (x86_64)",
        "\
В коде использован недопустимый ABI.

В Rust можно использовать только предопределённые ABI:
`Rust`, `C`, `system` и другие.",
        "\
잘못된 ABI가 코드에서 사용되었습니다.

Rust에서는 사전 정의된 ABI만 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a valid ABI",
            "Используйте допустимый ABI",
            "유효한 ABI 사용"
        ),
        code:        "extern \"C\" fn foo() {} // valid ABI"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0703.html"
    }]
};
