// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0365: private modules cannot be publicly re-exported

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0365",
    title:       LocalizedText::new(
        "Private modules cannot be publicly re-exported",
        "Приватные модули нельзя публично реэкспортировать",
        "비공개 모듈은 공개적으로 재내보내기할 수 없음"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
You attempted to use `pub use` to re-export a module that is not itself marked
as `pub`. In Rust, you cannot make a private module publicly accessible through
re-exporting.

When using `pub use` to re-export a module, the module being re-exported must
first be declared with the `pub` visibility modifier.",
        "\
Вы попытались использовать `pub use` для реэкспорта модуля, который сам
не помечен как `pub`. В Rust нельзя сделать приватный модуль публично
доступным через реэкспорт.

При использовании `pub use` для реэкспорта модуля, реэкспортируемый модуль
должен быть объявлен с модификатором видимости `pub`.",
        "\
`pub use`를 사용하여 `pub`로 표시되지 않은 모듈을 재내보내기하려고 했습니다.
Rust에서는 재내보내기를 통해 비공개 모듈을 공개적으로 접근 가능하게 만들 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Mark the module as pub before re-exporting",
            "Пометить модуль как pub перед реэкспортом",
            "재내보내기 전에 모듈을 pub로 표시"
        ),
        code:        "pub mod foo {\n    pub const X: u32 = 1;\n}\n\npub use foo as foo2;  // ok!"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Use Declarations",
            url:   "https://doc.rust-lang.org/reference/items/use-declarations.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0365.html"
        }
    ]
};
