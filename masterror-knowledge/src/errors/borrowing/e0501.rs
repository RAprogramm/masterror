// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0501: cannot borrow X as mutable because previous closure requires unique
//! access

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0501",
    title:       LocalizedText::new(
        "Cannot borrow because closure requires unique access",
        "Нельзя заимствовать, так как замыкание требует уникальный доступ",
        "클로저가 고유 접근을 필요로 하여 빌릴 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
A closure has captured a variable mutably, and now you're trying to borrow
that same variable again. The closure's capture acts like a mutable borrow
that lasts for the closure's entire lifetime.",
        "\
Замыкание захватило переменную изменяемо, и теперь вы пытаетесь заимствовать
ту же переменную снова.",
        "\
클로저가 변수를 가변으로 캡처했고, 이제 같은 변수를 다시 빌리려고 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use the closure before borrowing again",
            "Использовать замыкание перед повторным заимствованием",
            "다시 빌리기 전에 클로저 사용"
        ),
        code:        "let mut c = || x += 1;\nc(); // use closure\nlet r = &x; // now safe"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0501.html"
    }]
};
