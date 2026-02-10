// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0423: identifier used in wrong namespace

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0423",
    title:       LocalizedText::new(
        "Expected value, found struct/module",
        "Ожидалось значение, найдена структура/модуль",
        "값이 예상되었으나 구조체/모듈이 발견됨"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An identifier was used in a way that doesn't match its namespace. Common cases:
- Using a struct name as a function: Foo() instead of Foo { }
- Forgetting ! on macro invocations: println(\"\") instead of println!(\"\")
- Using a module with . instead of :: : a.I instead of a::I",
        "\
Идентификатор использован не в соответствии с его пространством имён:
- Использование имени структуры как функции: Foo() вместо Foo { }
- Забыли ! при вызове макроса: println(\"\") вместо println!(\"\")
- Использование модуля с . вместо :: : a.I вместо a::I",
        "\
식별자가 네임스페이스와 맞지 않는 방식으로 사용되었습니다. 일반적인 경우:
- 구조체 이름을 함수처럼 사용: Foo() 대신 Foo { }
- 매크로 호출에서 ! 누락: println(\"\") 대신 println!(\"\")
- 모듈에 . 대신 :: 사용: a.I 대신 a::I"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Add ! for macro invocation",
                "Добавить ! для вызова макроса",
                "매크로 호출에 ! 추가"
            ),
            code:        "println!(\"Hello\"); // Not println(\"Hello\")"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use :: for module paths",
                "Использовать :: для путей модулей",
                "모듈 경로에 :: 사용"
            ),
            code:        "let x = module::CONSTANT; // Not module.CONSTANT"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0423.html"
    }]
};
