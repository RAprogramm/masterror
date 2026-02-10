// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0446: private type in public interface

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0446",
    title:       LocalizedText::new(
        "Private type in public interface",
        "Приватный тип в публичном интерфейсе",
        "공개 인터페이스의 비공개 타입"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
A private type or trait was exposed through a public associated type in a
trait implementation. Since the trait or associated type is public, external
code can see and access it, but the underlying type is private.",
        "\
Приватный тип или трейт раскрыт через публичный ассоциированный тип в
реализации трейта. Поскольку трейт или ассоциированный тип публичен,
внешний код может его видеть, но базовый тип приватен.",
        "\
비공개 타입 또는 트레이트가 트레이트 구현의 공개 연관 타입을 통해
노출되었습니다. 트레이트 또는 연관 타입이 공개이므로 외부 코드가
볼 수 있지만 기본 타입은 비공개입니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Restrict the trait's visibility",
                "Ограничить видимость трейта",
                "트레이트의 가시성 제한"
            ),
            code:        "struct Bar;\n\npub(crate) trait PubTr {\n    type Alias;\n}\n\nimpl PubTr for u8 {\n    type Alias = Bar;\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Make the private type public",
                "Сделать приватный тип публичным",
                "비공개 타입을 공개로 만들기"
            ),
            code:        "pub struct Bar;\n\npub trait PubTr {\n    type Alias;\n}\n\nimpl PubTr for u8 {\n    type Alias = Bar;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0446.html"
    }]
};
