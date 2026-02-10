// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0251: duplicate item import

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0251",
    title:       LocalizedText::new(
        "Duplicate item name in imports",
        "Дублирующееся имя элемента при импорте",
        "임포트에서 중복된 항목 이름"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
Two items with the same name are imported into scope without rebinding
one of them under a new local name.

Note: This error code is no longer emitted by the compiler.

The error prevented name conflicts by requiring explicit disambiguation
when the same name was imported from multiple sources.",
        "\
Два элемента с одинаковым именем импортируются в область видимости
без переименования одного из них.

Примечание: Этот код ошибки больше не выдаётся компилятором.",
        "\
같은 이름의 두 항목이 리바인딩 없이 스코프로 임포트되었습니다.
참고: 이 오류 코드는 더 이상 컴파일러에서 발생하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use 'as' keyword to rebind one of the imports",
            "Используйте 'as' для переименования одного из импортов",
            "'as' 키워드로 임포트 중 하나를 리바인딩"
        ),
        code:        "use foo::baz;\nuse bar::baz as bar_baz;"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Bringing Paths into Scope",
            url:   "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0251.html"
        }
    ]
};
