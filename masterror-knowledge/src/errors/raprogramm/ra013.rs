// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA013: Testing with fakes over mocks

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA013",
    title:        LocalizedText::new(
        "Use fakes over mocks for testing",
        "Используйте fakes вместо mocks для тестирования",
        "테스트에 mock보다 fake 사용"
    ),
    category:     PracticeCategory::Testing,
    explanation:  LocalizedText::new(
        "\
Use simple fake implementations instead of mock libraries. Fakes provide real
behavior; mocks verify call sequences.

| Aspect | Mocks | Fakes |
|--------|-------|-------|
| Coupling | High | Low |
| Maintenance | Breaks on refactoring | Survives changes |
| Behavior | Simulates | Provides real |
| Debugging | Cryptic | Standard |

Mock appropriateness: Verifying external system interactions, ensuring methods
are NOT called, testing strict interaction ordering.",
        "\
Используйте простые fake-реализации вместо mock-библиотек. Fakes обеспечивают
реальное поведение; mocks проверяют последовательности вызовов.",
        "\
mock 라이브러리 대신 간단한 fake 구현을 사용하세요."
    ),
    good_example: r#"struct FakeDatabase {
    users: HashMap<u64, User>,
}

impl FakeDatabase {
    fn new() -> Self { Self { users: HashMap::new() } }
    fn insert(&mut self, user: User) { self.users.insert(user.id, user); }
}

impl Database for FakeDatabase {
    fn find_user(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }
}"#,
    bad_example:  r#"#[test]
fn test_user_service() {
    let mut mock = MockDatabase::new();
    mock.expect_find_user()
        .with(eq(42))
        .times(1)
        .returning(|_| Some(User::default()));
    // Breaks when implementation changes
}"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#9-testing-with-fakes"
};
