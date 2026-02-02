// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0426: use of undeclared label

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0426",
    title:       LocalizedText::new(
        "Use of undeclared label",
        "Использование необъявленной метки",
        "선언되지 않은 레이블 사용"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A label was used in a break or continue statement that has not been declared.
Labels are used with loop control flow statements to specify which loop to
exit or continue. Labels must be explicitly declared with a single quote
prefix before the loop keyword.",
        "\
В операторе break или continue использована метка, которая не была объявлена.
Метки используются с операторами управления циклами для указания, какой цикл
прервать или продолжить. Метки должны быть явно объявлены с префиксом
одинарной кавычки перед ключевым словом цикла.",
        "\
선언되지 않은 레이블이 break 또는 continue 문에서 사용되었습니다.
레이블은 루프 제어 흐름 문에서 어떤 루프를 종료하거나 계속할지 지정하는 데
사용됩니다. 레이블은 루프 키워드 앞에 작은따옴표 접두사와 함께 명시적으로
선언되어야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Declare the label before the loop",
            "Объявить метку перед циклом",
            "루프 앞에 레이블 선언"
        ),
        code:        "'outer: loop {\n    break 'outer; // Label declared with 'outer:\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0426.html"
    }]
};
