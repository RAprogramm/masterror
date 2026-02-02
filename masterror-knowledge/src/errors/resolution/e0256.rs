// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0256: import conflicts with type or module

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0256",
    title:       LocalizedText::new(
        "Import conflicts with existing type or module",
        "Импорт конфликтует с существующим типом или модулем",
        "임포트가 기존 타입 또는 모듈과 충돌"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An attempt was made to import a type or module using `use` where the
imported name conflicts with an existing type or submodule already
defined in the same module.

Note: This error code is no longer emitted by the compiler.",
        "\
Была попытка импортировать тип или модуль, где импортируемое имя
конфликтует с существующим типом или подмодулем.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
임포트 이름이 같은 모듈에 이미 정의된 타입이나 서브모듈과 충돌합니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use alias with 'as' keyword",
            "Используйте псевдоним с ключевым словом 'as'",
            "'as' 키워드로 별칭 사용"
        ),
        code:        "use foo::Bar as FooBar;\n\ntype Bar = u32;"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Use Keyword",
            url:   "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0256.html"
        }
    ]
};
