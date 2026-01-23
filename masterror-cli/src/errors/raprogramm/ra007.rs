// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA007: Method naming - nouns for accessors, verbs for mutators

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA007",
    title:        LocalizedText::new(
        "Method naming: nouns for accessors, verbs for mutators",
        "Именование методов: существительные для accessors, глаголы для mutators",
        "메서드 명명: accessor는 명사, mutator는 동사"
    ),
    category:     PracticeCategory::Naming,
    explanation:  LocalizedText::new(
        "\
Method names reflect purpose through grammatical form:
- Accessors (nouns): name(), length(), value() — not get_name()
- Predicates (adjectives): empty(), valid(), published() — not is_empty()
- Mutators (verbs): save(), publish(), delete()

The get_ prefix adds noise without information. Omitting verbs signals pure
accessors. Adjective predicates read more naturally than is_ constructions.",
        "\
Имена методов отражают назначение через грамматическую форму:
- Accessors: name(), length() — не get_name()
- Predicates: empty(), valid() — не is_empty()
- Mutators: save(), publish(), delete()",
        "\
메서드 이름은 문법적 형태로 목적을 반영합니다."
    ),
    good_example: r#"impl User {
    fn name(&self) -> &str { &self.name }
    fn empty(&self) -> bool { self.data.is_empty() }
    fn save(&mut self) { ... }
}"#,
    bad_example:  r#"impl User {
    fn get_name(&self) -> &str { &self.name }
    fn is_empty(&self) -> bool { self.data.is_empty() }
}"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#2-method-naming"
};
