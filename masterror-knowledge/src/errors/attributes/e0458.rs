// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0458: unknown link kind

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0458",
    title:       LocalizedText::new(
        "Unknown link kind in link attribute",
        "Неизвестный тип ссылки в атрибуте link",
        "link 속성의 알 수 없는 링크 종류"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An invalid kind value was specified in a #[link] attribute. The kind
parameter only accepts specific valid values for linking strategies.

Note: This error is no longer emitted by modern compilers.",
        "\
Недопустимое значение kind указано в атрибуте #[link]. Параметр kind
принимает только определённые допустимые значения для стратегий связывания.

Примечание: эта ошибка больше не выдаётся современными компиляторами.",
        "\
#[link] 속성에 유효하지 않은 kind 값이 지정되었습니다. kind
매개변수는 링크 전략에 대한 특정 유효한 값만 허용합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a valid link kind",
            "Использовать допустимый тип ссылки",
            "유효한 링크 종류 사용"
        ),
        code:        "// Valid kinds: static, dylib, framework (macOS), raw-dylib (Windows)\n#[link(kind = \"static\", name = \"foo\")] extern \"C\" {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0458.html"
    }]
};
