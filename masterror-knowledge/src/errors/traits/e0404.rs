// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0404: expected trait, found type

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0404",
    title:       LocalizedText::new(
        "Expected trait, found type",
        "Ожидался трейт, найден тип",
        "트레이트가 예상되었으나 타입이 발견됨"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
A type that is not a trait was used in a trait position, such as a bound
or impl declaration. Traits are required in certain contexts like:
- Trait bounds: <T: SomeType>
- impl declarations: impl SomeType for MyStruct

Using a struct or type alias in these positions is invalid.",
        "\
Тип, не являющийся трейтом, использован там, где ожидается трейт,
например, в ограничении или объявлении impl. Трейты требуются в:
- Ограничениях типа: <T: SomeType>
- Объявлениях impl: impl SomeType for MyStruct",
        "\
트레이트가 아닌 타입이 트레이트 위치에 사용되었습니다. 트레이트는
다음 컨텍스트에서 필요합니다:
- 트레이트 바운드: <T: SomeType>
- impl 선언: impl SomeType for MyStruct"
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Define an actual trait",
                "Определить настоящий трейт",
                "실제 트레이트 정의"
            ),
            code:        "trait Foo { }\nstruct Bar;\nimpl Foo for Bar { }"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Use trait alias (nightly)",
                "Использовать псевдоним трейта (nightly)",
                "트레이트 별칭 사용 (nightly)"
            ),
            code:        "#![feature(trait_alias)]\ntrait Foo = Iterator<Item=String>;\nfn bar<T: Foo>(t: T) {}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0404.html"
    }]
};
