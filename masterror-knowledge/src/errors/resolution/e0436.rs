// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0436: functional record update on non-struct

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0436",
    title:       LocalizedText::new(
        "Functional record update requires a struct",
        "Функциональное обновление записи требует структуру",
        "함수적 레코드 업데이트에는 구조체가 필요함"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The functional record update syntax (..) was used on something other than
a struct. This syntax is only valid for structs, not for enum variants
(even struct-like enum variants).",
        "\
Синтаксис функционального обновления записи (..) использован не для
структуры. Этот синтаксис допустим только для структур, не для
вариантов перечислений (даже структуроподобных).",
        "\
함수적 레코드 업데이트 구문(..)이 구조체가 아닌 것에 사용되었습니다.
이 구문은 구조체에만 유효하며, 열거형 변형(구조체와 유사한 열거형
변형 포함)에는 유효하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Extract and specify fields explicitly",
            "Извлечь и указать поля явно",
            "필드를 명시적으로 추출하고 지정"
        ),
        code:        "match variant {\n    Enum::Variant { field, .. } =>\n        Enum::Variant { field, other: true }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0436.html"
    }]
};
