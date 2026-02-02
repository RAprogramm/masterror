// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0430: self import appears more than once

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0430",
    title:       LocalizedText::new(
        "self import appears more than once",
        "Импорт self появляется более одного раза",
        "self 임포트가 두 번 이상 나타남"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The self import appeared multiple times in a single use statement. The self
import can only appear once in an import list.",
        "\
Импорт self появился несколько раз в одном операторе use. Импорт self
может появляться только один раз в списке импортов.",
        "\
단일 use 문에서 self 임포트가 여러 번 나타났습니다. self 임포트는
임포트 목록에서 한 번만 나타날 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove duplicate self import",
            "Удалить повторный импорт self",
            "중복 self 임포트 제거"
        ),
        code:        "use something::{self}; // Only one self"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0430.html"
    }]
};
