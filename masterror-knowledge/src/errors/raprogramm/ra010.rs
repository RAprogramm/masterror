// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA010: Constructors should only assign fields

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA010",
    title:        LocalizedText::new(
        "Constructors: assignment only, no logic",
        "Конструкторы: только присваивание, никакой логики",
        "생성자: 할당만, 로직 없음"
    ),
    category:     PracticeCategory::Design,
    explanation:  LocalizedText::new(
        "\
Constructors should only assign fields. All processing, validation, and I/O
belong in methods.

Problems with logic in constructors:
- Constructors can fail, complicating object creation
- Work happens eagerly even if unused
- Inflexible creation paths
- Hard to test without real resources

Benefits of assignment-only constructors:
- Infallible object creation
- Lazy evaluation of expensive work
- Multiple creation paths (from_data() for tests)",
        "\
Конструкторы должны только присваивать поля. Вся обработка, валидация и I/O
принадлежат методам.",
        "\
생성자는 필드만 할당해야 합니다. 모든 처리, 검증, I/O는 메서드에."
    ),
    good_example: r#"impl Server {
    pub fn new(config: Config) -> Self {
        Self { config, connection: None }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.connection = Some(Connection::establish(&self.config)?);
        Ok(())
    }
}"#,
    bad_example:  r#"impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let connection = Connection::establish(&config)?; // I/O in constructor!
        validate_config(&config)?; // Logic in constructor!
        Ok(Self { config, connection })
    }
}"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#5-constructor-design"
};
