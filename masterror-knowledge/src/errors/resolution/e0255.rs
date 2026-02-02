// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0255: duplicate name import

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0255",
    title:       LocalizedText::new(
        "Import name conflicts with existing item",
        "Имя импорта конфликтует с существующим элементом",
        "임포트 이름이 기존 항목과 충돌"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
You can't import a value whose name is the same as another value
already defined in the module.

This error occurs when you try to import an item using `use` that has
the same name as another item already in scope in your module.",
        "\
Нельзя импортировать значение, имя которого совпадает с именем другого
значения, уже определённого в модуле.

Эта ошибка возникает при попытке импортировать элемент с тем же именем,
что и элемент, уже находящийся в области видимости.",
        "\
모듈에 이미 정의된 항목과 같은 이름의 값을 임포트할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use alias with 'as' keyword",
                "Используйте псевдоним с ключевым словом 'as'",
                "'as' 키워드로 별칭 사용"
            ),
            code:        "use bar::foo as bar_foo;\n\nfn foo() {}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use fully qualified path",
                "Используйте полный путь",
                "완전 정규화 경로 사용"
            ),
            code:        "fn foo() {}\n\nfn main() {\n    bar::foo();  // access via module path\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Renaming with 'as'",
            url:   "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#providing-new-names-with-the-as-keyword"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0255.html"
        }
    ]
};
