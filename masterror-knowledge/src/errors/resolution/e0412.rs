// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0412",
    title:       LocalizedText::new(
        "Cannot find type in this scope",
        "Не удаётся найти тип в этой области видимости",
        "이 스코프에서 타입을 찾을 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "The type you're referencing doesn't exist or isn't in scope.",
        "Тип, на который вы ссылаетесь, не существует или не в области видимости.",
        "참조하는 타입이 존재하지 않거나 스코프에 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new("Import the type", "Импортировать тип", "타입 import"),
        code:        "use crate::types::MyType;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0412.html"
    }]
};
