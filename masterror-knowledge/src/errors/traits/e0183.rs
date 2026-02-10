// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0183: manual implementation of a Fn* trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0183",
    title:       LocalizedText::new(
        "Manual implementation of a Fn* trait",
        "Ручная реализация трейта Fn*",
        "Fn* 트레이트의 수동 구현"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
You attempted to manually implement Fn, FnMut, or FnOnce traits without
enabling the required unstable features. These trait implementations are
unstable and require the feature flags fn_traits and unboxed_closures.",
        "\
Вы попытались вручную реализовать трейты Fn, FnMut или FnOnce без
включения необходимых нестабильных функций. Эти реализации трейтов
нестабильны и требуют флагов fn_traits и unboxed_closures.",
        "\
필요한 불안정 기능을 활성화하지 않고 Fn, FnMut 또는 FnOnce 트레이트를
수동으로 구현하려고 했습니다. 이러한 트레이트 구현은 불안정하며
fn_traits 및 unboxed_closures 기능 플래그가 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Enable the required features",
            "Включить необходимые функции",
            "필요한 기능 활성화"
        ),
        code:        "#![feature(fn_traits, unboxed_closures)]\n\nstruct MyClosure {\n    foo: i32\n}\n\nimpl FnOnce<()> for MyClosure {\n    type Output = ();\n    extern \"rust-call\" fn call_once(self, args: ()) -> Self::Output {\n        println!(\"{}\", self.foo);\n    }\n}"
    }],
    links:       &[
        DocLink {
            title: "Tracking Issue: fn_traits",
            url:   "https://github.com/rust-lang/rust/issues/29625"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0183.html"
        }
    ]
};
