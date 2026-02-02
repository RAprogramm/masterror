// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0589: invalid repr(align) attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0589",
    title:       LocalizedText::new(
        "Invalid `repr(align)`: not a power of two",
        "Недопустимый `repr(align)`: не степень двойки",
        "잘못된 `repr(align)`: 2의 거듭제곱이 아님"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The value of N specified for `repr(align(N))` must be a power of two and
cannot exceed 2^29. If you provide a value that doesn't meet these
requirements, the compiler will raise this error.

Valid alignment values are: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, etc.",
        "\
Значение N, указанное для `repr(align(N))`, должно быть степенью двойки
и не может превышать 2^29. Если вы укажете значение, не соответствующее
этим требованиям, компилятор выдаст эту ошибку.

Допустимые значения выравнивания: 1, 2, 4, 8, 16, 32, 64, 128, 256 и т.д.",
        "\
`repr(align(N))`에 지정된 N 값은 2의 거듭제곱이어야 하며 2^29를 초과할 수
없습니다. 이러한 요구 사항을 충족하지 않는 값을 제공하면 컴파일러가
이 오류를 발생시킵니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a power of two for alignment",
            "Использовать степень двойки для выравнивания",
            "정렬에 2의 거듭제곱 사용"
        ),
        code:        "#[repr(align(16))]  // not align(15)\nenum Foo { Bar(u64) }"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0589.html"
    }]
};
