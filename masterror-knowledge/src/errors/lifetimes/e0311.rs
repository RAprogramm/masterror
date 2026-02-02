// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0311: unsatisfied outlives bound with elided region

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0311",
    title:       LocalizedText::new(
        "Unsatisfied outlives bound with elided region",
        "Неудовлетворённое ограничение outlives с опущенным временем жизни",
        "생략된 영역으로 인한 outlives 바운드 불만족"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
This error occurs when there is an unsatisfied outlives bound involving an
elided region and a generic type parameter. The compiler automatically adds
lifetime bounds based on function signatures, and when these don't align with
generic parameter requirements, this error is raised.

The fix is to explicitly name the elided lifetime and add the outlives bound
to the generic parameter.",
        "\
Эта ошибка возникает, когда есть неудовлетворённое ограничение outlives,
включающее опущенное время жизни и параметр обобщённого типа. Компилятор
автоматически добавляет ограничения времени жизни на основе сигнатур функций,
и когда они не согласуются с требованиями параметра, возникает эта ошибка.",
        "\
생략된 영역과 제네릭 타입 매개변수를 포함하는 outlives 바운드가 충족되지 않을 때
이 오류가 발생합니다. 컴파일러는 함수 시그니처를 기반으로 라이프타임 바운드를
자동으로 추가하며, 이것이 제네릭 매개변수 요구사항과 맞지 않으면 이 오류가 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Explicitly name elided lifetime and add bound",
                "Явно указать опущенное время жизни и добавить ограничение",
                "생략된 라이프타임을 명시하고 바운드 추가"
            ),
            code:        "fn no_restriction<'a, T: 'a>(x: &'a ()) -> &'a () {\n    with_restriction::<T>(x)\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Lifetime Elision",
            url:   "https://doc.rust-lang.org/reference/lifetime-elision.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0311.html"
        }
    ]
};
