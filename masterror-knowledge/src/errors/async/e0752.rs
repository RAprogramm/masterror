// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0752: async entry point not allowed

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0752",
    title:       LocalizedText::new(
        "Async entry point not allowed",
        "Асинхронная точка входа не допускается",
        "async 진입점 허용되지 않음"
    ),
    category:    Category::Async,
    explanation: LocalizedText::new(
        "\
The entry point of the program was marked as `async`. The `fn main()` function
or the specified start function is not allowed to be `async`.

The program entry point must be synchronous. To use async code, you need an
async runtime (like tokio or async-std) that will manage the async execution.",
        "\
Точка входа программы была помечена как `async`. Функция `fn main()`
не может быть асинхронной.

Для использования async кода нужен runtime (tokio, async-std).",
        "\
프로그램 진입점이 `async`로 표시되었습니다. `fn main()` 함수는
`async`일 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Remove async from main",
                "Удалите async из main",
                "main에서 async 제거"
            ),
            code:        "fn main() -> Result<(), ()> {\n    Ok(())\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use async runtime macro",
                "Используйте макрос async runtime",
                "async 런타임 매크로 사용"
            ),
            code:        "#[tokio::main]\nasync fn main() {\n    // async code\n}"
        }
    ],
    links:       &[
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0752.html"
        }
    ]
};
