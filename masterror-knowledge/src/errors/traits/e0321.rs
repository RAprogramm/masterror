// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0321: cross-crate opt-out trait implemented on invalid type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0321",
    title:       LocalizedText::new(
        "Cross-crate opt-out trait on invalid type",
        "Cross-crate opt-out трейт на недопустимом типе",
        "잘못된 타입에 대한 크로스 크레이트 opt-out 트레이트"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A cross-crate opt-out trait (like Send or Sync from core::marker) was
implemented on something that isn't a struct or enum type.

Only structs and enums are permitted to implement Send, Sync, and other
opt-out traits. The struct or enum must be local to the current crate.
Generic types or references to external types cannot implement these traits.",
        "\
Cross-crate opt-out трейт (например Send или Sync из core::marker) был
реализован на чём-то, что не является структурой или перечислением.

Только структуры и перечисления могут реализовывать Send, Sync и другие
opt-out трейты. Тип должен быть локальным для текущего крейта.",
        "\
크로스 크레이트 opt-out 트레이트(core::marker의 Send, Sync 등)가
구조체나 열거형이 아닌 타입에 구현되었습니다.

Send, Sync 및 기타 opt-out 트레이트는 구조체와 열거형만 구현할 수 있습니다.
구조체나 열거형은 현재 크레이트에 로컬이어야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Implement on local struct or enum type only",
                "Реализовать только на локальных структурах/enum",
                "로컬 구조체 또는 열거형 타입에만 구현"
            ),
            code:        "struct Foo;\n\nimpl !Sync for Foo {} // ok - local struct"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Send and Sync",
            url:   "https://doc.rust-lang.org/nomicon/send-and-sync.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0321.html"
        }
    ]
};
