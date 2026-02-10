// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0506: cannot assign to X because it is borrowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0506",
    title:       LocalizedText::new(
        "Cannot assign because it is borrowed",
        "Нельзя присвоить, так как значение заимствовано",
        "빌려져 있어서 할당할 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
You're trying to assign to a value while a borrow of it exists.
This would invalidate the existing reference.

You must wait for all borrows to end before assigning a new value.",
        "\
Вы пытаетесь присвоить значение, пока существует его заимствование.",
        "\
빌림이 존재하는 동안 값에 할당하려고 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "End the borrow before assigning",
            "Завершить заимствование перед присваиванием",
            "할당 전에 빌림 종료"
        ),
        code:        "{ let r = &x; use(r); } // borrow ends\nx = new_value;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0506.html"
    }]
};
