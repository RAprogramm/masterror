// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0373: captured variable may not live long enough

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0373",
    title:       LocalizedText::new(
        "Captured variable may not live long enough",
        "Захваченная переменная может жить недостаточно долго",
        "캡처된 변수가 충분히 오래 살지 않을 수 있음"
    ),
    category:    Category::Ownership,
    explanation: LocalizedText::new(
        "\
This error occurs when attempting to use data captured by a closure after that
data may no longer exist. It's commonly encountered in:

1. Returning closures - Stack-allocated data captured by reference becomes
   invalid once the function returns
2. Spawning threads - The stack frame containing captured variables may
   disappear before the thread completes
3. Using async blocks - Async blocks may capture data by reference but
   execute later

The `move` keyword causes the closure to own the captured data rather than
taking references to it, eliminating lifetime issues.",
        "\
Эта ошибка возникает при попытке использовать данные, захваченные замыканием,
после того как эти данные могут перестать существовать. Часто встречается:

1. При возврате замыканий - данные на стеке, захваченные по ссылке, становятся
   недействительными после возврата из функции
2. При порождении потоков - стековый кадр с захваченными переменными может
   исчезнуть до завершения потока
3. При использовании async-блоков - они могут захватить данные по ссылке,
   но выполниться позже

Ключевое слово `move` заставляет замыкание владеть захваченными данными.",
        "\
이 오류는 클로저에 의해 캡처된 데이터가 더 이상 존재하지 않을 수 있는 시점에
사용하려고 할 때 발생합니다. 일반적으로 다음 상황에서 발생합니다:

1. 클로저 반환 - 참조로 캡처된 스택 할당 데이터는 함수 반환 후 무효화됨
2. 스레드 생성 - 캡처된 변수를 포함하는 스택 프레임이 스레드 완료 전에 사라질 수 있음
3. async 블록 사용 - 참조로 데이터를 캡처하지만 나중에 실행됨"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use move closure to transfer ownership",
                "Использовать move-замыкание для передачи владения",
                "소유권을 이전하기 위해 move 클로저 사용"
            ),
            code:        "fn foo() -> Box<dyn Fn(u32) -> u32> {\n    let x = 0u32;\n    Box::new(move |y| x + y)  // x is moved into closure\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Clone the value before capturing",
                "Клонировать значение перед захватом",
                "캡처 전에 값 복제"
            ),
            code:        "let data = data.clone();\nstd::thread::spawn(move || {\n    // use data\n});"
        }
    ],
    links:       &[
        DocLink {
            title: "Rust Book: Closures",
            url:   "https://doc.rust-lang.org/book/ch13-01-closures.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0373.html"
        }
    ]
};
