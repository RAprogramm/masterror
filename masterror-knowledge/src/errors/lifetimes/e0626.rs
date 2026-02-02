// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0626: borrow persists across yield point

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0626",
    title:       LocalizedText::new(
        "Borrow in coroutine persists across yield point",
        "Заимствование в сопрограмме сохраняется через точку yield",
        "코루틴의 빌림이 yield 지점을 넘어 지속됨"
    ),
    category:    Category::Lifetimes,
    explanation: LocalizedText::new(
        "\
A borrow remains in scope across a `yield` point in a movable (unmarked)
coroutine. This is not permitted because the coroutine could be moved while
the borrow is still active, violating borrow safety rules.

In an unmarked (movable) coroutine, you cannot have a borrow that is still
in scope when a `yield` occurs.",
        "\
Заимствование остаётся в области видимости через точку `yield` в подвижной
(непомеченной) сопрограмме. Это не разрешено, потому что сопрограмма может
быть перемещена, пока заимствование ещё активно, нарушая правила
безопасности заимствования.

В непомеченной (подвижной) сопрограмме у вас не может быть заимствования,
которое ещё находится в области видимости при выполнении `yield`.",
        "\
이동 가능한(표시되지 않은) 코루틴에서 빌림이 `yield` 지점을 넘어
범위 내에 남아 있습니다. 코루틴이 빌림이 활성화된 상태에서 이동될 수
있어 빌림 안전 규칙을 위반할 수 있으므로 허용되지 않습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Mark the coroutine as static",
                "Пометить сопрограмму как static",
                "코루틴을 static으로 표시"
            ),
            code:        "let mut b = #[coroutine] static || {\n    let a = &String::from(\"hello\");\n    yield ();\n    println!(\"{}\", a);\n};"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Store by value instead of borrowing",
                "Хранить по значению вместо заимствования",
                "빌림 대신 값으로 저장"
            ),
            code:        "let mut b = #[coroutine] || {\n    let a = String::from(\"hello\");\n    yield ();\n    println!(\"{}\", a);\n};"
        }
    ],
    links:       &[
        DocLink {
            title: "Coroutines",
            url:   "https://doc.rust-lang.org/std/ops/trait.Coroutine.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0626.html"
        }
    ]
};
