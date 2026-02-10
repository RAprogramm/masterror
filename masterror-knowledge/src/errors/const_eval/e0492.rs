// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0492: borrow of const with interior mutability

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0492",
    title:       LocalizedText::new(
        "Borrow of constant containing interior mutability",
        "Заимствование константы с внутренней изменяемостью",
        "내부 가변성을 포함하는 상수의 빌림"
    ),
    category:    Category::ConstEval,
    explanation: LocalizedText::new(
        "\
An attempt was made to take a reference to a const that contains types with
interior mutability (like AtomicUsize or Cell<T>).

A const represents a constant value that should never change. However, types
with interior mutability allow mutation through shared references. This
creates a contradiction: a constant could theoretically be mutated.

A static is different - it's explicitly a single memory location.",
        "\
Попытка взять ссылку на const, содержащий типы с внутренней изменяемостью
(такие как AtomicUsize или Cell<T>).

Const представляет константное значение, которое не должно меняться.
Однако типы с внутренней изменяемостью позволяют мутацию через
разделяемые ссылки, что создаёт противоречие.",
        "\
내부 가변성을 가진 타입(AtomicUsize 또는 Cell<T> 등)을 포함하는
const에 대한 참조를 가져오려고 시도했습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use static instead of const",
            "Использовать static вместо const",
            "const 대신 static 사용"
        ),
        code:        "use std::sync::atomic::AtomicUsize;\n\nstatic A: AtomicUsize = AtomicUsize::new(0);\nstatic B: &'static AtomicUsize = &A; // ok!"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0492.html"
    }]
};
