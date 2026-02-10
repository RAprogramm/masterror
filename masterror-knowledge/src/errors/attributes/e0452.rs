// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0452: malformed lint attribute

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0452",
    title:       LocalizedText::new(
        "Malformed lint attribute",
        "Неправильный атрибут линта",
        "잘못된 형식의 린트 속성"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An invalid lint attribute has been given. Lint attributes must follow a
specific format and only accept a list of identifiers (lint names).
Assignments or string values are not allowed.",
        "\
Задан недопустимый атрибут линта. Атрибуты линтов должны следовать
определённому формату и принимают только список идентификаторов
(имён линтов). Присваивания или строковые значения не допускаются.",
        "\
유효하지 않은 린트 속성이 제공되었습니다. 린트 속성은 특정 형식을
따라야 하며 식별자(린트 이름) 목록만 허용합니다. 할당이나 문자열
값은 허용되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use comma-separated lint identifiers",
            "Использовать идентификаторы линтов через запятую",
            "쉼표로 구분된 린트 식별자 사용"
        ),
        code:        "#![allow(unused, dead_code)]"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0452.html"
    }]
};
