// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0561: non-ident pattern in function pointer type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0561",
    title:       LocalizedText::new(
        "Non-ident pattern in function pointer type parameter",
        "Не-идентификаторный образец в параметре типа указателя на функцию",
        "함수 포인터 타입 매개변수에 비식별자 패턴"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A non-ident or non-wildcard pattern is used as a parameter in a function
pointer type. When defining a type alias for a function pointer, you cannot
use patterns like `mut` or reference patterns (`&`) on parameters.

Only simple identifiers or wildcards (`_`) are allowed in function pointer
type definitions.",
        "\
Не-идентификаторный или не-подстановочный образец используется в качестве
параметра в типе указателя на функцию. При определении псевдонима типа
для указателя на функцию нельзя использовать образцы вроде `mut` или `&`.",
        "\
함수 포인터 타입의 매개변수로 비식별자 또는 비와일드카드 패턴이 사용되었습니다.
함수 포인터에 대한 타입 별칭을 정의할 때 `mut`이나 참조 패턴(`&`)을
매개변수에 사용할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove patterns from parameter",
                "Удалить образцы из параметра",
                "매개변수에서 패턴 제거"
            ),
            code:        "type A1 = fn(param: u8);  // ok\ntype A2 = fn(_: u32);     // wildcard ok"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Omit parameter name entirely",
                "Полностью опустить имя параметра",
                "매개변수 이름 완전히 생략"
            ),
            code:        "type A3 = fn(i16);  // ok"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0561.html"
    }]
};
