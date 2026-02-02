// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0422: identifier used as struct but is not a struct

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0422",
    title:       LocalizedText::new(
        "Identifier is not a struct",
        "Идентификатор не является структурой",
        "식별자가 구조체가 아님"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
An identifier was used with struct instantiation syntax ({ }) but it is
either undefined or not a struct. This happens when:
- The identifier doesn't exist at all
- The identifier refers to a variable, not a struct type",
        "\
Идентификатор использован с синтаксисом создания структуры ({ }),
но он либо не определён, либо не является структурой. Это происходит когда:
- Идентификатор вообще не существует
- Идентификатор ссылается на переменную, а не на тип структуры",
        "\
식별자가 구조체 인스턴스화 구문({ })으로 사용되었지만 정의되지 않았거나
구조체가 아닙니다. 다음 경우에 발생합니다:
- 식별자가 존재하지 않음
- 식별자가 구조체 타입이 아닌 변수를 참조함"
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Define the struct first",
            "Сначала определить структуру",
            "먼저 구조체 정의"
        ),
        code:        "struct Foo { x: i32, y: i32 }\n\nlet x = Foo { x: 1, y: 2 };"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0422.html"
    }]
};
