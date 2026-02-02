// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0576: associated item not found in type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0576",
    title:       LocalizedText::new(
        "Associated item not found in type",
        "Ассоциированный элемент не найден в типе",
        "타입에서 연관 항목을 찾을 수 없음"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An associated item wasn't found in the given type. The compiler cannot find
the specified associated item (such as a type or method) that you're trying
to access.

This usually happens when referencing an associated type or constant that
doesn't exist in the trait or impl.",
        "\
Ассоциированный элемент не был найден в данном типе. Компилятор не может
найти указанный ассоциированный элемент (например, тип или метод), к которому
вы пытаетесь получить доступ.",
        "\
주어진 타입에서 연관 항목을 찾을 수 없습니다. 컴파일러가 접근하려는
지정된 연관 항목(예: 타입 또는 메서드)을 찾을 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use the correct associated type name",
            "Использовать правильное имя ассоциированного типа",
            "올바른 연관 타입 이름 사용"
        ),
        code:        "trait Hello {\n    type Who;\n    fn hello() -> <Self as Hello>::Who; // not ::You\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0576.html"
    }]
};
