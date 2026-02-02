// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0445: private trait in public interface

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0445",
    title:       LocalizedText::new(
        "Private trait in public interface",
        "Приватный трейт в публичном интерфейсе",
        "공개 인터페이스의 비공개 트레이트"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
A private trait was used as a constraint on a public type parameter in a
public interface. This violates visibility rules because public APIs should
not expose private types.

Note: This error is no longer emitted by modern compiler versions.",
        "\
Приватный трейт использован как ограничение публичного параметра типа
в публичном интерфейсе. Это нарушает правила видимости, так как
публичные API не должны раскрывать приватные типы.

Примечание: эта ошибка больше не выдаётся современными версиями компилятора.",
        "\
비공개 트레이트가 공개 인터페이스에서 공개 타입 매개변수의 제약으로
사용되었습니다. 이는 공개 API가 비공개 타입을 노출해서는 안 되기
때문에 가시성 규칙을 위반합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Make the trait public",
            "Сделать трейт публичным",
            "트레이트를 공개로 만들기"
        ),
        code:        "pub trait Foo { }\npub fn foo<T: Foo>(t: T) {} // ok!"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0445.html"
    }]
};
