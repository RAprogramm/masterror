// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0451: private field in struct constructor

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0451",
    title:       LocalizedText::new(
        "Struct constructor with private fields invoked",
        "Вызов конструктора структуры с приватными полями",
        "비공개 필드가 있는 구조체 생성자 호출됨"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
An attempt was made to directly instantiate a struct that has private fields
using struct literal syntax. Since the fields are not publicly accessible,
you cannot initialize them directly from outside the module.",
        "\
Попытка напрямую создать экземпляр структуры с приватными полями
с использованием синтаксиса литерала структуры. Поскольку поля
недоступны публично, их нельзя инициализировать извне модуля.",
        "\
구조체 리터럴 구문을 사용하여 비공개 필드가 있는 구조체를 직접
인스턴스화하려고 시도했습니다. 필드가 공개적으로 접근할 수 없으므로
모듈 외부에서 직접 초기화할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Make all fields public",
                "Сделать все поля публичными",
                "모든 필드를 공개로 만들기"
            ),
            code:        "pub struct Foo {\n    pub a: isize,\n    pub b: isize,\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Implement a constructor method",
                "Реализовать метод-конструктор",
                "생성자 메서드 구현"
            ),
            code:        "impl Foo {\n    pub fn new() -> Foo {\n        Foo { a: 0, b: 0 }\n    }\n}\n\nlet f = Foo::new();"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0451.html"
    }]
};
