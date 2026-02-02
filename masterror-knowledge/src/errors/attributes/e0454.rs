// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0454: link with empty name

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0454",
    title:       LocalizedText::new(
        "Link name given with empty name",
        "Имя ссылки указано как пустая строка",
        "링크 이름이 빈 문자열로 제공됨"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A link name was specified with an empty string. The Rust compiler requires
an actual library name to link to external C libraries. An empty name string
provides no valid target for the linker.",
        "\
Имя ссылки указано как пустая строка. Компилятор Rust требует реальное
имя библиотеки для связывания с внешними C библиотеками. Пустая строка
не предоставляет допустимую цель для компоновщика.",
        "\
링크 이름이 빈 문자열로 지정되었습니다. Rust 컴파일러는 외부 C
라이브러리에 연결하기 위해 실제 라이브러리 이름이 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Provide a valid library name",
            "Указать допустимое имя библиотеки",
            "유효한 라이브러리 이름 제공"
        ),
        code:        "#[link(name = \"some_lib\")] extern \"C\" {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0454.html"
    }]
};
