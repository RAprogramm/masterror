// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0320: recursion limit reached while creating drop-check rules

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0320",
    title:       LocalizedText::new(
        "Recursion limit reached while creating drop-check rules",
        "Достигнут лимит рекурсии при создании правил drop-check",
        "drop-check 규칙 생성 중 재귀 제한 도달"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
The compiler must be able to reason about how a type is Dropped and the types
of its fields to generate proper cleanup code. This error occurs when the
compiler encounters a type with recursive drop-check rules that prevent it
from completing this analysis.

This is not the same as infinite type size - the type has finite size, but
attempting to Drop it would recurse infinitely. The compiler cannot infer
the drop behavior for recursively-defined types.",
        "\
Компилятор должен иметь возможность рассуждать о том, как тип освобождается
(Drop) и о типах его полей для генерации кода очистки. Эта ошибка возникает,
когда компилятор встречает тип с рекурсивными правилами drop-check.

Это не то же самое, что бесконечный размер типа - тип имеет конечный размер,
но попытка вызвать Drop приведёт к бесконечной рекурсии.",
        "\
컴파일러는 적절한 정리 코드를 생성하기 위해 타입이 어떻게 Drop되는지와
필드 타입에 대해 추론할 수 있어야 합니다. 컴파일러가 재귀적 drop-check 규칙을
가진 타입을 만나면 이 오류가 발생합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove recursive structure in type definition",
                "Удалить рекурсивную структуру в определении типа",
                "타입 정의에서 재귀 구조 제거"
            ),
            code:        "// Redesign type hierarchy to avoid infinite\n// recursion in drop-check analysis"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Reference: Drop Trait",
            url:   "https://doc.rust-lang.org/std/ops/trait.Drop.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0320.html"
        }
    ]
};
