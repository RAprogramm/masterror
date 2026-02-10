// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0518: inline attribute incorrectly placed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0518",
    title:       LocalizedText::new(
        "`#[inline(..)]` attribute incorrectly placed",
        "Атрибут `#[inline(..)]` неправильно размещён",
        "`#[inline(..)]` 속성이 잘못 배치됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An `#[inline(..)]` attribute was incorrectly placed on something other than a
function or method. The `#[inline]` attribute can only be applied to functions
and methods.

It supports the following forms:
- `#[inline]` - hint to inline
- `#[inline(always)]` - force inlining
- `#[inline(never)]` - prevent inlining

Note: This error code is no longer emitted by the compiler.",
        "\
Атрибут `#[inline(..)]` был неправильно размещён на чём-то, кроме функции
или метода. Атрибут `#[inline]` можно применять только к функциям и методам.

Примечание: этот код ошибки больше не выдаётся компилятором.",
        "\
`#[inline(..)]` 속성이 함수나 메서드가 아닌 다른 것에 잘못 배치되었습니다.
`#[inline]` 속성은 함수와 메서드에만 적용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Apply inline to individual methods",
            "Применить inline к отдельным методам",
            "개별 메서드에 inline 적용"
        ),
        code:        "impl Foo {\n    #[inline(always)]\n    fn method1() { }\n    \n    #[inline(never)]\n    fn method2() { }\n}"
    }],
    links:       &[
        DocLink {
            title: "Inline Attribute",
            url:   "https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0518.html"
        }
    ]
};
