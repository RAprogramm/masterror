// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0464: multiple matching crates found

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0464",
    title:       LocalizedText::new(
        "Multiple library files found with the same crate name",
        "Найдено несколько библиотек с одним именем крейта",
        "같은 크레이트 이름을 가진 여러 라이브러리 파일이 발견됨"
    ),
    category:    Category::Linking,
    explanation: LocalizedText::new(
        "\
The Rust compiler found multiple library files with the same crate name,
making it unable to determine which one to use. This can occur when:
- Multiple versions of a crate exist in the search path
- Caching issues with the build directory
- Using extern crate without specifying the path",
        "\
Компилятор Rust нашёл несколько библиотечных файлов с одинаковым именем
крейта и не может определить, какой использовать. Это может произойти:
- Несколько версий крейта в пути поиска
- Проблемы кэширования в директории сборки
- Использование extern crate без указания пути",
        "\
Rust 컴파일러가 같은 크레이트 이름을 가진 여러 라이브러리 파일을
발견하여 어떤 것을 사용할지 결정할 수 없습니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Clean the build directory",
                "Очистить директорию сборки",
                "빌드 디렉토리 정리"
            ),
            code:        "cargo clean"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Specify the full path to the crate",
                "Указать полный путь к крейту",
                "크레이트에 대한 전체 경로 지정"
            ),
            code:        "rustc --extern crate_name=/path/to/libcrate.rlib main.rs"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0464.html"
    }]
};
