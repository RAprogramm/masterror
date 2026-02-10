// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0607: cast between thin and wide pointer

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0607",
    title:       LocalizedText::new(
        "Cannot cast between thin and wide pointer",
        "Невозможно привести между тонким и широким указателем",
        "얇은 포인터와 넓은 포인터 간 캐스팅 불가"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A cast between a thin and a wide pointer was attempted.

Thin pointers contain only a memory address. Wide pointers reference
Dynamically Sized Types (DSTs) and carry additional metadata:
- Slices: address + length
- Trait objects: address + vtable pointer

You cannot directly cast between these pointer types.",
        "\
Была предпринята попытка приведения между тонким и широким указателем.

Тонкие указатели содержат только адрес памяти. Широкие указатели ссылаются
на типы с динамическим размером (DST) и несут дополнительные метаданные:
- Срезы: адрес + длина
- Трейт-объекты: адрес + указатель на vtable

Вы не можете напрямую приводить между этими типами указателей.",
        "\
얇은 포인터와 넓은 포인터 간 캐스팅이 시도되었습니다.

얇은 포인터는 메모리 주소만 포함합니다. 넓은 포인터는 동적 크기 타입(DST)을
참조하며 추가 메타데이터를 포함합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use proper Rust constructs instead of casting",
            "Использовать правильные конструкции Rust вместо приведения",
            "캐스팅 대신 적절한 Rust 구조 사용"
        ),
        code:        "// Create slice from array properly\nlet arr = [1, 2, 3];\nlet slice: &[i32] = &arr;"
    }],
    links:       &[
        DocLink {
            title: "Type Cast Expressions",
            url:   "https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0607.html"
        }
    ]
};
