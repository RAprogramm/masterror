// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0472: inline assembly not supported on target

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0472",
    title:       LocalizedText::new(
        "Inline assembly not supported on this target",
        "Встроенный ассемблер не поддерживается для этой цели",
        "이 타겟에서 인라인 어셈블리가 지원되지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
An attempt was made to use the asm! macro for inline assembly on a target
architecture that does not support it. While all Tier 1 targets support
inline assembly, support among Tier 2 and 3 targets is not guaranteed.

This error indicates that support for your target is not planned or in
progress (as opposed to E0658 which indicates an unstable feature).",
        "\
Попытка использовать макрос asm! для встроенного ассемблера на целевой
архитектуре, которая его не поддерживает. Хотя все цели Tier 1
поддерживают встроенный ассемблер, поддержка для Tier 2 и 3 не гарантирована.

Эта ошибка указывает, что поддержка для вашей цели не планируется.",
        "\
인라인 어셈블리를 지원하지 않는 타겟 아키텍처에서 asm! 매크로를
사용하려고 시도했습니다. 모든 Tier 1 타겟은 인라인 어셈블리를
지원하지만, Tier 2 및 3 타겟 간의 지원은 보장되지 않습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Write assembly externally and link it",
            "Написать ассемблер отдельно и связать",
            "외부에 어셈블리 작성 후 링크"
        ),
        code:        "// Compile .s file separately and link\n// Or contribute support to Rust"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0472.html"
    }]
};
