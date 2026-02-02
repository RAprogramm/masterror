// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0521: borrowed data escapes outside of closure

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0521",
    title:       LocalizedText::new(
        "Borrowed data escapes outside of closure",
        "Заимствованные данные выходят за пределы замыкания",
        "빌린 데이터가 클로저 외부로 탈출함"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
This error occurs when borrowed data is used within a closure in a way that
causes it to escape the closure's scope. When you explicitly annotate a closure
parameter with a type that includes a reference, it creates a new lifetime
declaration that may be incompatible with how the borrowed data is being used.

The issue typically arises when a closure parameter is explicitly annotated
with a reference type, and that reference is then stored or used in a way
that extends beyond the closure's lifetime.",
        "\
Эта ошибка возникает, когда заимствованные данные используются в замыкании
так, что они выходят за пределы области видимости замыкания. Когда вы явно
аннотируете параметр замыкания типом со ссылкой, создаётся новое объявление
времени жизни, которое может быть несовместимо с использованием данных.",
        "\
이 오류는 빌린 데이터가 클로저 내에서 클로저의 스코프를 벗어나는 방식으로
사용될 때 발생합니다. 참조를 포함하는 타입으로 클로저 매개변수를 명시적으로
어노테이션하면 새로운 라이프타임이 생성됩니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove explicit type annotation, let compiler infer",
            "Удалить явную аннотацию типа",
            "명시적 타입 어노테이션 제거"
        ),
        code:        "let mut list: Vec<&str> = Vec::new();\nlet _add = |el| { list.push(el); }; // no type annotation"
    }],
    links:       &[
        DocLink {
            title: "Closure Type Inference",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html#closure-type-inference-and-annotation"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0521.html"
        }
    ]
};
