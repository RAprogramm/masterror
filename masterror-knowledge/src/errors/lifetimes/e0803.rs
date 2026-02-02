// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0803: lifetime mismatch in trait implementation

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0803",
    title:       LocalizedText::new(
        "Lifetime mismatch in trait implementation",
        "Несоответствие времён жизни в реализации трейта",
        "트레이트 구현에서 라이프타임 불일치"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A trait implementation returns a reference without an explicit lifetime
linking it to `self`. This commonly arises in generic trait implementations
requiring explicit lifetime bounds.

The issue occurs when:
- A trait method returns a reference
- The trait is implemented for a struct with a lifetime parameter
- The compiler cannot verify if the returned reference satisfies constraints

Solution: Explicitly bind lifetimes in the trait definition and implementation.",
        "\
Реализация трейта возвращает ссылку без явного времени жизни,
связывающего её с `self`. Это часто возникает в обобщённых
реализациях трейтов, требующих явных ограничений времени жизни.

Проблема возникает когда:
- Метод трейта возвращает ссылку
- Трейт реализован для структуры с параметром времени жизни
- Компилятор не может проверить, удовлетворяет ли ссылка ограничениям

Решение: явно привязать времена жизни в определении и реализации трейта.",
        "\
트레이트 구현이 `self`에 명시적 라이프타임 연결 없이 참조를 반환합니다.
이는 명시적 라이프타임 바운드가 필요한 제네릭 트레이트 구현에서 자주 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add lifetime parameter to trait",
            "Добавить параметр времени жизни в трейт",
            "트레이트에 라이프타임 매개변수 추가"
        ),
        code:        "\
trait DataAccess<'a, T> {
    fn get_ref(&'a self) -> T;
}

impl<'a> DataAccess<'a, &'a f64> for Container<'a> {
    fn get_ref(&'a self) -> &'a f64 {
        self.value
    }
}"
    }],
    links:       &[
        DocLink {
            title: "Rust Book: Lifetimes",
            url:   "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0803.html"
        }
    ]
};
