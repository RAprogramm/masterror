// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0045: variadic parameters on non-C ABI function

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0045",
    title:       LocalizedText::new(
        "Variadic parameters on non-C ABI",
        "Вариативные параметры с не-C ABI",
        "비 C ABI에서 가변 매개변수"
    ),
    category:    Category::Abi,
    explanation: LocalizedText::new(
        "\
Variadic parameters (`...`) can only be used with functions using the C ABI.
Rust only supports variadic parameters for FFI interoperability with C.

Example:
    extern \"Rust\" {
        fn foo(x: u8, ...);  // Error: variadic not allowed
    }",
        "\
Вариативные параметры (`...`) можно использовать только с функциями,
использующими C ABI. Rust поддерживает вариативные параметры только для
взаимодействия с C через FFI.",
        "\
가변 매개변수(`...`)는 C ABI를 사용하는 함수에서만 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use extern \"C\" for variadic functions",
            "Использовать extern \"C\" для вариативных функций",
            "가변 함수에 extern \"C\" 사용"
        ),
        code:        "extern \"C\" {\n    fn foo(x: u8, ...);\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0045.html"
    }]
};
