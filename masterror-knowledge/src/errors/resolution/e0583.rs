// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0583: file not found for module

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0583",
    title:       LocalizedText::new(
        "File not found for out-of-line module",
        "Файл не найден для внешнего модуля",
        "아웃오브라인 모듈에 대한 파일을 찾을 수 없음"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
A file wasn't found for an out-of-line module. This error occurs when you
declare an out-of-line module in Rust but the corresponding file doesn't
exist in the file system.

For a module named `foo`, you need to create either `foo.rs` or `foo/mod.rs`
in the same directory as the file declaring the module.",
        "\
Файл не был найден для внешнего модуля. Эта ошибка возникает, когда вы
объявляете внешний модуль в Rust, но соответствующий файл не существует
в файловой системе.

Для модуля с именем `foo` нужно создать либо `foo.rs`, либо `foo/mod.rs`.",
        "\
아웃오브라인 모듈에 대한 파일을 찾을 수 없습니다. 이 오류는 Rust에서
아웃오브라인 모듈을 선언했지만 해당 파일이 파일 시스템에 존재하지 않을 때
발생합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Create the module file",
            "Создать файл модуля",
            "모듈 파일 생성"
        ),
        code:        "// Create: file_that_doesnt_exist.rs\n// Or: file_that_doesnt_exist/mod.rs"
    }],
    links:       &[
        DocLink {
            title: "Modules Chapter",
            url:   "https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0583.html"
        }
    ]
};
