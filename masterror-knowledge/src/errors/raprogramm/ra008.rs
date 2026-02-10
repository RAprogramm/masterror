// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA008: Structure size - maximum 4 fields

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA008",
    title:        LocalizedText::new(
        "Structure size: maximum 4 fields",
        "Размер структуры: максимум 4 поля",
        "구조체 크기: 최대 4개 필드"
    ),
    category:     PracticeCategory::Design,
    explanation:  LocalizedText::new(
        "\
A structure should have no more than 4 fields. More fields indicate multiple
responsibilities requiring composition.

Problems with large structures:
- Complex testing with many combinations
- Changes ripple through unrelated code
- Purpose becomes unclear
- Parts cannot be reused independently

Solution: Decompose into focused sub-structures.",
        "\
Структура должна иметь не более 4 полей. Больше полей указывает на
множественные ответственности, требующие композиции.",
        "\
구조체는 4개 이하의 필드를 가져야 합니다. 더 많은 필드는 분해가 필요함을 나타냅니다."
    ),
    good_example: r#"struct User {
    identity: UserIdentity,
    credentials: Credentials,
    profile: UserProfile,
    access: AccessControl,
}"#,
    bad_example:  r#"struct User {
    id: u64, email: String, password_hash: String,
    name: String, avatar: Option<String>, bio: String,
    created_at: DateTime, updated_at: DateTime,
    role: Role, permissions: Vec<Permission>,
    last_login: Option<DateTime>, login_count: u32,
    is_verified: bool, verification_token: Option<String>,
}"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#3-structure-size"
};
