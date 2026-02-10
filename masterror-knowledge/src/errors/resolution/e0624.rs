// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0624: private item access

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0624",
    title:       LocalizedText::new(
        "Private item used outside of its scope",
        "Приватный элемент использован вне своей области",
        "비공개 항목이 범위 외부에서 사용됨"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A private item was used outside of its scope. In Rust, items that aren't
explicitly marked as `pub` (public) are private and can only be accessed
within their defining module or scope.

This commonly occurs when trying to call a private method from outside
its defining module.",
        "\
Приватный элемент был использован вне своей области видимости. В Rust
элементы, не помеченные явно как `pub` (публичные), являются приватными
и могут быть доступны только в определяющем их модуле или области.

Это часто происходит при попытке вызвать приватный метод извне
определяющего его модуля.",
        "\
비공개 항목이 범위 외부에서 사용되었습니다. Rust에서 명시적으로 `pub`로
표시되지 않은 항목은 비공개이며 정의하는 모듈이나 범위 내에서만
접근할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Make the item public",
                "Сделать элемент публичным",
                "항목을 공개로 설정"
            ),
            code:        "impl Foo {\n    pub fn method(&self) {} // now public\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use a public wrapper function",
                "Использовать публичную функцию-обёртку",
                "공개 래퍼 함수 사용"
            ),
            code:        "pub fn call_method(foo: &Foo) {\n    foo.method(); // called within scope\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Visibility and Privacy",
            url:   "https://doc.rust-lang.org/reference/visibility-and-privacy.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0624.html"
        }
    ]
};
