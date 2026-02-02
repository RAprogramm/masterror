// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0510: cannot assign in match guard

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0510",
    title:       LocalizedText::new(
        "Cannot assign in match guard",
        "Нельзя присваивать в охранном выражении match",
        "매치 가드에서 할당할 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
The matched value was assigned in a match guard. This is not allowed because
mutating the matched value in a guard could cause the match to become
non-exhaustive, as it might change which pattern arm should execute.

The guard expression is evaluated after pattern matching but before the arm
body executes, so modifying the matched value would require re-evaluating
previous patterns.",
        "\
Сопоставляемое значение было изменено в охранном выражении match.
Это запрещено, поскольку изменение значения в охране может нарушить
полноту сопоставления, так как может измениться подходящая ветвь.

Охранное выражение вычисляется после сопоставления с образцом, но до
выполнения тела ветви.",
        "\
매치 가드에서 매칭된 값이 할당되었습니다. 가드에서 매칭된 값을 변경하면
매치가 비완전해질 수 있어 허용되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Move mutation into match arm body",
            "Переместить изменение в тело ветви",
            "변경을 매치 암 본문으로 이동"
        ),
        code:        "match x {\n    Some(_) => { x = None; } // ok in body\n    None => {}\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0510.html"
    }]
};
