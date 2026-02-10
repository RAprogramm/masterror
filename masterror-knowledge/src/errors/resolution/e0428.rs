// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0428: duplicate definition

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0428",
    title:       LocalizedText::new(
        "Duplicate definition of type or module",
        "Повторное определение типа или модуля",
        "타입 또는 모듈의 중복 정의"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A type or module has been defined more than once in the same scope. Rust
does not allow duplicate definitions of types, structs, enums, or modules
with the same name in the same namespace.",
        "\
Тип или модуль определён более одного раза в одной области видимости.
Rust не допускает повторных определений типов, структур, перечислений
или модулей с одинаковым именем в одном пространстве имён.",
        "\
같은 스코프에서 타입 또는 모듈이 두 번 이상 정의되었습니다. Rust는
같은 네임스페이스에서 같은 이름의 타입, 구조체, 열거형 또는 모듈의
중복 정의를 허용하지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Rename the duplicate definition",
            "Переименовать повторное определение",
            "중복 정의 이름 변경"
        ),
        code:        "struct Bar;\nstruct Bar2; // Renamed from Bar"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0428.html"
    }]
};
