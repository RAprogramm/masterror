// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0161: cannot move a value of type: the size cannot be statically determined

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0161",
    title:       LocalizedText::new(
        "Cannot move a value of type: the size cannot be statically determined",
        "Нельзя переместить значение: размер не может быть определён статически",
        "값을 이동할 수 없음: 크기를 정적으로 결정할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A value was moved whose size was not known at compile time. In Rust, you can
only move a value when its size is known at compile time. This error occurs
when attempting to move a dynamically-sized type (like dyn Trait).",
        "\
Было перемещено значение, размер которого не известен во время компиляции.
В Rust можно перемещать только значения с известным размером во время
компиляции. Эта ошибка возникает при попытке переместить динамически
размерный тип (например, dyn Trait).",
        "\
컴파일 시간에 크기를 알 수 없는 값이 이동되었습니다. Rust에서는
컴파일 시간에 크기가 알려진 경우에만 값을 이동할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a reference instead of moving",
            "Использовать ссылку вместо перемещения",
            "이동 대신 참조 사용"
        ),
        code:        "trait Bar {\n    fn f(&self); // use &self instead of self\n}\n\nimpl Bar for i32 {\n    fn f(&self) {}\n}\n\nfn main() {\n    let b: Box<dyn Bar> = Box::new(0i32);\n    b.f(); // ok!\n}"
    }],
    links:       &[
        DocLink {
            title: "Rust Reference: Dynamically Sized Types",
            url:   "https://doc.rust-lang.org/reference/dynamically-sized-types.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0161.html"
        }
    ]
};
