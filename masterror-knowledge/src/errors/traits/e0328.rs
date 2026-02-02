// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0328: Unsize trait should not be implemented directly

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0328",
    title:       LocalizedText::new(
        "Unsize trait should not be implemented directly",
        "Трейт Unsize не должен реализовываться напрямую",
        "Unsize 트레이트는 직접 구현하면 안 됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
The Unsize trait is automatically implemented by the compiler and should
not be manually implemented by users. All implementations of Unsize are
provided automatically.

If you're defining a custom smart pointer type and want to enable conversion
from a sized to an unsized type, use CoerceUnsized instead.",
        "\
Трейт Unsize автоматически реализуется компилятором и не должен
реализовываться вручную пользователями.

Если вы определяете собственный умный указатель и хотите включить
преобразование из sized в unsized тип, используйте CoerceUnsized.",
        "\
Unsize 트레이트는 컴파일러가 자동으로 구현하며 사용자가 직접
구현해서는 안 됩니다.

사용자 정의 스마트 포인터 타입을 정의하고 sized에서 unsized 타입으로의
변환을 활성화하려면 대신 CoerceUnsized를 사용하세요."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use CoerceUnsized instead of Unsize",
                "Использовать CoerceUnsized вместо Unsize",
                "Unsize 대신 CoerceUnsized 사용"
            ),
            code:        "#![feature(coerce_unsized)]\nuse std::ops::CoerceUnsized;\n\npub struct MyType<T: ?Sized> {\n    field: T,\n}\n\nimpl<T, U> CoerceUnsized<MyType<U>> for MyType<T>\n    where T: CoerceUnsized<U> {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops::CoerceUnsized",
            url:   "https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0328.html"
        }
    ]
};
