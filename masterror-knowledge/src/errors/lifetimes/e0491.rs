// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0491: reference has longer lifetime than data it references

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0491",
    title:       LocalizedText::new(
        "Reference has longer lifetime than data it references",
        "Ссылка имеет большее время жизни чем данные",
        "참조가 참조하는 데이터보다 긴 라이프타임을 가짐"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The compiler cannot guarantee that a lifetime parameter will outlive another
lifetime parameter in a reference type. When you have a reference with
lifetime 'a to data with lifetime 'b, 'b must outlive 'a to ensure the
reference is always valid.",
        "\
Компилятор не может гарантировать, что один параметр времени жизни
переживёт другой в ссылочном типе. Когда есть ссылка с временем жизни 'a
на данные с временем жизни 'b, 'b должно пережить 'a, чтобы ссылка
всегда была действительна.",
        "\
컴파일러가 참조 타입에서 한 라이프타임 매개변수가 다른 라이프타임
매개변수보다 오래 살 것을 보장할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add lifetime bound to enforce that 'b outlives 'a",
            "Добавить ограничение, чтобы 'b пережило 'a",
            "'b가 'a보다 오래 살도록 라이프타임 바운드 추가"
        ),
        code:        "impl<'a, 'b: 'a> Trait<'a, 'b> for usize {\n    type Out = &'a Foo<'b>; // works!\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0491.html"
    }]
};
