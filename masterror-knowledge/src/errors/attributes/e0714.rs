// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0714: marker trait with associated items

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0714",
    title:       LocalizedText::new(
        "Marker trait with associated items",
        "Маркерный трейт с ассоциированными элементами",
        "연관 항목이 있는 마커 트레이트"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A `#[marker]` trait contained an associated item.

Marker traits cannot have associated items like constants, because the items
of marker traits cannot be overridden, making them unnecessary when they
cannot be changed per-type anyway.",
        "\
Маркерный трейт `#[marker]` содержит ассоциированный элемент.

Маркерные трейты не могут иметь ассоциированных элементов, так как
их нельзя переопределить.",
        "\
`#[marker]` 트레이트에 연관 항목이 포함되었습니다.

마커 트레이트는 연관 항목을 가질 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use extension trait for associated items",
                "Используйте расширяющий трейт для элементов",
                "연관 항목에 확장 트레이트 사용"
            ),
            code:        "#[marker]\ntrait Marker {}\n\ntrait MarkerExt: Marker {\n    const N: usize;\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0714.html"
        }
    ]
};
