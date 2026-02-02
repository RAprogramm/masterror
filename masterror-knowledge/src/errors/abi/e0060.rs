// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0060: variadic function called with insufficient arguments

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0060",
    title:       LocalizedText::new(
        "Variadic function called with insufficient arguments",
        "Вариативная функция вызвана с недостаточным количеством аргументов",
        "가변 함수가 불충분한 인수로 호출됨"
    ),
    category:    Category::Abi,
    explanation: LocalizedText::new(
        "\
Variadic external C functions still require their minimum fixed arguments.
You cannot call a variadic function with fewer arguments than the non-variadic
part requires.

Example:
    extern \"C\" {
        fn printf(fmt: *const c_char, ...) -> c_int;
    }
    unsafe { printf(); }  // Error: missing format string argument",
        "\
Вариативные внешние C функции по-прежнему требуют минимальные фиксированные
аргументы. Нельзя вызвать вариативную функцию с меньшим количеством аргументов,
чем требует невариативная часть.",
        "\
가변 외부 C 함수는 여전히 최소한의 고정 인수가 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Provide required arguments",
            "Предоставить обязательные аргументы",
            "필수 인수 제공"
        ),
        code:        "unsafe {\n    printf(c\"test\\n\".as_ptr());\n    printf(c\"%d\\n\".as_ptr(), 42);\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0060.html"
    }]
};
