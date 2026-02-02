// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA009: Public API size - maximum 5 methods

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA009",
    title:        LocalizedText::new(
        "Public API size: maximum 5 methods",
        "Размер публичного API: максимум 5 методов",
        "공개 API 크기: 최대 5개 메서드"
    ),
    category:     PracticeCategory::Design,
    explanation:  LocalizedText::new(
        "\
A structure's public interface should have no more than 5 methods.
More methods signal the structure does too much.

Large APIs indicate mixed responsibilities, forcing users to understand more,
expanding documentation and testing complexity.

Solution: Extract secondary concerns into separate types.
Excludes: trait implementations (Display, Debug, From) and generic new().",
        "\
Публичный интерфейс структуры должен иметь не более 5 методов.
Больше методов означает, что структура делает слишком много.",
        "\
구조체의 공개 인터페이스는 5개 이하의 메서드를 가져야 합니다."
    ),
    good_example: r#"impl Document {
    pub fn new() -> Self { ... }
    pub fn load(path: &Path) -> Result<Self> { ... }
    pub fn save(&self) -> Result<()> { ... }
    pub fn content(&self) -> &str { ... }
    pub fn metadata(&self) -> &Metadata { ... }
}

// Rendering is separate
impl Renderer { ... }
// Export is separate
impl Exporter { ... }"#,
    bad_example:  r#"impl Document {
    pub fn new() -> Self { ... }
    pub fn load() -> Result<Self> { ... }
    pub fn save() -> Result<()> { ... }
    pub fn content(&self) -> &str { ... }
    pub fn metadata(&self) -> &Metadata { ... }
    pub fn render_html(&self) -> String { ... }
    pub fn render_pdf(&self) -> Vec<u8> { ... }
    pub fn export_json(&self) -> String { ... }
    pub fn validate(&self) -> Result<()> { ... }
    // ... 10+ more methods
}"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#4-public-api-size"
};
