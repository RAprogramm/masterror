// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0689: method called on ambiguous numeric type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0689",
    title:       LocalizedText::new(
        "Method called on ambiguous numeric type",
        "Метод вызван на неоднозначном числовом типе",
        "모호한 숫자 타입에서 메서드 호출"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A method was called on an ambiguous numeric type. The Rust compiler cannot
determine the concrete type of the numeric value, creating ambiguity that
prevents resolving which implementation of the method to use.

This happens with numeric literals or bindings without an identified
concrete type.",
        "\
Метод был вызван на неоднозначном числовом типе. Компилятор Rust не может
определить конкретный тип числового значения, что создаёт неоднозначность
и не позволяет определить, какую реализацию метода использовать.

Это происходит с числовыми литералами или привязками без определённого
конкретного типа.",
        "\
모호한 숫자 타입에서 메서드가 호출되었습니다. Rust 컴파일러가 숫자 값의
구체적인 타입을 결정할 수 없어 어떤 메서드 구현을 사용할지 해결할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use type suffix on literal",
                "Использовать суффикс типа для литерала",
                "리터럴에 타입 접미사 사용"
            ),
            code:        "let _ = 2.0_f32.neg(); // type suffix"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use type annotation on binding",
                "Использовать аннотацию типа для привязки",
                "바인딩에 타입 주석 사용"
            ),
            code:        "let x: f32 = 2.0;\nlet _ = x.neg();"
        }
    ],
    links:       &[
        DocLink {
            title: "Numeric Types",
            url:   "https://doc.rust-lang.org/book/ch03-02-data-types.html#numeric-types"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0689.html"
        }
    ]
};
