// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0431: invalid self import

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0431",
    title:       LocalizedText::new(
        "self import can only appear with non-empty prefix",
        "Импорт self может использоваться только с непустым префиксом",
        "self 임포트는 비어 있지 않은 접두사와 함께만 사용 가능"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An attempt was made to import the current module into itself using self
without a proper prefix. The self keyword in an import list can only be
used with a non-empty prefix - you cannot import the current module into
itself.",
        "\
Попытка импортировать текущий модуль в себя с помощью self без
правильного префикса. Ключевое слово self в списке импортов может
использоваться только с непустым префиксом.",
        "\
적절한 접두사 없이 self를 사용하여 현재 모듈을 자신에게 임포트하려고
시도했습니다. 임포트 목록의 self 키워드는 비어 있지 않은 접두사와
함께만 사용할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove the invalid import",
            "Удалить недопустимый импорт",
            "유효하지 않은 임포트 제거"
        ),
        code:        "// Remove: use {self};\n// Instead, just access items directly"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0431.html"
    }]
};
