// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0459: link without name parameter

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0459",
    title:       LocalizedText::new(
        "Link specified without name parameter",
        "Ссылка указана без параметра name",
        "name 매개변수 없이 link가 지정됨"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
The #[link(...)] attribute was used in an extern block without specifying
the required name parameter. The Rust compiler needs the name parameter
to know which library to link against.",
        "\
Атрибут #[link(...)] использован в блоке extern без указания
обязательного параметра name. Компилятору Rust нужен параметр name,
чтобы знать, с какой библиотекой связываться.",
        "\
extern 블록에서 필수 name 매개변수를 지정하지 않고 #[link(...)]
속성이 사용되었습니다. Rust 컴파일러는 어떤 라이브러리에 연결할지
알기 위해 name 매개변수가 필요합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add the name parameter",
            "Добавить параметр name",
            "name 매개변수 추가"
        ),
        code:        "#[link(kind = \"dylib\", name = \"some_lib\")] extern \"C\" {}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0459.html"
    }]
};
