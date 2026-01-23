// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0508: cannot move out of type [T], a non-copy slice

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0508",
    title:       LocalizedText::new(
        "Cannot move out of type, a non-copy slice",
        "Нельзя переместить из типа — это не-Copy срез",
        "타입에서 이동할 수 없음, 비복사 슬라이스"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
You're trying to move a value out of a slice, but slices don't own their data.
They're just views into an array or Vec.

Moving out would leave a \"hole\" in the slice, which isn't allowed.",
        "\
Вы пытаетесь переместить значение из среза, но срезы не владеют данными.",
        "\
슬라이스에서 값을 이동하려고 하지만, 슬라이스는 데이터를 소유하지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Clone the element",
                "Клонировать элемент",
                "요소 복제"
            ),
            code:        "let elem = slice[i].clone();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use into_iter() on Vec",
                "Использовать into_iter() на Vec",
                "Vec에 into_iter() 사용"
            ),
            code:        "for elem in vec.into_iter() { ... }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0508.html"
    }]
};
