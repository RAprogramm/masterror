// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0378: DispatchFromDyn trait implemented on invalid type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0378",
    title:       LocalizedText::new(
        "DispatchFromDyn on invalid type",
        "DispatchFromDyn на недопустимом типе",
        "잘못된 타입에 DispatchFromDyn"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
The DispatchFromDyn trait was implemented on something which is not a pointer
or a newtype wrapper around a pointer.

DispatchFromDyn can only be implemented for:
- Built-in pointer types
- Structs that are newtype wrappers around pointers (single field, except
  PhantomData, where that field implements DispatchFromDyn)",
        "\
Трейт DispatchFromDyn был реализован на чём-то, что не является указателем
или newtype обёрткой вокруг указателя.

DispatchFromDyn может быть реализован только для:
- Встроенных типов указателей
- Структур-обёрток вокруг указателей (одно поле, кроме PhantomData)",
        "\
DispatchFromDyn 트레이트가 포인터나 포인터를 감싸는 newtype 래퍼가 아닌
타입에 구현되었습니다.

DispatchFromDyn은 다음에만 구현될 수 있습니다:
- 내장 포인터 타입
- 포인터를 감싸는 newtype 래퍼 구조체(PhantomData를 제외한 단일 필드)"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement on pointer wrapper with single field",
                "Реализовать на обёртке указателя с одним полем",
                "단일 필드가 있는 포인터 래퍼에 구현"
            ),
            code:        "#![feature(dispatch_from_dyn, unsize)]\nuse std::{marker::Unsize, ops::DispatchFromDyn};\n\nstruct Ptr<T: ?Sized>(*const T);\n\nimpl<T: ?Sized, U: ?Sized> DispatchFromDyn<Ptr<U>> for Ptr<T>\nwhere T: Unsize<U> {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops::DispatchFromDyn",
            url:   "https://doc.rust-lang.org/std/ops/trait.DispatchFromDyn.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0378.html"
        }
    ]
};
