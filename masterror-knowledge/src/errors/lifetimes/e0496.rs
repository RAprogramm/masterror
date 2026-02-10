// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0496: lifetime name shadowing

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0496",
    title:       LocalizedText::new(
        "Lifetime name shadows another lifetime",
        "Имя времени жизни затеняет другое время жизни",
        "라이프타임 이름이 다른 라이프타임을 가림"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
The same lifetime name was used in nested scopes, causing the inner lifetime
to shadow (hide) the outer lifetime. This is not allowed in Rust because it
creates ambiguity about which lifetime is being referenced.",
        "\
Одно и то же имя времени жизни использовано во вложенных областях
видимости, из-за чего внутреннее время жизни затеняет внешнее.
Это запрещено в Rust, так как создаёт неоднозначность.",
        "\
중첩된 스코프에서 같은 라이프타임 이름이 사용되어 내부 라이프타임이
외부 라이프타임을 가립니다. 이는 어떤 라이프타임이 참조되는지
모호함을 만들기 때문에 Rust에서 허용되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename one of the conflicting lifetimes",
            "Переименовать одно из конфликтующих времён жизни",
            "충돌하는 라이프타임 중 하나의 이름 변경"
        ),
        code:        "struct Foo<'a> {\n    a: &'a i32,\n}\n\nimpl<'a> Foo<'a> {\n    fn f<'b>(x: &'b i32) {} // Use 'b instead of 'a\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0496.html"
    }]
};
