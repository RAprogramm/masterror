// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0260: name conflict with external crate

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0260",
    title:       LocalizedText::new(
        "Name conflict with external crate",
        "Конфликт имени с внешним крейтом",
        "외부 크레이트와 이름 충돌"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An item was declared with the same name as an external crate that has
been imported into your project.

You cannot have a local item declaration with the same name as an
external crate that's already been imported. You must either rename
your item or import the crate under a different name.",
        "\
Элемент был объявлен с тем же именем, что и внешний крейт, импортированный
в проект.

Нельзя иметь локальное объявление элемента с тем же именем, что и
импортированный внешний крейт.",
        "\
프로젝트에 임포트된 외부 크레이트와 같은 이름으로 항목이 선언되었습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Rename the item",
                "Переименуйте элемент",
                "항목 이름 변경"
            ),
            code:        "extern crate core;\n\nstruct xyz;  // renamed from core"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Import crate with alias",
                "Импортируйте крейт с псевдонимом",
                "별칭으로 크레이트 임포트"
            ),
            code:        "extern crate core as xyz;\n\nstruct core;  // now allowed"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Extern Crates",
            url:   "https://doc.rust-lang.org/reference/items/extern-crates.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0260.html"
        }
    ]
};
