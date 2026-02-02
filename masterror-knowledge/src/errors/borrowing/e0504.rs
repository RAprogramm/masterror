// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0504: cannot move borrowed variable into closure

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0504",
    title:       LocalizedText::new(
        "Cannot move borrowed value into closure",
        "Нельзя переместить заимствованное значение в замыкание",
        "빌린 값을 클로저로 이동할 수 없음"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
This error occurs when attempting to move a borrowed variable into a closure
using the `move` keyword. A value cannot be moved into a closure while it is
being borrowed elsewhere, as this would invalidate the existing borrow.

Note: This error code is no longer emitted by the compiler.",
        "\
Эта ошибка возникает при попытке переместить заимствованную переменную
в замыкание с помощью ключевого слова `move`. Значение нельзя переместить
в замыкание, пока оно заимствовано в другом месте.

Примечание: этот код ошибки больше не выдаётся компилятором.",
        "\
이 오류는 `move` 키워드를 사용하여 빌린 변수를 클로저로 이동하려고 할 때 발생합니다.
값이 다른 곳에서 빌려진 동안에는 클로저로 이동할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use a reference in the closure instead",
                "Использовать ссылку в замыкании",
                "클로저에서 참조 사용"
            ),
            code:        "let x = move || { println!(\"{}\", fancy_ref.num); };"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Limit borrow lifetime with a scoped block",
                "Ограничить время жизни заимствования блоком",
                "스코프 블록으로 빌림 수명 제한"
            ),
            code:        "{ let r = &val; use(r); } // r dropped\nlet x = move || use(val);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use Arc for shared ownership in threads",
                "Использовать Arc для разделяемого владения",
                "스레드에서 공유 소유권을 위해 Arc 사용"
            ),
            code:        "use std::sync::Arc;\nlet shared = Arc::new(val);\nlet clone = shared.clone();"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0504.html"
    }]
};
