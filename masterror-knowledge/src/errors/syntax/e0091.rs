// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0091: unused type parameter in type alias

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0091",
    title:       LocalizedText::new(
        "Unused type parameter in type alias",
        "Неиспользуемый параметр типа в псевдониме типа",
        "타입 별칭에서 사용되지 않은 타입 매개변수"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
This error occurs when a type alias declares type parameters that are never
used in the type definition.

Example:
    type Foo<T> = u32;      // Error: T is never used
    type Bar<A, B> = Box<A>; // Error: B is never used",
        "\
Эта ошибка возникает, когда псевдоним типа объявляет параметры типа,
которые никогда не используются в определении типа.",
        "\
이 오류는 타입 별칭이 타입 정의에서 사용되지 않는 타입 매개변수를 선언할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove unused type parameter",
                "Удалить неиспользуемый параметр типа",
                "사용되지 않은 타입 매개변수 제거"
            ),
            code:        "type Foo = u32;\ntype Bar<A> = Box<A>;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use PhantomData if parameter is needed",
                "Использовать PhantomData если параметр нужен",
                "매개변수가 필요한 경우 PhantomData 사용"
            ),
            code:        "use std::marker::PhantomData;\nstruct Foo<T> { data: u32, _marker: PhantomData<T> }"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0091.html"
    }]
};
