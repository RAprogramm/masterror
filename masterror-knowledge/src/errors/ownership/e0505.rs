// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0505: cannot move out of X because it is borrowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0505",
    title:       LocalizedText::new(
        "Cannot move out because it is borrowed",
        "Нельзя переместить, так как значение заимствовано",
        "빌려져 있어서 이동할 수 없음"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
You're trying to move a value while a borrow of it still exists.
This would invalidate the reference, creating a dangling pointer.

The borrow must end (go out of scope) before you can move the value.

Rust tracks the lifetime of all borrows to prevent this at compile time.",
        "\
Вы пытаетесь переместить значение, пока существует его заимствование.
Это сделает ссылку недействительной.

Заимствование должно закончиться до перемещения значения.",
        "\
빌림이 존재하는 동안 값을 이동하려고 합니다.
이것은 참조를 무효화하여 댕글링 포인터를 만듭니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "End the borrow before moving",
                "Завершить заимствование перед перемещением",
                "이동 전에 빌림 종료"
            ),
            code:        "{ let r = &x; use(r); } // borrow ends\nmove_value(x);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Clone before borrowing",
                "Клонировать перед заимствованием",
                "빌리기 전에 복제"
            ),
            code:        "let cloned = x.clone();\nlet r = &cloned;\nmove_value(x);"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0505.html"
    }]
};
