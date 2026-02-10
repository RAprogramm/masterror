// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0511: invalid monomorphization of intrinsic function

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0511",
    title:       LocalizedText::new(
        "Invalid monomorphization of intrinsic function",
        "Неверная мономорфизация внутренней функции",
        "내장 함수의 잘못된 단형화"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs when an intrinsic function is used with an invalid type
during monomorphization. For SIMD intrinsics like `simd_add`, the generic type
parameter must be a SIMD type, not a scalar type like `i32`.

Intrinsic functions have specific type requirements that must be satisfied.",
        "\
Эта ошибка возникает, когда внутренняя функция используется с неверным
типом при мономорфизации. Для SIMD-функций, таких как `simd_add`,
параметр типа должен быть SIMD-типом, а не скаляром вроде `i32`.",
        "\
이 오류는 단형화 중 내장 함수가 잘못된 타입으로 사용될 때 발생합니다.
`simd_add`와 같은 SIMD 내장 함수의 경우 제네릭 타입 매개변수는
`i32`와 같은 스칼라 타입이 아닌 SIMD 타입이어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a SIMD type instead of scalar",
            "Использовать SIMD-тип вместо скаляра",
            "스칼라 대신 SIMD 타입 사용"
        ),
        code:        "#[repr(simd)]\n#[derive(Copy, Clone)]\nstruct i32x2([i32; 2]);\n\nunsafe { simd_add(i32x2([0, 0]), i32x2([1, 2])); }"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0511.html"
    }]
};
