// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0509: cannot move out of type X, which implements the Drop trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0509",
    title:       LocalizedText::new(
        "Cannot move out of type which implements Drop",
        "Нельзя переместить из типа, реализующего Drop",
        "Drop을 구현하는 타입에서 이동할 수 없음"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
Types that implement Drop have custom cleanup logic that runs when they're
destroyed. Moving a field out would leave the struct in a partially valid
state, and Drop wouldn't know what to clean up.

Rust prevents this to ensure Drop always sees a valid value.",
        "\
Типы с Drop имеют пользовательскую логику очистки при уничтожении.
Перемещение поля оставит структуру в частично валидном состоянии,
и Drop не будет знать, что очищать.

Rust предотвращает это для гарантии валидности значения в Drop.",
        "\
Drop을 구현하는 타입은 파괴될 때 실행되는 사용자 정의 정리 로직이 있습니다.
필드를 이동하면 구조체가 부분적으로 유효한 상태가 됩니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new("Clone the field", "Клонировать поле", "필드 복제"),
            code:        "let field = self.field.clone();"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use Option and take()",
                "Использовать Option и take()",
                "Option과 take() 사용"
            ),
            code:        "struct S { field: Option<T> }\nlet field = self.field.take();"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0509.html"
    }]
};
