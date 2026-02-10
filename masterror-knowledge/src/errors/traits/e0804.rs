// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0804: cannot add auto trait via pointer cast

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0804",
    title:       LocalizedText::new(
        "Cannot add auto trait via pointer cast",
        "Нельзя добавить auto trait через приведение указателя",
        "포인터 캐스트로 auto trait 추가 불가"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
An auto trait cannot be added to the bounds of a `dyn Trait` type via a
pointer cast.

Adding an auto trait through a pointer cast can make the vtable invalid,
potentially causing undefined behavior in safe code. The vtable associated
with a trait object may not have entries for all methods when an auto trait
is added after the fact.

Example of invalid code:
`let ptr: *const dyn Any = &();`
`_ = ptr as *const (dyn Any + Send);`  // E0804

This is dangerous because the vtable for the original trait object may not
contain entries for methods that require the auto trait bound.",
        "\
Auto trait нельзя добавить к границам типа `dyn Trait` через приведение
указателя.

Добавление auto trait через приведение указателя может сделать vtable
недействительной, что потенциально вызовет неопределённое поведение.
Vtable, связанная с trait object, может не иметь записей для всех методов,
когда auto trait добавляется постфактум.

Пример неверного кода:
`let ptr: *const dyn Any = &();`
`_ = ptr as *const (dyn Any + Send);`  // E0804",
        "\
포인터 캐스트를 통해 `dyn Trait` 타입의 바운드에 auto trait를 추가할 수 없습니다.
이는 vtable을 무효화하여 정의되지 않은 동작을 일으킬 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Ensure trait object has correct bounds from start",
            "Убедиться, что trait object имеет правильные границы с самого начала",
            "처음부터 올바른 바운드로 trait object 생성"
        ),
        code:        "\
// Create trait object with correct bounds from the start
let ptr: *const (dyn Any + Send) = &();"
    }],
    links:       &[
        DocLink {
            title: "Trait Objects",
            url:   "https://doc.rust-lang.org/book/ch17-02-trait-objects.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0804.html"
        }
    ]
};
