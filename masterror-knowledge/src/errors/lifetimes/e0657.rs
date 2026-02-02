// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0657: impl Trait captures higher-ranked lifetime

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0657",
    title:       LocalizedText::new(
        "impl Trait captured higher-ranked lifetime",
        "impl Trait захватил время жизни высшего ранга",
        "impl Trait가 고차 라이프타임을 캡처함"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
An `impl Trait` type attempts to capture a higher-ranked lifetime
(a lifetime introduced by a `for<'a>` binder), which is not supported.

`impl Trait` types are only allowed to capture lifetimes from their parent
items, not from any `for<'a>` binders in scope.",
        "\
Тип `impl Trait` пытается захватить время жизни высшего ранга
(время жизни, введённое связывателем `for<'a>`), что не поддерживается.

Типам `impl Trait` разрешено захватывать только времена жизни из их
родительских элементов, но не из связывателей `for<'a>` в области видимости.",
        "\
`impl Trait` 타입이 고차 라이프타임(`for<'a>` 바인더로 도입된
라이프타임)을 캡처하려고 시도하는데, 이는 지원되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Avoid capturing higher-ranked lifetimes in impl Trait",
            "Избегать захвата времён жизни высшего ранга в impl Trait",
            "impl Trait에서 고차 라이프타임 캡처 피하기"
        ),
        code:        "// Refactor to use concrete types for associated types"
    }],
    links:       &[
        DocLink {
            title: "Higher-Ranked Trait Bounds",
            url:   "https://doc.rust-lang.org/nomicon/hrtb.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0657.html"
        }
    ]
};
