// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Rust compiler error explanations organized by category.

pub mod borrowing;
pub mod lifetimes;
pub mod ownership;
pub mod raprogramm;
pub mod resolution;
pub mod traits;
pub mod types;

use std::collections::HashMap;

/// Link with title for documentation.
#[derive(Debug, Clone)]
pub struct DocLink {
    pub title: &'static str,
    pub url:   &'static str
}

/// Fix suggestion with code example.
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub description: LocalizedText,
    pub code:        &'static str
}

/// Localized text with translations.
#[derive(Debug, Clone)]
pub struct LocalizedText {
    pub en: &'static str,
    pub ru: &'static str,
    pub ko: &'static str
}

impl LocalizedText {
    pub const fn new(en: &'static str, ru: &'static str, ko: &'static str) -> Self {
        Self {
            en,
            ru,
            ko
        }
    }

    pub fn get(&self, lang: &str) -> &'static str {
        match lang {
            "ru" => self.ru,
            "ko" => self.ko,
            _ => self.en
        }
    }
}

/// Error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Ownership,
    Borrowing,
    Lifetimes,
    Types,
    Traits,
    Resolution
}

impl Category {
    pub fn name(&self, lang: &str) -> &'static str {
        match (self, lang) {
            (Self::Ownership, "ru") => "Владение",
            (Self::Ownership, "ko") => "소유권",
            (Self::Ownership, _) => "Ownership",

            (Self::Borrowing, "ru") => "Заимствование",
            (Self::Borrowing, "ko") => "빌림",
            (Self::Borrowing, _) => "Borrowing",

            (Self::Lifetimes, "ru") => "Времена жизни",
            (Self::Lifetimes, "ko") => "라이프타임",
            (Self::Lifetimes, _) => "Lifetimes",

            (Self::Types, "ru") => "Типы",
            (Self::Types, "ko") => "타입",
            (Self::Types, _) => "Types",

            (Self::Traits, "ru") => "Трейты",
            (Self::Traits, "ko") => "트레이트",
            (Self::Traits, _) => "Traits",

            (Self::Resolution, "ru") => "Разрешение имён",
            (Self::Resolution, "ko") => "이름 확인",
            (Self::Resolution, _) => "Name Resolution"
        }
    }
}

/// Complete error entry.
#[derive(Debug, Clone)]
pub struct ErrorEntry {
    pub code:        &'static str,
    pub title:       LocalizedText,
    pub category:    Category,
    pub explanation: LocalizedText,
    pub fixes:       &'static [FixSuggestion],
    pub links:       &'static [DocLink]
}

/// Registry of all known errors.
pub struct ErrorRegistry {
    errors: HashMap<&'static str, &'static ErrorEntry>
}

impl ErrorRegistry {
    /// Build registry from all modules.
    pub fn new() -> Self {
        let mut errors = HashMap::new();

        // Ownership errors
        for entry in ownership::entries() {
            errors.insert(entry.code, *entry);
        }

        // Borrowing errors
        for entry in borrowing::entries() {
            errors.insert(entry.code, *entry);
        }

        // Lifetime errors
        for entry in lifetimes::entries() {
            errors.insert(entry.code, *entry);
        }

        // Type errors
        for entry in types::entries() {
            errors.insert(entry.code, *entry);
        }

        // Trait errors
        for entry in traits::entries() {
            errors.insert(entry.code, *entry);
        }

        // Resolution errors
        for entry in resolution::entries() {
            errors.insert(entry.code, *entry);
        }

        Self {
            errors
        }
    }

    /// Find error by code.
    pub fn find(&self, code: &str) -> Option<&'static ErrorEntry> {
        let normalized = if code.starts_with('E') || code.starts_with('e') {
            code.to_uppercase()
        } else {
            format!("E{code}")
        };
        self.errors.get(normalized.as_str()).copied()
    }

    /// Get all errors.
    pub fn all(&self) -> impl Iterator<Item = &'static ErrorEntry> + '_ {
        self.errors.values().copied()
    }

    /// Get errors by category.
    pub fn by_category(&self, cat: Category) -> Vec<&'static ErrorEntry> {
        self.errors
            .values()
            .filter(|e| e.category == cat)
            .copied()
            .collect()
    }
}

impl Default for ErrorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
