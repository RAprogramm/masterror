// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0478: lifetime bound not satisfied

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0478",
    title:       LocalizedText::new(
        "Lifetime bound not satisfied",
        "Ограничение времени жизни не выполнено",
        "라이프타임 바운드가 충족되지 않음"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A lifetime parameter doesn't satisfy the required lifetime bounds of a trait
or type. When a trait has a lifetime bound (superbound), any type implementing
or using that trait must ensure its lifetimes respect those bounds.

The bound 'a: 'b means 'a must live at least as long as 'b.",
        "\
Параметр времени жизни не удовлетворяет требуемым ограничениям трейта
или типа. Когда трейт имеет ограничение времени жизни (superbound),
любой тип, реализующий или использующий этот трейт, должен обеспечить
соответствие своих времён жизни этим ограничениям.

Ограничение 'a: 'b означает, что 'a должно жить не меньше чем 'b.",
        "\
라이프타임 매개변수가 트레이트 또는 타입의 필수 라이프타임 바운드를
충족하지 않습니다. 'a: 'b 바운드는 'a가 최소한 'b만큼 살아야 함을
의미합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Add lifetime bound to enforce relationship",
            "Добавить ограничение времени жизни",
            "관계를 강제하기 위해 라이프타임 바운드 추가"
        ),
        code:        "struct Prince<'kiss, 'snow: 'kiss> {\n    child: Box<dyn Wedding<'kiss> + 'snow>,\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0478.html"
    }]
};
