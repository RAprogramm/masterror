// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0530: binding shadowed something it shouldn't

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0530",
    title:       LocalizedText::new(
        "Match binding shadows an existing item",
        "Привязка в match затеняет существующий элемент",
        "매치 바인딩이 기존 항목을 섀도잉함"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A binding shadowed something it shouldn't. This error occurs when a match arm
or variable uses a name that conflicts with existing bindings, such as:
- Struct names
- Enum variants
- Statics
- Associated constants

This error also occurs when an enum variant with fields is used in a pattern
without its fields.",
        "\
Привязка затенила то, что не должна была. Эта ошибка возникает, когда ветвь
match или переменная использует имя, конфликтующее с существующими привязками:
- Имена структур
- Варианты перечислений
- Статические переменные
- Ассоциированные константы",
        "\
바인딩이 그래선 안 되는 것을 섀도잉했습니다. 이 오류는 매치 암이나 변수가
기존 바인딩과 충돌하는 이름을 사용할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a different binding name",
                "Использовать другое имя привязки",
                "다른 바인딩 이름 사용"
            ),
            code:        "static TEST: i32 = 0;\nmatch r {\n    some_value => {} // not TEST\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use const instead of static for pattern matching",
                "Использовать const вместо static для сопоставления",
                "패턴 매칭을 위해 static 대신 const 사용"
            ),
            code:        "const TEST: i32 = 0; // const allowed in patterns\nmatch r {\n    TEST => {}\n    _ => {}\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0530.html"
    }]
};
