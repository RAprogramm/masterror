// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA005: No inline comments - use docblocks only

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA005",
    title:        LocalizedText::new(
        "No inline comments - use docblocks only",
        "Никаких инлайн комментариев - только docblocks",
        "인라인 주석 금지 - docblock만 사용"
    ),
    category:     PracticeCategory::Documentation,
    explanation:  LocalizedText::new(
        "\
Avoid // and /* */ explanations in code. All documentation lives in docblocks:
/// for items, //! for modules.

Standardized headings for IDE/LSP stability:
- # Overview - Short purpose statement
- # Examples - Minimal, compilable samples
- # Errors - Precise failure modes for Result types
- # Panics - Only if unavoidable
- # Safety - Required if unsafe code present",
        "\
Избегайте // и /* */ объяснений в коде. Вся документация живёт в docblocks:
/// для элементов, //! для модулей.",
        "\
코드에서 // 및 /* */ 설명을 피하세요. 모든 문서는 docblock에."
    ),
    good_example: r#"/// Fetches user data from the database.
///
/// # Errors
/// Returns `DbError::NotFound` if user doesn't exist.
///
/// # Examples
/// ```
/// let user = fetch_user(42)?;
/// ```
pub fn fetch_user(id: u64) -> Result<User, DbError>"#,
    bad_example:  r#"// This function fetches user data from the database
// It returns an error if user is not found
pub fn fetch_user(id: u64) -> Result<User, DbError>"#,
    source:       "https://github.com/RAprogramm/RustManifest#8-code-documentation"
};
