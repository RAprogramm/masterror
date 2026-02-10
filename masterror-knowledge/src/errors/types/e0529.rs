// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0529: expected array or slice, found different type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0529",
    title:       LocalizedText::new(
        "Expected an array or slice, found different type",
        "Ожидался массив или срез, найден другой тип",
        "배열 또는 슬라이스가 예상되었지만 다른 타입 발견"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An array or slice pattern was matched against a value that is not an array
or slice type. The pattern syntax `[a, b]` expects the matched expression
to be an array or slice, but a different type was provided.

Ensure the pattern and the expression being matched are of consistent types.",
        "\
Образец массива или среза был сопоставлен со значением, которое не является
массивом или срезом. Синтаксис образца `[a, b]` ожидает, что сопоставляемое
выражение будет массивом или срезом.",
        "\
배열 또는 슬라이스 패턴이 배열이나 슬라이스 타입이 아닌 값과 매칭되었습니다.
패턴 구문 `[a, b]`는 매칭되는 표현식이 배열 또는 슬라이스일 것으로 예상합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use an array or slice as the matched value",
            "Использовать массив или срез как сопоставляемое значение",
            "매칭되는 값으로 배열 또는 슬라이스 사용"
        ),
        code:        "let r = [1.0, 2.0];\nmatch r {\n    [a, b] => println!(\"a={}, b={}\", a, b),\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0529.html"
    }]
};
