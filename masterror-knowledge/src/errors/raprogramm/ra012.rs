// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA012: Constant encapsulation - associate with types

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA012",
    title:        LocalizedText::new(
        "Encapsulate constants in their types",
        "Инкапсулируйте константы в их типы",
        "상수를 타입에 캡슐화"
    ),
    category:     PracticeCategory::Design,
    explanation:  LocalizedText::new(
        "\
Constants belong to structures using them, not global scope. This improves
discoverability and prevents namespace pollution.

Benefits:
- Clear discovery location
- Built-in documentation
- Automatic namespacing
- Encapsulation
- Easy refactoring",
        "\
Константы принадлежат структурам, которые их используют, а не глобальной области.
Это улучшает обнаруживаемость и предотвращает загрязнение пространства имён.",
        "\
상수는 전역 범위가 아닌 사용하는 구조체에 속합니다."
    ),
    good_example: r#"impl ConnectionPool {
    pub const MAX_SIZE: usize = 100;
}

impl Client {
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
}

// Usage: ConnectionPool::MAX_SIZE"#,
    bad_example:  r#"const MAX_POOL_SIZE: usize = 100;
const DEFAULT_CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RETRIES: u32 = 3;
const DEFAULT_PORT: u16 = 8080;
// ... scattered constants"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#8-constant-encapsulation"
};
