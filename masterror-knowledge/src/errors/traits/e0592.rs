// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0592: duplicate method/associated function definition

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0592",
    title:       LocalizedText::new(
        "Duplicate method or associated function definition",
        "Дублирующееся определение метода или ассоциированной функции",
        "중복된 메서드 또는 연관 함수 정의"
    ),
    category:    Category::Traits,
    explanation: LocalizedText::new(
        "\
Methods or associated functions with the same name were defined in separate
`impl` blocks for the same struct/type. Each function must have a unique name
within the implementation scope.

This is different from E0201, which occurs when duplicate definitions appear
in a single declaration block.",
        "\
Методы или ассоциированные функции с одинаковым именем были определены
в отдельных блоках `impl` для одной и той же структуры/типа. Каждая функция
должна иметь уникальное имя в области реализации.",
        "\
같은 이름의 메서드나 연관 함수가 같은 구조체/타입에 대해 별도의 `impl`
블록에서 정의되었습니다. 각 함수는 구현 범위 내에서 고유한 이름을
가져야 합니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Give each function a unique name",
            "Дать каждой функции уникальное имя",
            "각 함수에 고유한 이름 부여"
        ),
        code:        "impl Foo {\n    fn bar() {}\n}\nimpl Foo {\n    fn baz() {} // different name\n}"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0592.html"
    }]
};
