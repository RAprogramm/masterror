// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0232: invalid rustc_on_unimplemented attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0232",
    title:       LocalizedText::new(
        "Invalid #[rustc_on_unimplemented] attribute",
        "Недопустимый атрибут #[rustc_on_unimplemented]",
        "잘못된 #[rustc_on_unimplemented] 속성"
    ),
    category:    Category::Syntax,
    explanation: LocalizedText::new(
        "\
The `#[rustc_on_unimplemented]` attribute is used incorrectly on a trait.
This attribute is used to specify a custom error message when a trait
is not implemented on a type that requires it.

The attribute requires a note to be specified. An empty attribute or
one without meaningful content will trigger this error.",
        "\
Атрибут `#[rustc_on_unimplemented]` использован неправильно.
Этот атрибут используется для указания пользовательского сообщения об ошибке.

Атрибут требует указания примечания. Пустой атрибут вызовет эту ошибку.",
        "\
`#[rustc_on_unimplemented]` 속성이 잘못 사용되었습니다.
이 속성은 트레이트가 구현되지 않았을 때 사용자 정의 오류 메시지를 지정합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add helpful note or remove attribute",
            "Добавьте полезное примечание или удалите атрибут",
            "유용한 노트 추가 또는 속성 제거"
        ),
        code:        "#[rustc_on_unimplemented(message = \"Custom message for {Self}\")]\ntrait MyTrait {}"
    }],
    links:       &[
        DocLink {
            title: "Rust Internals: Custom Error Messages",
            url:   "https://doc.rust-lang.org/unstable-book/language-features/rustc-attrs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0232.html"
        }
    ]
};
