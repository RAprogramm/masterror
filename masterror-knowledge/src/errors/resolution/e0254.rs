// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0254: duplicate import with extern crate

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0254",
    title:       LocalizedText::new(
        "Import conflicts with extern crate name",
        "Импорт конфликтует с именем внешнего крейта",
        "임포트가 extern crate 이름과 충돌"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An attempt was made to import an item with a name that conflicts with
an already-imported extern crate.

When you import an extern crate, its name is bound in the current module's
namespace. Attempting to import another item with the same name creates
a naming conflict.",
        "\
Была попытка импортировать элемент с именем, которое конфликтует
с уже импортированным внешним крейтом.

Когда вы импортируете внешний крейт, его имя связывается в пространстве
имён текущего модуля.",
        "\
이미 임포트된 extern crate와 충돌하는 이름의 항목을 임포트하려고 했습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename the extern crate with 'as'",
            "Переименуйте внешний крейт с помощью 'as'",
            "'as'로 extern crate 이름 변경"
        ),
        code:        "extern crate core as libcore;\n\nmod foo {\n    pub trait core {}\n}\n\nuse foo::core;"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Extern Crates",
            url:   "https://doc.rust-lang.org/reference/items/extern-crates.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0254.html"
        }
    ]
};
