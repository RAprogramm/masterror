// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0574: expected struct/variant/union, found something else

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0574",
    title:       LocalizedText::new(
        "Expected struct, variant or union, found something else",
        "Ожидалась структура, вариант или объединение",
        "구조체, 변형 또는 유니온이 예상되었지만 다른 것 발견"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
Something other than a struct, variant or union has been used when one was
expected. This error occurs when you attempt to instantiate or pattern match
against something that isn't a struct, variant, or union.

The compiler expects a concrete type that can be instantiated with field
initialization syntax `{ }`, but received something else (like a module or
an enum instead of its variant).",
        "\
Вместо структуры, варианта или объединения было использовано что-то другое.
Эта ошибка возникает при попытке создать экземпляр или сопоставить с образцом
что-то, что не является структурой, вариантом или объединением.",
        "\
구조체, 변형 또는 유니온이 예상되는 곳에 다른 것이 사용되었습니다.
이 오류는 구조체, 변형 또는 유니온이 아닌 것을 인스턴스화하거나
패턴 매칭하려고 할 때 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use full path to struct in module",
                "Использовать полный путь к структуре в модуле",
                "모듈 내 구조체의 전체 경로 사용"
            ),
            code:        "mod mordor { pub struct TheRing { pub x: usize } }\nlet sauron = mordor::TheRing { x: 1 };"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Match enum variant, not the enum itself",
                "Сопоставлять вариант перечисления, а не само перечисление",
                "열거형 자체가 아닌 열거형 변형과 매칭"
            ),
            code:        "match eco {\n    Jak::Daxter { i } => {} // not just Jak { i }\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0574.html"
    }]
};
