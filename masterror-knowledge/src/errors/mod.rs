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

use std::{collections::HashMap, sync::LazyLock};

use arrayvec::ArrayString;

/// Global error registry singleton.
static ERROR_REGISTRY: LazyLock<ErrorRegistry> = LazyLock::new(ErrorRegistry::build);

/// Link with title for documentation.
#[derive(Debug, Clone, Copy)]
pub struct DocLink {
    /// Link display title.
    pub title: &'static str,
    /// URL to documentation.
    pub url:   &'static str
}

/// Fix suggestion with code example.
#[derive(Debug, Clone, Copy)]
pub struct FixSuggestion {
    /// Description of the fix approach.
    pub description: LocalizedText,
    /// Code example showing the fix.
    pub code:        &'static str
}

/// Localized text with translations.
///
/// All fields are `&'static str` for zero-copy access.
#[derive(Debug, Clone, Copy)]
pub struct LocalizedText {
    /// English text (always present).
    pub en: &'static str,
    /// Russian translation.
    pub ru: &'static str,
    /// Korean translation.
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
///
/// Fields ordered by size (largest first) to minimize padding.
#[derive(Debug, Clone)]
pub struct ErrorEntry {
    /// Error explanation text.
    pub explanation: LocalizedText,
    /// Short error title.
    pub title:       LocalizedText,
    /// Suggested fixes.
    pub fixes:       &'static [FixSuggestion],
    /// Documentation links.
    pub links:       &'static [DocLink],
    /// Error code (E0382).
    pub code:        &'static str,
    /// Error category.
    pub category:    Category
}

/// Registry of all known errors.
pub struct ErrorRegistry {
    errors: HashMap<&'static str, &'static ErrorEntry>
}

impl ErrorRegistry {
    /// Get the global registry instance.
    pub fn new() -> &'static Self {
        &ERROR_REGISTRY
    }

    /// Build registry from all modules.
    fn build() -> Self {
        let mut errors = HashMap::with_capacity(34);

        for entry in ownership::entries() {
            errors.insert(entry.code, *entry);
        }
        for entry in borrowing::entries() {
            errors.insert(entry.code, *entry);
        }
        for entry in lifetimes::entries() {
            errors.insert(entry.code, *entry);
        }
        for entry in types::entries() {
            errors.insert(entry.code, *entry);
        }
        for entry in traits::entries() {
            errors.insert(entry.code, *entry);
        }
        for entry in resolution::entries() {
            errors.insert(entry.code, *entry);
        }

        Self {
            errors
        }
    }

    /// Find error by code.
    ///
    /// Accepts formats: "E0382", "e0382", "0382".
    /// Uses stack-allocated buffer to avoid heap allocation.
    pub fn find(&self, code: &str) -> Option<&'static ErrorEntry> {
        // Fast path: try exact match first (covers "E0382" case)
        if let Some(entry) = self.errors.get(code) {
            return Some(*entry);
        }

        // Normalize to uppercase with 'E' prefix using stack buffer
        let mut buf: ArrayString<8> = ArrayString::new();

        if !code.starts_with('E') && !code.starts_with('e') {
            buf.push('E');
        }

        for c in code.chars().take(7) {
            buf.push(c.to_ascii_uppercase());
        }

        self.errors.get(buf.as_str()).copied()
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

impl Default for &'static ErrorRegistry {
    fn default() -> Self {
        ErrorRegistry::new()
    }
}
