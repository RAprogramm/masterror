// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0507: cannot move out of borrowed content

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0507",
    title:       LocalizedText::new(
        "Cannot move out of borrowed content",
        "Нельзя переместить из заимствованного содержимого",
        "빌린 내용에서 이동할 수 없음"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
You're trying to take ownership of a value that you only have a reference to.
References are borrows - they don't own the data.

Moving out of a reference would leave the original owner with invalid data,
violating Rust's memory safety guarantees.

Common cases:
- Indexing into a Vec or array with `vec[i]` and trying to own the element
- Dereferencing a reference and trying to move the value
- Pattern matching on borrowed data with ownership patterns",
        "\
Вы пытаетесь забрать владение значением, на которое у вас только ссылка.
Ссылки - это заимствования, они не владеют данными.

Перемещение из ссылки оставит исходного владельца с недействительными данными.

Частые случаи:
- Индексация Vec с попыткой забрать элемент
- Разыменование ссылки с попыткой переместить
- Pattern matching на заимствованных данных",
        "\
참조만 있는 값의 소유권을 가져오려고 합니다.
참조는 빌림입니다 - 데이터를 소유하지 않습니다.

참조에서 이동하면 원래 소유자가 무효한 데이터를 갖게 됩니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new("Clone the value", "Клонировать значение", "값 복제"),
            code:        "let owned = borrowed.clone();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use mem::take or mem::replace",
                "Использовать mem::take или mem::replace",
                "mem::take 또는 mem::replace 사용"
            ),
            code:        "let owned = std::mem::take(&mut vec[i]);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use swap_remove for Vec",
                "Использовать swap_remove для Vec",
                "Vec에 swap_remove 사용"
            ),
            code:        "let owned = vec.swap_remove(i);"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0507.html"
    }]
};
