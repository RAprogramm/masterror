// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0375: CoerceUnsized with multiple unsized fields

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0375",
    title:       LocalizedText::new(
        "CoerceUnsized with multiple unsized fields",
        "CoerceUnsized с несколькими unsized полями",
        "여러 unsized 필드가 있는 CoerceUnsized"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
CoerceUnsized or DispatchFromDyn was implemented on a struct that contains
more than one field that is being unsized.

CoerceUnsized is designed to coerce structs with a single unsized field.
When multiple fields can be unsized, the compiler cannot generate a valid
implementation because it doesn't know which field(s) to coerce.

These traits are typically used by smart pointers like Box, Rc, and Arc.",
        "\
CoerceUnsized или DispatchFromDyn был реализован на структуре с более чем
одним полем, которое делается unsized.

CoerceUnsized предназначен для приведения структур с одним unsized полем.
Когда несколько полей могут быть unsized, компилятор не может сгенерировать
корректную реализацию.",
        "\
CoerceUnsized 또는 DispatchFromDyn이 둘 이상의 unsized 필드가 있는
구조체에 구현되었습니다.

CoerceUnsized는 단일 unsized 필드가 있는 구조체를 강제 변환하도록 설계되었습니다.
여러 필드가 unsized될 수 있을 때 컴파일러는 어떤 필드를 강제 변환할지 알 수 없어
유효한 구현을 생성할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Ensure struct has only one unsized field",
                "Убедитесь, что структура имеет только одно unsized поле",
                "구조체에 unsized 필드가 하나만 있도록 보장"
            ),
            code:        "struct Foo<T: ?Sized> {\n    a: i32,\n    b: T, // only one unsized field\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust std::ops::CoerceUnsized",
            url:   "https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0375.html"
        }
    ]
};
