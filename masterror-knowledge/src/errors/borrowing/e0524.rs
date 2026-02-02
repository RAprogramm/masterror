// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0524: variable requiring unique access used in multiple closures

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0524",
    title:       LocalizedText::new(
        "Two closures require unique access to same variable",
        "Два замыкания требуют уникальный доступ к одной переменной",
        "두 클로저가 같은 변수에 대한 고유 접근 필요"
    ),
    category:    Category::Borrowing,
    explanation: LocalizedText::new(
        "\
A variable which requires unique access is being used in more than one closure
at the same time. Since mutable references require exclusive access, Rust's
borrow checker prevents this to maintain memory safety.

This error occurs when you attempt to borrow a mutable variable in multiple
closures simultaneously.",
        "\
Переменная, требующая уникального доступа, используется одновременно
в нескольких замыканиях. Поскольку изменяемые ссылки требуют
эксклюзивного доступа, проверка заимствований Rust предотвращает это.",
        "\
고유 접근이 필요한 변수가 동시에 둘 이상의 클로저에서 사용되고 있습니다.
가변 참조는 배타적 접근이 필요하므로 Rust의 빌림 검사기가 이를 방지합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use Rc<RefCell<T>> for shared mutable access",
                "Использовать Rc<RefCell<T>> для общего изменяемого доступа",
                "공유 가변 접근을 위해 Rc<RefCell<T>> 사용"
            ),
            code:        "use std::rc::Rc;\nuse std::cell::RefCell;\nlet x = Rc::new(RefCell::new(val));\nlet y = Rc::clone(&x);"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Run closures sequentially in separate scopes",
                "Выполнять замыкания последовательно в разных областях",
                "별도의 스코프에서 클로저를 순차적으로 실행"
            ),
            code:        "{ let mut c1 = || set(&mut *x); c1(); }\nlet mut c2 = || set(&mut *x); c2();"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0524.html"
    }]
};
