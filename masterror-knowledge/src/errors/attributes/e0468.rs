// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0468: macro import from non-root module

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0468",
    title:       LocalizedText::new(
        "Non-root module tried to import macros from another crate",
        "Нерневой модуль попытался импортировать макросы из другого крейта",
        "비루트 모듈이 다른 크레이트에서 매크로를 임포트하려고 시도함"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An attempt was made to use #[macro_use] with extern crate in a non-root
module. The Rust compiler only allows macro imports from external crates
when the extern crate declaration is at the crate root level.",
        "\
Попытка использовать #[macro_use] с extern crate в некорневом модуле.
Компилятор Rust разрешает импорт макросов из внешних крейтов только
когда объявление extern crate находится на корневом уровне крейта.",
        "\
비루트 모듈에서 extern crate와 함께 #[macro_use]를 사용하려고
시도했습니다. Rust 컴파일러는 extern crate 선언이 크레이트 루트
수준에 있을 때만 외부 크레이트에서 매크로 임포트를 허용합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Move macro import to crate root",
            "Переместить импорт макросов в корень крейта",
            "매크로 임포트를 크레이트 루트로 이동"
        ),
        code:        "// In lib.rs or main.rs:\n#[macro_use]\nextern crate some_crate;\n\nmod foo {\n    fn run_macro() { some_macro!(); }\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0468.html"
    }]
};
