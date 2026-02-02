// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0469: imported macro not found

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0469",
    title:       LocalizedText::new(
        "Imported macro not found in crate",
        "Импортируемый макрос не найден в крейте",
        "임포트된 매크로가 크레이트에서 찾을 수 없음"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A macro listed for import via #[macro_use(macro_name)] cannot be found
in the imported crate. The macro must:
1. Exist in the crate
2. Be exported with the #[macro_export] attribute
3. Be spelled correctly",
        "\
Макрос, указанный для импорта через #[macro_use(macro_name)], не найден
в импортируемом крейте. Макрос должен:
1. Существовать в крейте
2. Быть экспортирован с атрибутом #[macro_export]
3. Быть написан правильно",
        "\
#[macro_use(macro_name)]를 통해 임포트하려는 매크로를 임포트된
크레이트에서 찾을 수 없습니다. 매크로는:
1. 크레이트에 존재해야 함
2. #[macro_export] 속성으로 익스포트되어야 함
3. 올바르게 철자되어야 함"
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Verify macro is exported in the crate",
            "Проверить, что макрос экспортирован в крейте",
            "매크로가 크레이트에서 익스포트되었는지 확인"
        ),
        code:        "// In some_crate:\n#[macro_export]\nmacro_rules! my_macro { ... }\n\n// In your crate:\n#[macro_use(my_macro)]\nextern crate some_crate;"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0469.html"
    }]
};
