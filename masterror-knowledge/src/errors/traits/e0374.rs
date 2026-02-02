// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0374: CoerceUnsized on struct without unsized fields

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0374",
    title:       LocalizedText::new(
        "CoerceUnsized on struct without unsized fields",
        "CoerceUnsized на структуре без unsized полей",
        "unsized 필드가 없는 구조체에 CoerceUnsized"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
CoerceUnsized or DispatchFromDyn was implemented on a struct that doesn't
contain any fields that are being unsized.

CoerceUnsized is used to coerce structs with unsizable fields (e.g.,
MyBox<T> to MyBox<dyn Trait>). If a struct has no unsized fields, there
is no meaningful coercion to perform, making the trait implementation invalid.

These traits are primarily used by smart pointers like Box, Rc, and Arc.",
        "\
CoerceUnsized или DispatchFromDyn был реализован на структуре, которая не
содержит полей, которые делаются unsized.

CoerceUnsized используется для приведения структур с unsized полями.
Если структура не имеет unsized полей, нет осмысленного приведения,
делая реализацию трейта недействительной.",
        "\
CoerceUnsized 또는 DispatchFromDyn이 unsized 필드가 없는 구조체에 구현되었습니다.

CoerceUnsized는 unsized 필드가 있는 구조체를 강제 변환하는 데 사용됩니다
(예: MyBox<T>를 MyBox<dyn Trait>로). 구조체에 unsized 필드가 없으면
수행할 의미있는 강제 변환이 없어 트레이트 구현이 유효하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add an unsized field to the struct",
                "Добавить unsized поле в структуру",
                "구조체에 unsized 필드 추가"
            ),
            code:        "#![feature(coerce_unsized)]\nuse std::ops::CoerceUnsized;\n\nstruct Foo<T: ?Sized> {\n    a: i32,\n    b: T, // unsized field\n}\n\nimpl<T, U> CoerceUnsized<Foo<U>> for Foo<T>\n    where T: CoerceUnsized<U> {}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops::CoerceUnsized",
            url:   "https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0374.html"
        }
    ]
};
