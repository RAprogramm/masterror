// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0433",
    title:       LocalizedText::new(
        "Failed to resolve: use of undeclared crate or module",
        "Не удалось разрешить: необъявленный крейт или модуль",
        "해결 실패: 선언되지 않은 크레이트 또는 모듈"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "Rust can't find the crate, module, or type you're trying to use.",
        "Rust не может найти крейт, модуль или тип.",
        "Rust가 크레이트, 모듈 또는 타입을 찾을 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new("Add use statement", "Добавить use", "use 문 추가"),
            code:        "use std::collections::HashMap;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Add dependency",
                "Добавить зависимость",
                "의존성 추가"
            ),
            code:        "[dependencies]\nserde = \"1.0\""
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0433.html"
    }]
};
