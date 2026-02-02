// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0493: value with Drop may be dropped during const-eval

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0493",
    title:       LocalizedText::new(
        "Value with custom Drop may be dropped during const-eval",
        "Значение с Drop может быть уничтожено при const-вычислении",
        "커스텀 Drop을 가진 값이 const-eval 중에 드롭될 수 있음"
    ),
    category:    Category::ConstEval,
    explanation: LocalizedText::new(
        "\
A value that implements the Drop trait was used in a const context (like a
static initializer). The issue is that Drop implementations can execute
arbitrary code that isn't const-checked, which violates the constraints
of compile-time evaluation.

Drop logic could have unpredictable side effects and must be deterministic
and verifiable at compile-time.",
        "\
Значение, реализующее трейт Drop, использовано в const-контексте
(например, в инициализаторе static). Проблема в том, что реализации
Drop могут выполнять произвольный код, который не проверяется на
const-совместимость, что нарушает ограничения вычисления во время
компиляции.",
        "\
Drop 트레이트를 구현하는 값이 const 컨텍스트(예: static 초기화자)에서
사용되었습니다. Drop 구현은 const-checked되지 않는 임의의 코드를
실행할 수 있어 컴파일 타임 평가 제약을 위반합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Initialize fields directly without temporaries",
            "Инициализировать поля напрямую без временных значений",
            "임시 값 없이 필드 직접 초기화"
        ),
        code:        "static FOO: Foo = Foo { field1: DropType::A }; // Direct init"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0493.html"
    }]
};
