// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0572: return statement outside of function body

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0572",
    title:       LocalizedText::new(
        "Return statement outside of function body",
        "Оператор return вне тела функции",
        "함수 본문 외부의 return 문"
    ),
    category:    Category::Types,
    explanation: LocalizedText::new(
        "\
A `return` statement was found in a context where it is not allowed. The
`return` keyword can only be used inside function bodies, not in other
contexts like constant declarations or module-level code.",
        "\
Оператор `return` был найден в контексте, где он не разрешён. Ключевое
слово `return` можно использовать только внутри тел функций, но не в
других контекстах, таких как объявления констант или код на уровне модуля.",
        "\
`return` 문이 허용되지 않는 컨텍스트에서 발견되었습니다. `return` 키워드는
상수 선언이나 모듈 수준 코드와 같은 다른 컨텍스트가 아닌 함수 본문 내에서만
사용할 수 있습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove return from const declaration",
                "Удалить return из объявления const",
                "const 선언에서 return 제거"
            ),
            code:        "const FOO: u32 = 0;  // not: const FOO: u32 = return 0;"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Move return into a function",
                "Переместить return в функцию",
                "return을 함수로 이동"
            ),
            code:        "fn some_fn() -> u32 {\n    return FOO;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0572.html"
    }]
};
