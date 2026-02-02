// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0449: visibility qualifiers not permitted

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0449",
    title:       LocalizedText::new(
        "Visibility qualifiers not permitted here",
        "Модификаторы видимости здесь не разрешены",
        "여기서 가시성 수식어가 허용되지 않음"
    ),
    category:    Category::Visibility,
    explanation: LocalizedText::new(
        "\
Visibility qualifiers (like pub) were used in a context where they are
not allowed. Visibility qualifiers cannot be applied to:
- Enum variants (they inherit the enum's visibility)
- Trait items (they inherit the trait's visibility)
- Impl blocks (they inherit the type's visibility)
- Extern blocks",
        "\
Модификаторы видимости (например, pub) использованы там, где они
не разрешены. Модификаторы видимости нельзя применять к:
- Вариантам перечислений (они наследуют видимость перечисления)
- Элементам трейтов (они наследуют видимость трейта)
- Блокам impl (они наследуют видимость типа)
- Блокам extern",
        "\
가시성 수식어(pub 등)가 허용되지 않는 컨텍스트에서 사용되었습니다.
가시성 수식어는 다음에 적용할 수 없습니다:
- 열거형 변형 (열거형의 가시성 상속)
- 트레이트 항목 (트레이트의 가시성 상속)
- impl 블록 (타입의 가시성 상속)
- extern 블록"
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Remove visibility qualifiers",
            "Удалить модификаторы видимости",
            "가시성 수식어 제거"
        ),
        code:        "impl Foo for Bar {\n    fn foo() {} // Remove pub\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0449.html"
    }]
};
