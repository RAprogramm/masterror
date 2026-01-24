// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0599",
    title:       LocalizedText::new(
        "No method named X found for type",
        "Метод не найден для типа",
        "타입에서 메서드를 찾을 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "You're calling a method that doesn't exist on this type. Check trait imports.",
        "Вы вызываете метод, который не существует для этого типа. Проверьте импорт трейтов.",
        "이 타입에 존재하지 않는 메서드를 호출하고 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Import the trait",
            "Импортировать трейт",
            "트레이트 import"
        ),
        code:        "use std::io::Read;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0599.html"
    }]
};
