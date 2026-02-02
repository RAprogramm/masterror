// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0152: a lang item was redefined

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0152",
    title:       LocalizedText::new(
        "A lang item was redefined",
        "Языковой элемент был переопределён",
        "lang 아이템이 재정의됨"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A lang item was redefined. Lang items are special items in Rust that are
already implemented in the standard library. They provide core functionality
for the language itself.

Unless you are writing a free-standing application (such as a kernel or
embedded system), you should not provide your own implementations of lang items.",
        "\
Языковой элемент был переопределён. Языковые элементы - это специальные
элементы в Rust, которые уже реализованы в стандартной библиотеке.
Они обеспечивают базовую функциональность языка.

Если вы не пишете автономное приложение (ядро или встраиваемую систему),
не следует предоставлять собственные реализации языковых элементов.",
        "\
lang 아이템이 재정의되었습니다. lang 아이템은 표준 라이브러리에 이미
구현된 Rust의 특별한 아이템입니다. 언어 자체의 핵심 기능을 제공합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use #![no_std] for freestanding applications",
            "Использовать #![no_std] для автономных приложений",
            "독립 실행형 애플리케이션에 #![no_std] 사용"
        ),
        code:        "#![no_std]\n// Now you can define lang items"
    }],
    links:       &[
        DocLink {
            title: "Rustonomicon: Lang Items",
            url:   "https://doc.rust-lang.org/nomicon/beneath-std.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0152.html"
        }
    ]
};
