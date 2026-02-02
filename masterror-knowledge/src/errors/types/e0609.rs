// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0609: no field on type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0609",
    title:       LocalizedText::new("No field on type", "Нет поля у типа", "타입에 필드가 없음"),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Attempted to access a nonexistent field in a struct. This is typically
caused by:
- Misspelling the field name
- Referencing a field that was never defined in the struct",
        "\
Попытка доступа к несуществующему полю структуры. Обычно это вызвано:
- Опечаткой в имени поля
- Обращением к полю, которое никогда не было определено в структуре",
        "\
구조체에 존재하지 않는 필드에 접근하려고 시도했습니다. 일반적으로:
- 필드 이름 오타
- 구조체에 정의되지 않은 필드 참조"
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Check the field name spelling",
            "Проверить правописание имени поля",
            "필드 이름 철자 확인"
        ),
        code:        "struct Foo { x: u32 }\nlet f = Foo { x: 0 };\nprintln!(\"{}\", f.x); // correct field name"
    }],
    links:       &[
        DocLink {
            title: "Structs",
            url:   "https://doc.rust-lang.org/book/ch05-01-defining-structs.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0609.html"
        }
    ]
};
