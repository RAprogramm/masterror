// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0722: malformed optimize attribute (no longer emitted)

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0722",
    title:       LocalizedText::new(
        "Malformed optimize attribute",
        "Неправильный атрибут optimize",
        "잘못된 optimize 속성"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
Note: This error code is no longer emitted by the compiler (now E0539).

The `#[optimize]` attribute was malformed. Valid arguments are:
- `#[optimize(size)]` - generate smaller code
- `#[optimize(speed)]` - generate faster code",
        "\
Примечание: Эта ошибка больше не выдаётся (теперь E0539).

Атрибут `#[optimize]` был неправильно сформирован.",
        "\
참고: 이 오류 코드는 더 이상 발생하지 않습니다 (현재 E0539)."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use valid optimize argument",
                "Используйте допустимый аргумент optimize",
                "유효한 optimize 인수 사용"
            ),
            code:        "#[optimize(size)]\npub fn small_fn() {}"
        }
    ],
    links:       &[
        DocLink {
            title: "RFC 2412",
            url:   "https://rust-lang.github.io/rfcs/2412-optimize-attr.html"
        }
    ]
};
