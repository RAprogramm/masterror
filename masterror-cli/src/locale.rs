// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Localization system for masterror-cli.
#![allow(dead_code)]

mod en;
#[cfg(feature = "lang-ko")]
mod ko;
#[cfg(feature = "lang-ru")]
mod ru;

use std::collections::HashMap;

/// Locale with messages and error translations.
pub struct Locale {
    messages:     HashMap<&'static str, &'static str>,
    translations: HashMap<&'static str, &'static str>,
    lang:         String
}

impl Locale {
    /// Create locale for given language code.
    pub fn new(lang: &str) -> Self {
        let (messages, translations) = match lang {
            #[cfg(feature = "lang-ru")]
            "ru" => (ru::messages(), ru::translations()),
            #[cfg(feature = "lang-ko")]
            "ko" => (ko::messages(), ko::translations()),
            _ => (en::messages(), HashMap::new())
        };
        Self {
            messages,
            translations,
            lang: lang.to_string()
        }
    }

    /// Get localized message by key.
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.messages.get(key).copied().unwrap_or(key)
    }

    /// Get full translated error message by code.
    pub fn translate_error(&self, error_code: &str, _original_msg: &str) -> Option<&'static str> {
        if self.lang == "en" {
            return None;
        }
        // Get full translation by error code from messages
        let key = format!("{}-translation", error_code.to_lowercase());
        self.messages.get(key.as_str()).copied()
    }

    /// Check if translation field should be shown.
    pub fn has_translation(&self) -> bool {
        self.lang != "en"
    }

    /// Get language code.
    pub fn lang(&self) -> &str {
        &self.lang
    }

    /// Translate full rendered compiler output.
    pub fn translate_rendered(&self, rendered: &str) -> String {
        if self.lang == "en" {
            return rendered.to_string();
        }

        let mut result = rendered.to_string();

        // Sort by length descending to replace longer phrases first
        let mut pairs: Vec<_> = self.translations.iter().collect();
        pairs.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        for (en, translated) in pairs {
            result = result.replace(en, translated);
        }

        result
    }
}
