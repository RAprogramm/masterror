// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0614: type cannot be dereferenced

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0614",
    title:       LocalizedText::new(
        "Type cannot be dereferenced",
        "Тип не может быть разыменован",
        "타입을 역참조할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to dereference a variable which cannot be dereferenced.

Only types that implement `std::ops::Deref` can be dereferenced.
Common dereferenceable types include:
- References (`&T`)
- Smart pointers (`Box<T>`, `Rc<T>`)
- Custom types implementing `Deref`",
        "\
Попытка разыменовать переменную, которую нельзя разыменовать.

Только типы, реализующие `std::ops::Deref`, могут быть разыменованы.
Общие разыменовываемые типы включают:
- Ссылки (`&T`)
- Умные указатели (`Box<T>`, `Rc<T>`)
- Пользовательские типы, реализующие `Deref`",
        "\
역참조할 수 없는 변수를 역참조하려고 시도했습니다.

`std::ops::Deref`를 구현하는 타입만 역참조할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Dereference a reference instead",
            "Разыменовать ссылку вместо этого",
            "대신 참조를 역참조"
        ),
        code:        "let y = 0u32;\nlet x = &y;\n*x; // ok - x is &u32"
    }],
    links:       &[
        DocLink {
            title: "std::ops::Deref",
            url:   "https://doc.rust-lang.org/std/ops/trait.Deref.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0614.html"
        }
    ]
};
