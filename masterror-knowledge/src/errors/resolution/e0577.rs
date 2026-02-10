// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0577: non-module in visibility scope

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0577",
    title:       LocalizedText::new(
        "Expected module in visibility path, found something else",
        "Ожидался модуль в пути видимости, найдено что-то другое",
        "가시성 경로에서 모듈이 예상되었지만 다른 것 발견"
    ),
    category:    Category::Resolution,
    explanation: LocalizedText::new(
        "\
Something other than a module was found in visibility scope. The
`pub(in path)` syntax requires that the path points to a module, not other
types like enums or structs.

Additionally, the visibility scope can only be applied to ancestors in the
module hierarchy.",
        "\
В области видимости было найдено что-то, кроме модуля. Синтаксис
`pub(in path)` требует, чтобы путь указывал на модуль, а не на другие
типы, такие как перечисления или структуры.

Кроме того, область видимости может применяться только к предкам
в иерархии модулей.",
        "\
가시성 스코프에서 모듈이 아닌 다른 것이 발견되었습니다. `pub(in path)` 구문은
경로가 열거형이나 구조체와 같은 다른 타입이 아닌 모듈을 가리켜야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Use a module path instead of enum/struct",
            "Использовать путь к модулю вместо enum/struct",
            "enum/struct 대신 모듈 경로 사용"
        ),
        code:        "pub mod sea {\n    pub (in crate::sea) struct Shark; // ok\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0577.html"
    }]
};
