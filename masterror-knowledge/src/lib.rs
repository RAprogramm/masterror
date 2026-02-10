// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Knowledge base for Rust compiler errors and best practices.
//!
//! This crate provides structured explanations for Rust compiler errors
//! with translations (en/ru/ko) and actionable fix suggestions.
//!
//! # Example
//!
//! ```rust,ignore
//! use masterror_knowledge::{ErrorRegistry, PracticeRegistry};
//!
//! let registry = ErrorRegistry::new();
//! if let Some(entry) = registry.find("E0502") {
//!     println!("Error: {}", entry.title.en);
//!     println!("Explanation: {}", entry.explanation.en);
//! }
//!
//! let practices = PracticeRegistry::new();
//! if let Some(practice) = practices.find("RA001") {
//!     println!("Practice: {}", practice.title.en);
//! }
//! ```

pub mod errors;
pub mod i18n;

pub use errors::{
    Category, DocLink, ErrorEntry, ErrorRegistry, FixSuggestion, LocalizedText,
    raprogramm::{BestPractice, PracticeCategory, PracticeRegistry}
};
pub use i18n::{Lang, messages::UiMsg, phrases};
