// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0503: cannot use X because it was mutably borrowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0503",
    title:       LocalizedText::new(
        "Cannot use value because it was mutably borrowed",
        "Нельзя использовать значение, так как оно изменяемо заимствовано",
        "가변으로 빌려져서 값을 사용할 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
While a mutable borrow is active, you cannot access the original value
in any way. This prevents you from observing partially modified state
or creating aliased mutable references.

The mutable borrow has exclusive access until it ends.",
        "\
Пока активно изменяемое заимствование, вы не можете обращаться к
исходному значению никак.",
        "\
가변 빌림이 활성화된 동안 원래 값에 어떤 방식으로도 접근할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "End the mutable borrow first",
            "Сначала завершить изменяемое заимствование",
            "먼저 가변 빌림 종료"
        ),
        code:        "{ let r = &mut x; modify(r); } // r dropped\nuse_value(&x);"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0503.html"
    }]
};
