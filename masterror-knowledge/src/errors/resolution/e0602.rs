// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0602: unknown lint

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0602",
    title:       LocalizedText::new(
        "Unknown or invalid lint name",
        "Неизвестное или недопустимое имя линта",
        "알 수 없거나 잘못된 린트 이름"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An unknown or invalid lint name is used on the command line when invoking
the Rust compiler (`rustc`).

This error is triggered when you:
- Misspell a lint name
- Use a lint that no longer exists in the current Rust version
- Pass an invalid/unrecognized lint to the compiler",
        "\
Неизвестное или недопустимое имя линта используется в командной строке
при вызове компилятора Rust (`rustc`).

Эта ошибка возникает, когда вы:
- Допускаете опечатку в имени линта
- Используете линт, который больше не существует в текущей версии Rust
- Передаёте недопустимый/нераспознанный линт компилятору",
        "\
Rust 컴파일러(`rustc`)를 호출할 때 명령줄에서 알 수 없거나 잘못된
린트 이름이 사용되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Check lint spelling and view valid lints",
            "Проверить правописание линта и просмотреть допустимые линты",
            "린트 철자 확인 및 유효한 린트 보기"
        ),
        code:        "rustc -W help"
    }],
    links:       &[
        DocLink {
            title: "Lints",
            url:   "https://doc.rust-lang.org/rustc/lints/index.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0602.html"
        }
    ]
};
