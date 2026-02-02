// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0802: invalid CoercePointee derive target

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0802",
    title:       LocalizedText::new(
        "Invalid CoercePointee derive target",
        "Недопустимая цель для derive(CoercePointee)",
        "잘못된 CoercePointee derive 대상"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
The target of `derive(CoercePointee)` macro has inadmissible specification.

The `CoercePointee` macro requires ALL of the following:
1. Must be a `struct` (not enum or other type)
2. Must be generic over at least one type parameter
3. Must have `#[repr(transparent)]` layout
4. Must have at least one data field
5. Must designate exactly one generic type as pointee with `#[pointee]`
6. The pointee type must be marked `?Sized`",
        "\
Цель макроса `derive(CoercePointee)` имеет недопустимую спецификацию.

Макрос `CoercePointee` требует ВСЕ из следующего:
1. Должна быть `struct` (не enum или другой тип)
2. Должна быть обобщённой хотя бы по одному параметру типа
3. Должна иметь атрибут `#[repr(transparent)]`
4. Должна иметь хотя бы одно поле данных
5. Ровно один обобщённый тип должен быть помечен `#[pointee]`
6. Тип pointee должен быть помечен `?Sized`",
        "\
`derive(CoercePointee)` 매크로의 대상이 허용되지 않는 사양을 가지고 있습니다.
모든 요구 사항을 충족해야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Correct CoercePointee usage",
            "Правильное использование CoercePointee",
            "올바른 CoercePointee 사용법"
        ),
        code:        "\
#[derive(CoercePointee)]
#[repr(transparent)]
struct MyPointer<'a, #[pointee] T: ?Sized> {
    ptr: &'a T,
}"
    }],
    links:       &[
        DocLink {
            title: "CoercePointee Documentation",
            url:   "https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0802.html"
        }
    ]
};
