// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0519: current crate indistinguishable from dependency

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0519",
    title:       LocalizedText::new(
        "Current crate indistinguishable from dependency",
        "Текущий крейт неотличим от зависимости",
        "현재 크레이트가 의존성과 구별 불가"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The current crate is indistinguishable from one of its dependencies. This
error occurs when the current crate has the same `crate-name` and metadata
as one of its dependencies.

This creates ambiguity because the compiler cannot distinguish between symbols
(public item names) from the two crates, potentially causing symbol conflicts.",
        "\
Текущий крейт неотличим от одной из его зависимостей. Эта ошибка возникает,
когда текущий крейт имеет то же имя и метаданные, что и одна из его
зависимостей.

Это создаёт неоднозначность, поскольку компилятор не может различить символы
из двух крейтов.",
        "\
현재 크레이트가 의존성 중 하나와 구별할 수 없습니다. 이 오류는 현재 크레이트가
의존성 중 하나와 동일한 `crate-name` 및 메타데이터를 가질 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use Cargo to manage crate naming",
                "Использовать Cargo для управления именами крейтов",
                "크레이트 이름 지정에 Cargo 사용"
            ),
            code:        "// Use Cargo.toml to manage dependencies\n// It handles crate naming automatically"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Change the crate name to be unique",
                "Изменить имя крейта на уникальное",
                "크레이트 이름을 고유하게 변경"
            ),
            code:        "#![crate_name = \"my_unique_crate\"]"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0519.html"
    }]
};
