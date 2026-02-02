// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA004: Use descriptive, meaningful names

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA004",
    title:        LocalizedText::new(
        "Use descriptive, meaningful names",
        "Используйте описательные, значимые имена",
        "설명적이고 의미 있는 이름 사용"
    ),
    category:     PracticeCategory::Naming,
    explanation:  LocalizedText::new(
        "\
Names must reflect purpose. Avoid generic terms like 'create', 'handle', 'data'.
Descriptive names reduce ambiguity, facilitate easier onboarding, and improve
maintainability.

Conventions:
- snake_case for variables and functions
- PascalCase for structs and enums
- SCREAMING_SNAKE_CASE for constants",
        "\
Имена должны отражать назначение. Избегайте общих терминов вроде 'create', 'handle', 'data'.
Описательные имена уменьшают неоднозначность и улучшают поддерживаемость.",
        "\
이름은 목적을 반영해야 합니다. 'create', 'handle', 'data' 같은 일반적인 용어를 피하세요."
    ),
    good_example: r#"fn create_user_handler(req: CreateUserRequest) -> Result<User>
const MAX_RETRY_ATTEMPTS: u32 = 3;
struct UserAuthenticationService { ... }"#,
    bad_example:  r#"fn create(r: Request) -> Result<Data>
const MAX: u32 = 3;
struct Service { ... }"#,
    source:       "https://github.com/RAprogramm/RustManifest#2-naming-conventions"
};
