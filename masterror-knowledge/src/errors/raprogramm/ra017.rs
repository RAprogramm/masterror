// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA017: No TODO without issue reference

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA017",
    title:        LocalizedText::new(
        "No TODO without issue reference",
        "TODO только со ссылкой на issue",
        "이슈 참조 없는 TODO 금지"
    ),
    category:     PracticeCategory::Documentation,
    explanation:  LocalizedText::new(
        "\
TODO comments without issue references become forgotten technical debt.
They accumulate over time and lose context about why they were added.

Every TODO should reference a tracked issue (GitHub, Jira, etc.) so it can be
prioritized, assigned, and eventually resolved. Use `todo!()` macro only
in development, never in production code.",
        "\
TODO комментарии без ссылок на issue становятся забытым техническим долгом.
Они накапливаются со временем и теряют контекст о причине добавления.

Каждый TODO должен ссылаться на отслеживаемую задачу (GitHub, Jira и т.д.).
Используйте макрос `todo!()` только в разработке, никогда в продакшене.",
        "\
이슈 참조가 없는 TODO 주석은 잊혀진 기술 부채가 됩니다.
시간이 지남에 따라 축적되고 추가된 이유에 대한 컨텍스트를 잃습니다."
    ),
    good_example: r#"// TODO(#123): Add retry logic for network failures
// FIXME(PROJ-456): Handle edge case when buffer is empty
fn process() {
    // Implementation
}"#,
    bad_example:  r#"fn process() {
    todo!("implement later");
    // TODO: fix this somehow
    // FIXME: doesn't work
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#documentation"
};
