// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0034: ambiguous method call

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0034",
    title:       LocalizedText::new(
        "Ambiguous method call",
        "Неоднозначный вызов метода",
        "모호한 메서드 호출"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
This error occurs when multiple traits define methods with the same name,
and you call the method without specifying which trait's implementation to use.

Example:
    trait Foo { fn method(&self); }
    trait Bar { fn method(&self); }
    impl Foo for MyType { ... }
    impl Bar for MyType { ... }

    my_value.method();  // Error: ambiguous - Foo::method or Bar::method?",
        "\
Эта ошибка возникает, когда несколько трейтов определяют методы с одинаковым
именем, и вы вызываете метод без указания, какую реализацию использовать.",
        "\
이 오류는 여러 트레이트가 같은 이름의 메서드를 정의하고, 어떤 구현을 사용할지
지정하지 않고 메서드를 호출할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use fully qualified syntax",
            "Использовать полный синтаксис",
            "완전한 한정 구문 사용"
        ),
        code:        "<MyType as Foo>::method(&my_value);\n// or\nFoo::method(&my_value);"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Fully Qualified Syntax",
            url:   "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0034.html"
        }
    ]
};
