// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0532: pattern arm did not match expected kind

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0532",
    title:       LocalizedText::new(
        "Pattern arm doesn't match expected kind",
        "Ветвь образца не соответствует ожидаемому виду",
        "패턴 암이 예상 종류와 일치하지 않음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A pattern in a match arm doesn't match the expected pattern type for that
value. This error occurs when you're trying to match a pattern of one kind
(e.g., a unit variant) when the actual value is a different kind (e.g., a
tuple variant).

Ensure that the pattern in your match arm matches the structure of the variant.",
        "\
Образец в ветви match не соответствует ожидаемому типу образца для этого
значения. Эта ошибка возникает при попытке сопоставить образец одного вида
(например, единичный вариант), когда фактическое значение другого вида
(например, кортежный вариант).",
        "\
매치 암의 패턴이 해당 값에 대해 예상되는 패턴 타입과 일치하지 않습니다.
이 오류는 실제 값이 다른 종류(예: 튜플 변형)일 때 한 종류의 패턴(예: 유닛 변형)을
매칭하려고 할 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match tuple variants with parentheses",
            "Сопоставлять кортежные варианты со скобками",
            "튜플 변형을 괄호와 함께 매칭"
        ),
        code:        "match *state {\n    State::Failed(ref msg) => println!(\"Failed: {}\", msg),\n    _ => ()\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0532.html"
    }]
};
