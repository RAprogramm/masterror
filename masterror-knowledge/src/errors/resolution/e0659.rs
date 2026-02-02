// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0659: ambiguous item usage

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0659",
    title:       LocalizedText::new(
        "Ambiguous item usage",
        "Неоднозначное использование элемента",
        "모호한 항목 사용"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An item usage is ambiguous because two or more items with the same name
have been imported into a module, creating a naming conflict.

This error occurs when:
- Multiple items with identical names are imported into the same module
- These items are reexported via wildcard imports (`pub use module::*`)
- You attempt to reference the ambiguous name without a full path",
        "\
Использование элемента неоднозначно, потому что два или более элемента
с одинаковым именем были импортированы в модуль, создавая конфликт имён.

Эта ошибка возникает, когда:
- Несколько элементов с одинаковыми именами импортируются в один модуль
- Эти элементы реэкспортируются через подстановочный импорт (`pub use module::*`)
- Вы пытаетесь обратиться к неоднозначному имени без полного пути",
        "\
동일한 이름을 가진 두 개 이상의 항목이 모듈에 가져와져서 이름 충돌이
발생하여 항목 사용이 모호합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use full path to disambiguate",
            "Использовать полный путь для устранения неоднозначности",
            "모호성 해소를 위해 전체 경로 사용"
        ),
        code:        "mod collider {\n    pub use crate::moon;\n    pub use crate::earth;\n}\n\ncrate::collider::moon::foo(); // disambiguated"
    }],
    links:       &[
        DocLink {
            title: "Use Declarations",
            url:   "https://doc.rust-lang.org/reference/items/use-declarations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0659.html"
        }
    ]
};
