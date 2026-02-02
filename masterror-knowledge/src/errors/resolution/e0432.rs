// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0432: unresolved import

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0432",
    title:       LocalizedText::new(
        "Unresolved import",
        "Неразрешённый импорт",
        "해결되지 않은 임포트"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An import could not be resolved. Rust cannot find or resolve the import
statement. Common causes:
- Incorrect import path
- Misspelled module or item name
- Item not publicly accessible
- Missing extern crate declaration (Rust 2015)",
        "\
Импорт не может быть разрешён. Rust не может найти или разрешить
оператор импорта. Распространённые причины:
- Неправильный путь импорта
- Опечатка в имени модуля или элемента
- Элемент не является публичным
- Отсутствует объявление extern crate (Rust 2015)",
        "\
임포트를 해결할 수 없습니다. Rust가 임포트 문을 찾거나 해결할 수
없습니다. 일반적인 원인:
- 잘못된 임포트 경로
- 모듈 또는 항목 이름 오타
- 항목에 공개 접근 불가
- extern crate 선언 누락 (Rust 2015)"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use self:: prefix for relative imports",
                "Использовать префикс self:: для относительных импортов",
                "상대 임포트에 self:: 접두사 사용"
            ),
            code:        "use self::something::Foo;\n\nmod something {\n    pub struct Foo;\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use crate:: for imports from current crate",
                "Использовать crate:: для импорта из текущего крейта",
                "현재 크레이트에서 임포트할 때 crate:: 사용"
            ),
            code:        "use crate::my_module::MyType;"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0432.html"
    }]
};
