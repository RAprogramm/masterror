// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0520: specialization requires parent impl to be `default`

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0520",
    title:       LocalizedText::new(
        "Specialization requires parent impl to be `default`",
        "Специализация требует, чтобы родительская реализация была `default`",
        "특수화는 부모 impl이 `default`이어야 함"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A non-default implementation was already made on this type so it cannot be
specialized further. When using the `specialization` feature, you can only
override a parent implementation if that parent is marked as `default`.

The specialization feature allows more specific implementations to override
generic ones, but all intermediate implementations in the hierarchy must be
marked `default` to permit further specialization.",
        "\
Не-default реализация уже была сделана для этого типа, поэтому её нельзя
специализировать дальше. При использовании функции `specialization` можно
переопределить родительскую реализацию, только если она помечена как `default`.",
        "\
이 타입에 대해 이미 non-default 구현이 만들어졌으므로 더 이상 특수화할 수
없습니다. `specialization` 기능을 사용할 때 부모 구현이 `default`로
표시된 경우에만 재정의할 수 있습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Mark parent implementations as `default`",
            "Пометить родительские реализации как `default`",
            "부모 구현을 `default`로 표시"
        ),
        code:        "impl<T: Clone> SpaceLlama for T {\n    default fn fly(&self) {} // add default\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0520.html"
    }]
};
