// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0569: may_dangle requires unsafe impl

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0569",
    title:       LocalizedText::new(
        "`#[may_dangle]` requires `unsafe impl`",
        "`#[may_dangle]` требует `unsafe impl`",
        "`#[may_dangle]`는 `unsafe impl` 필요"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
If an impl has a generic parameter with the `#[may_dangle]` attribute, then
that impl must be declared as an `unsafe impl`.

The `#[may_dangle]` attribute is used to assert that a destructor will not
access any data of a generic type parameter. Since the compiler does not
verify this assertion, the impl must be explicitly marked as `unsafe` to
indicate that the programmer takes responsibility for the safety.",
        "\
Если реализация имеет обобщённый параметр с атрибутом `#[may_dangle]`, то
эта реализация должна быть объявлена как `unsafe impl`.

Атрибут `#[may_dangle]` утверждает, что деструктор не будет обращаться
к данным обобщённого типа. Поскольку компилятор не проверяет это,
реализация должна быть помечена как `unsafe`.",
        "\
impl에 `#[may_dangle]` 속성이 있는 제네릭 매개변수가 있으면 해당 impl은
`unsafe impl`로 선언되어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Mark the impl as unsafe",
            "Пометить impl как unsafe",
            "impl을 unsafe로 표시"
        ),
        code:        "#![feature(dropck_eyepatch)]\n\nstruct Foo<X>(X);\nunsafe impl<#[may_dangle] X> Drop for Foo<X> {\n    fn drop(&mut self) { }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0569.html"
    }]
};
