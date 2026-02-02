// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0453: forbid overruled by allow

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0453",
    title:       LocalizedText::new(
        "Lint forbid overruled by inner attribute",
        "Запрет линта нарушен внутренним атрибутом",
        "내부 속성에 의해 린트 금지가 무시됨"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
An attempt was made to override a forbid lint directive using an inner
attribute like #[allow(...)]. The forbid lint setting is stricter than deny
because it prevents itself from being overridden by any inner attributes.

- forbid: turns warning into error AND prevents overriding
- deny: turns warning into error BUT allows overriding",
        "\
Попытка переопределить директиву forbid с помощью внутреннего атрибута
#[allow(...)]. Настройка forbid строже чем deny, так как она запрещает
переопределение внутренними атрибутами.

- forbid: превращает предупреждение в ошибку И запрещает переопределение
- deny: превращает предупреждение в ошибку НО позволяет переопределение",
        "\
#[allow(...)]와 같은 내부 속성을 사용하여 forbid 린트 지시문을
재정의하려고 시도했습니다. forbid 린트 설정은 내부 속성에 의한
재정의를 방지하므로 deny보다 엄격합니다."
    ),
    fixes:       &[
        FixSuggestion {
            description: LocalizedText::new(
                "Replace forbid with deny to allow overriding",
                "Заменить forbid на deny для разрешения переопределения",
                "재정의를 허용하려면 forbid를 deny로 교체"
            ),
            code:        "#![deny(non_snake_case)]\n\n#[allow(non_snake_case)]\nfn main() {\n    let MyNumber = 2; // ok!\n}"
        },
        FixSuggestion {
            description: LocalizedText::new(
                "Fix the code to comply with the lint",
                "Исправить код для соответствия линту",
                "린트에 맞게 코드 수정"
            ),
            code:        "#![forbid(non_snake_case)]\n\nfn main() {\n    let my_number = 2;\n}"
        }
    ],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0453.html"
    }]
};
