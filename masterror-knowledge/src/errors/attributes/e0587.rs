// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0587: packed and align on same type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0587",
    title:       LocalizedText::new(
        "Cannot use both `packed` and `align` on same type",
        "Нельзя использовать `packed` и `align` для одного типа",
        "같은 타입에 `packed`와 `align`을 모두 사용할 수 없음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type has both `packed` and `align` representation hints. You cannot use both
on the same type because they provide conflicting layout specifications.

`packed` removes padding between fields to minimize size, while `align`
specifies a minimum alignment requirement.",
        "\
Тип имеет оба атрибута представления `packed` и `align`. Нельзя использовать
оба для одного типа, поскольку они задают конфликтующие спецификации
размещения.

`packed` удаляет отступы между полями, а `align` указывает минимальное
требование выравнивания.",
        "\
타입에 `packed`와 `align` 표현 힌트가 모두 있습니다. 이들은 충돌하는
레이아웃 사양을 제공하므로 같은 타입에 둘 다 사용할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use packed(N) to specify both packing and size",
            "Использовать packed(N) для указания упаковки и размера",
            "패킹과 크기를 지정하려면 packed(N) 사용"
        ),
        code:        "#[repr(packed(8))]  // not #[repr(packed, align(8))]\nstruct Umbrella(i32);"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0587.html"
    }]
};
