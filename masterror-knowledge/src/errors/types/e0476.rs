// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0476: coerced type doesn't outlive the value being coerced to

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0476",
    title:       LocalizedText::new(
        "Coerced type doesn't outlive the value being coerced to",
        "Приводимый тип не переживает значение, к которому приводится",
        "강제 변환된 타입이 변환 대상 값보다 오래 살지 않음"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
This error occurs during type coercion when the source pointer's lifetime
does not outlive the lifetime bound of the target type. In the coercion,
'b (source lifetime) is not a subtype of 'a (target lifetime requirement).

This error can currently only be encountered with the unstable CoerceUnsized
trait.",
        "\
Эта ошибка возникает при приведении типов, когда время жизни исходного
указателя не переживает ограничение времени жизни целевого типа.
При приведении 'b (исходное время жизни) не является подтипом 'a
(требование целевого времени жизни).

Эта ошибка может возникнуть только с нестабильным трейтом CoerceUnsized.",
        "\
이 오류는 타입 강제 변환 중 소스 포인터의 라이프타임이 대상 타입의
라이프타임 바운드보다 오래 살지 않을 때 발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Ensure source lifetime outlives target",
            "Обеспечить, чтобы исходное время жизни пережило целевое",
            "소스 라이프타임이 대상보다 오래 살도록 보장"
        ),
        code:        "// Ensure 'b: 'a (source outlives target)"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0476.html"
    }]
};
