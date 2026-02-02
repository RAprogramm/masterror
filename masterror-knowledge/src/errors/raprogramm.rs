// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Best practices from RAprogramm's RustManifest.
//!
//! These are not compiler errors but recommendations and patterns
//! from <https://github.com/RAprogramm/RustManifest>

mod ra001;
mod ra002;
mod ra003;
mod ra004;
mod ra005;
mod ra006;
mod ra007;
mod ra008;
mod ra009;
mod ra010;
mod ra011;
mod ra012;
mod ra013;
mod ra014;
mod ra015;
mod ra016;
mod ra017;
mod ra018;
mod ra019;
mod ra020;
mod ra021;

use std::{collections::HashMap, sync::LazyLock};

use arrayvec::ArrayString;

pub use crate::errors::LocalizedText;

/// Global practice registry singleton.
static PRACTICE_REGISTRY: LazyLock<PracticeRegistry> = LazyLock::new(PracticeRegistry::build);

/// Best practice category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PracticeCategory {
    ErrorHandling,
    Performance,
    Naming,
    Documentation,
    Design,
    Testing,
    Security,
    Safety,
    Memory,
    Idiomatic
}

impl PracticeCategory {
    pub fn name(&self, lang: &str) -> &'static str {
        match (self, lang) {
            (Self::ErrorHandling, "ru") => "Обработка ошибок",
            (Self::ErrorHandling, "ko") => "오류 처리",
            (Self::ErrorHandling, _) => "Error Handling",

            (Self::Performance, "ru") => "Производительность",
            (Self::Performance, "ko") => "성능",
            (Self::Performance, _) => "Performance",

            (Self::Naming, "ru") => "Именование",
            (Self::Naming, "ko") => "명명",
            (Self::Naming, _) => "Naming",

            (Self::Documentation, "ru") => "Документация",
            (Self::Documentation, "ko") => "문서화",
            (Self::Documentation, _) => "Documentation",

            (Self::Design, "ru") => "Проектирование",
            (Self::Design, "ko") => "설계",
            (Self::Design, _) => "Design",

            (Self::Testing, "ru") => "Тестирование",
            (Self::Testing, "ko") => "테스트",
            (Self::Testing, _) => "Testing",

            (Self::Security, "ru") => "Безопасность",
            (Self::Security, "ko") => "보안",
            (Self::Security, _) => "Security",

            (Self::Safety, "ru") => "Безопасность памяти",
            (Self::Safety, "ko") => "메모리 안전",
            (Self::Safety, _) => "Memory Safety",

            (Self::Memory, "ru") => "Управление памятью",
            (Self::Memory, "ko") => "메모리 관리",
            (Self::Memory, _) => "Memory Management",

            (Self::Idiomatic, "ru") => "Идиоматичность",
            (Self::Idiomatic, "ko") => "관용적 코드",
            (Self::Idiomatic, _) => "Idiomatic Code"
        }
    }
}

/// A best practice recommendation.
#[derive(Debug, Clone)]
pub struct BestPractice {
    pub code:         &'static str,
    pub title:        LocalizedText,
    pub category:     PracticeCategory,
    pub explanation:  LocalizedText,
    pub good_example: &'static str,
    pub bad_example:  &'static str,
    pub source:       &'static str
}

static ENTRIES: &[&BestPractice] = &[
    &ra001::ENTRY,
    &ra002::ENTRY,
    &ra003::ENTRY,
    &ra004::ENTRY,
    &ra005::ENTRY,
    &ra006::ENTRY,
    &ra007::ENTRY,
    &ra008::ENTRY,
    &ra009::ENTRY,
    &ra010::ENTRY,
    &ra011::ENTRY,
    &ra012::ENTRY,
    &ra013::ENTRY,
    &ra014::ENTRY,
    &ra015::ENTRY,
    &ra016::ENTRY,
    &ra017::ENTRY,
    &ra018::ENTRY,
    &ra019::ENTRY,
    &ra020::ENTRY,
    &ra021::ENTRY
];

pub fn entries() -> &'static [&'static BestPractice] {
    ENTRIES
}

/// Registry for best practices.
pub struct PracticeRegistry {
    practices: HashMap<&'static str, &'static BestPractice>
}

impl PracticeRegistry {
    /// Get the global registry instance.
    pub fn new() -> &'static Self {
        &PRACTICE_REGISTRY
    }

    /// Build registry from all practices.
    fn build() -> Self {
        let mut practices = HashMap::with_capacity(21);
        for entry in entries() {
            practices.insert(entry.code, *entry);
        }
        Self {
            practices
        }
    }

    /// Find practice by code (RA001, etc.).
    ///
    /// Accepts formats: "RA001", "ra001".
    /// Uses stack-allocated buffer to avoid heap allocation.
    pub fn find(&self, code: &str) -> Option<&'static BestPractice> {
        // Fast path: try exact match first
        if let Some(entry) = self.practices.get(code) {
            return Some(*entry);
        }

        // Normalize to uppercase using stack buffer
        let mut buf: ArrayString<8> = ArrayString::new();
        for c in code.chars().take(8) {
            buf.push(c.to_ascii_uppercase());
        }

        self.practices.get(buf.as_str()).copied()
    }

    /// Get all practices.
    pub fn all(&self) -> impl Iterator<Item = &'static BestPractice> + '_ {
        self.practices.values().copied()
    }

    /// Get practices by category.
    pub fn by_category(&self, cat: PracticeCategory) -> Vec<&'static BestPractice> {
        self.practices
            .values()
            .filter(|p| p.category == cat)
            .copied()
            .collect()
    }
}

impl Default for &'static PracticeRegistry {
    fn default() -> Self {
        PracticeRegistry::new()
    }
}
