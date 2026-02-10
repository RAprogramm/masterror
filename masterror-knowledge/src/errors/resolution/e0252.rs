// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0252: two items with same name cannot be imported

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0252",
    title:       LocalizedText::new(
        "Two items with same name cannot be imported without rebinding",
        "Два элемента с одинаковым именем не могут быть импортированы без переименования",
        "같은 이름의 두 항목은 리바인딩 없이 임포트할 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
Two items of the same name cannot be imported without rebinding one
of the items under a new local name.

This error occurs when you attempt to import two items with identical
names from different modules into the same scope without using aliases
to disambiguate them.",
        "\
Два элемента с одинаковым именем не могут быть импортированы без
переименования одного из них.

Эта ошибка возникает при попытке импортировать два элемента с идентичными
именами из разных модулей в одну область видимости.",
        "\
같은 이름의 두 항목은 리바인딩 없이 임포트할 수 없습니다.
별칭을 사용하여 구분해야 합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use aliases with 'as' keyword",
                "Используйте псевдонимы с ключевым словом 'as'",
                "'as' 키워드로 별칭 사용"
            ),
            code:        "use foo::baz as foo_baz;\nuse bar::baz;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Reference with parent module path",
                "Обращайтесь через путь родительского модуля",
                "부모 모듈 경로로 참조"
            ),
            code:        "use bar::baz;\n\nfn main() {\n    let x = foo::baz;  // full path\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Renaming with 'as'",
            url:   "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#providing-new-names-with-the-as-keyword"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0252.html"
        }
    ]
};
