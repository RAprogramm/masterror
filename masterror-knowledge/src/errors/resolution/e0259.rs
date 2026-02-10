// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0259: duplicate external crate name

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0259",
    title:       LocalizedText::new(
        "Duplicate external crate name",
        "Дублирующееся имя внешнего крейта",
        "중복된 외부 크레이트 이름"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The name chosen for an external crate conflicts with another external
crate that has been imported into the current module.

You cannot import two external crates using the same name in the
same scope. One of them must be renamed using the `as` keyword.",
        "\
Выбранное имя для внешнего крейта конфликтует с другим внешним крейтом,
который уже импортирован в текущий модуль.

Нельзя импортировать два внешних крейта с одинаковым именем в одну
область видимости.",
        "\
외부 크레이트에 선택한 이름이 현재 모듈에 이미 임포트된 다른 외부 크레이트와 충돌합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Choose different name with 'as' keyword",
            "Выберите другое имя с помощью ключевого слова 'as'",
            "'as' 키워드로 다른 이름 선택"
        ),
        code:        "extern crate core;\nextern crate std as other_name;"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Extern Crates",
            url:   "https://doc.rust-lang.org/reference/items/extern-crates.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0259.html"
        }
    ]
};
