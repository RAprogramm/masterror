// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0634: conflicting packed representation hints

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0634",
    title:       LocalizedText::new(
        "Conflicting packed representation hints",
        "Конфликтующие подсказки packed-представления",
        "충돌하는 packed 표현 힌트"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A type has conflicting `packed` representation hints. You cannot mix
different `packed` directives on a single struct or type definition.

Both `packed` (default packing) and `packed(N)` (N-byte packing) cannot
be used together on the same type.",
        "\
Тип имеет конфликтующие подсказки `packed`-представления. Вы не можете
смешивать разные директивы `packed` в одном определении структуры или типа.

Оба варианта `packed` (упаковка по умолчанию) и `packed(N)` (N-байтовая
упаковка) не могут использоваться вместе для одного типа.",
        "\
타입에 충돌하는 `packed` 표현 힌트가 있습니다. 하나의 구조체나 타입
정의에서 다른 `packed` 지시문을 혼합할 수 없습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Choose one packed representation",
            "Выбрать одно packed-представление",
            "하나의 packed 표현 선택"
        ),
        code:        "#[repr(packed)] // or #[repr(packed(2))]\nstruct Company(i32);"
    }],
    links:       &[
        DocLink {
            title: "Type Layout",
            url:   "https://doc.rust-lang.org/reference/type-layout.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0634.html"
        }
    ]
};
