// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0593: closure/function argument count mismatch

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0593",
    title:       LocalizedText::new(
        "Closure/function has wrong number of arguments",
        "Замыкание/функция имеет неправильное количество аргументов",
        "클로저/함수의 인수 수가 잘못됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
You supplied an `Fn`-based type with an incorrect number of arguments compared
to what was expected. The closure or function signature doesn't match the
required number of parameters.

This error occurs when the closure or function provided has a different arity
(number of parameters) than what the trait bound specifies.",
        "\
Вы передали тип на основе `Fn` с неправильным количеством аргументов
по сравнению с ожидаемым. Сигнатура замыкания или функции не соответствует
требуемому количеству параметров.",
        "\
예상보다 잘못된 수의 인수를 가진 `Fn` 기반 타입을 제공했습니다.
클로저 또는 함수 시그니처가 필요한 매개변수 수와 일치하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Match the expected argument count",
            "Сопоставить ожидаемое количество аргументов",
            "예상 인수 수와 일치"
        ),
        code:        "fn foo<F: Fn()>(x: F) { }\n\nfoo(|| { }); // 0 arguments, matching Fn()"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0593.html"
    }]
};
