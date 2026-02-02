// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0411: Self used outside impl/trait

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0411",
    title:       LocalizedText::new(
        "Self used outside of impl, trait, or type definition",
        "Self использован вне impl, трейта или определения типа",
        "Self가 impl, 트레이트 또는 타입 정의 외부에서 사용됨"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
The Self keyword was used in a context where it's not valid. Self represents
the current type and can only be used within:
- impl blocks
- trait definitions
- type definitions

Using Self anywhere else results in this error because there is no
\"current type\" to refer to.",
        "\
Ключевое слово Self использовано там, где это недопустимо. Self представляет
текущий тип и может использоваться только внутри:
- блоков impl
- определений трейтов
- определений типов",
        "\
Self 키워드가 유효하지 않은 컨텍스트에서 사용되었습니다. Self는
현재 타입을 나타내며 다음 내에서만 사용할 수 있습니다:
- impl 블록
- 트레이트 정의
- 타입 정의"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Use Self within a trait definition",
                "Использовать Self внутри определения трейта",
                "트레이트 정의 내에서 Self 사용"
            ),
            code:        "trait Baz {\n    fn bar() -> Self;\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Disambiguate with fully qualified syntax",
                "Уточнить с помощью полного синтаксиса",
                "완전한 구문으로 명확히 지정"
            ),
            code:        "trait Baz : Foo {\n    fn bar() -> <Self as Foo>::Bar;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0411.html"
    }]
};
