// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA006: Entity naming - no -er suffixes

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA006",
    title:        LocalizedText::new(
        "Entity naming: avoid -er, -or, -manager suffixes",
        "Именование сущностей: избегайте суффиксов -er, -or, -manager",
        "엔티티 명명: -er, -or, -manager 접미사 피하기"
    ),
    category:     PracticeCategory::Naming,
    explanation:  LocalizedText::new(
        "\
Structures represent entities, not actions. The -er suffix encourages procedural
thinking that separates data from behavior, creating anemic domain models.
Entity naming naturally unifies data and operations.

Transforms:
- ConfigLoader → Config
- MessageParser → Message
- RequestHandler → Request
- DataValidator → Data
- ConnectionManager → ConnectionPool

Exceptions: Iterator, Builder, Visitor, Formatter (established patterns).",
        "\
Структуры представляют сущности, не действия. Суффикс -er поощряет процедурное
мышление, разделяющее данные и поведение. Именование сущностей объединяет их.",
        "\
구조체는 동작이 아닌 엔티티를 나타냅니다. -er 접미사는 절차적 사고를 장려합니다."
    ),
    good_example: r#"struct Config { ... }
struct Message { ... }
struct Request { ... }
struct ConnectionPool { ... }"#,
    bad_example:  r#"struct ConfigLoader { ... }
struct MessageParser { ... }
struct RequestHandler { ... }
struct ConnectionManager { ... }"#,
    source:       "https://github.com/RAprogramm/RustManifest/blob/main/STRUCTURE.md#1-entity-naming"
};
