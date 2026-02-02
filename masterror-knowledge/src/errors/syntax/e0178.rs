// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0178: the + type operator was used in an ambiguous context

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0178",
    title:       LocalizedText::new(
        "The + type operator was used in an ambiguous context",
        "Оператор типа + использован в неоднозначном контексте",
        "타입 연산자 +가 모호한 컨텍스트에서 사용됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The + type operator was used without proper parentheses in type annotations.
Since the + operator has low precedence in type contexts, it can create
ambiguity about what the type bounds apply to.",
        "\
Оператор типа + использован без надлежащих скобок в аннотациях типов.
Поскольку оператор + имеет низкий приоритет в контексте типов, это может
создать неоднозначность относительно того, к чему применяются ограничения.",
        "\
타입 주석에서 + 타입 연산자가 적절한 괄호 없이 사용되었습니다.
+ 연산자는 타입 컨텍스트에서 우선순위가 낮기 때문에 타입 바운드가
무엇에 적용되는지 모호성을 만들 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Wrap trait bounds in parentheses",
            "Обернуть ограничения трейтов в скобки",
            "트레이트 바운드를 괄호로 감싸기"
        ),
        code:        "trait Foo {}\n\nstruct Bar<'a> {\n    x: &'a (dyn Foo + 'a),     // ok!\n    y: &'a mut (dyn Foo + 'a), // ok!\n    z: fn() -> (dyn Foo + 'a), // ok!\n}"
    }],
    links:       &[
        DocLink {
            title: "RFC 438: Parentheses in Trait Bounds",
            url:   "https://github.com/rust-lang/rfcs/pull/438"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0178.html"
        }
    ]
};
