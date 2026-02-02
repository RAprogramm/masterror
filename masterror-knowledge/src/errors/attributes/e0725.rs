// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0725: feature not in allowed list

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0725",
    title:       LocalizedText::new(
        "Feature not in allowed list",
        "Функция не в списке разрешённых",
        "허용 목록에 없는 기능"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A feature attribute named a feature that was disallowed in the compiler
command line flags via `-Z allow_features`.

The specified feature is not in the allowed features list.",
        "\
Атрибут feature указывает функцию, запрещённую флагами командной строки
компилятора через `-Z allow_features`.",
        "\
기능 속성이 `-Z allow_features`를 통해 컴파일러 명령줄 플래그에서
허용되지 않은 기능을 지정했습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove disallowed feature or add to allow list",
                "Удалите запрещённую функцию или добавьте в список",
                "허용되지 않은 기능 제거 또는 목록에 추가"
            ),
            code:        "// Remove: #![feature(disallowed_feature)]\n// Or add to -Z allow_features"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0725.html"
        }
    ]
};
