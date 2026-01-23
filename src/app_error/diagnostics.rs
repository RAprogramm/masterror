// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Diagnostic information for enhanced error reporting.
//!
//! This module provides rich diagnostic capabilities for errors, inspired by
//! compiler diagnostics (rustc, miette) but designed for runtime application
//! errors.
//!
//! # Features
//!
//! - **Hints**: Contextual advice explaining why an error occurred
//! - **Suggestions**: Actionable fixes with optional commands/code snippets
//! - **Documentation links**: URLs to detailed explanations
//! - **Related codes**: Cross-references to related error codes
//! - **Visibility control**: Per-item visibility for dev/staging/prod
//!   environments
//!
//! # Example
//!
//! ```rust
//! use masterror::{AppError, DiagnosticVisibility};
//!
//! let err = AppError::not_found("User not found")
//!     .with_hint("Check if the user ID is correct")
//!     .with_hint("User might have been deleted")
//!     .with_suggestion_cmd("List all users to verify", "curl -X GET /api/users")
//!     .with_docs("https://docs.example.com/errors/USER_NOT_FOUND");
//! ```
//!
//! # Display Modes
//!
//! Diagnostics are filtered based on [`DisplayMode`](crate::DisplayMode):
//!
//! | Visibility | Local | Staging | Prod |
//! |------------|-------|---------|------|
//! | `DevOnly`  | ✅    | ❌      | ❌   |
//! | `Internal` | ✅    | ✅      | ❌   |
//! | `Public`   | ✅    | ✅      | ✅   |

use alloc::borrow::Cow;

use super::inline_vec::InlineVec;

/// Visibility of diagnostic information across environments.
///
/// Controls where hints, suggestions, and other diagnostic information
/// are displayed based on the deployment environment.
///
/// # Ordering
///
/// Variants are ordered from most restrictive to least restrictive:
/// `DevOnly < Internal < Public`. This enables filtering with simple
/// comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum DiagnosticVisibility {
    /// Shown only in Local mode (development).
    ///
    /// Use for internal debugging hints that may expose implementation
    /// details or sensitive information.
    #[default]
    DevOnly = 0,

    /// Shown in Local and Staging environments.
    ///
    /// Use for hints that help with testing and debugging but should
    /// not reach production users.
    Internal = 1,

    /// Shown everywhere including Production.
    ///
    /// Use for user-facing hints and documentation links that help
    /// end users understand and resolve errors.
    Public = 2
}

/// A single hint providing context about an error.
///
/// Hints explain why an error might have occurred without necessarily
/// providing a fix. They help developers and users understand the
/// error context.
///
/// # Example
///
/// ```rust
/// use masterror::diagnostics::{DiagnosticVisibility, Hint};
///
/// let hint = Hint {
///     message:    "Database connection pool may be exhausted".into(),
///     visibility: DiagnosticVisibility::Internal
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hint {
    /// The hint message.
    pub message: Cow<'static, str>,

    /// Where this hint should be displayed.
    pub visibility: DiagnosticVisibility
}

impl Hint {
    /// Creates a new hint with default (DevOnly) visibility.
    #[must_use]
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message:    message.into(),
            visibility: DiagnosticVisibility::DevOnly
        }
    }

    /// Creates a new hint with specified visibility.
    #[must_use]
    pub fn with_visibility(
        message: impl Into<Cow<'static, str>>,
        visibility: DiagnosticVisibility
    ) -> Self {
        Self {
            message: message.into(),
            visibility
        }
    }

    /// Creates a public hint visible in all environments.
    #[must_use]
    pub fn public(message: impl Into<Cow<'static, str>>) -> Self {
        Self::with_visibility(message, DiagnosticVisibility::Public)
    }

    /// Creates an internal hint visible in Local and Staging.
    #[must_use]
    pub fn internal(message: impl Into<Cow<'static, str>>) -> Self {
        Self::with_visibility(message, DiagnosticVisibility::Internal)
    }
}

/// An actionable suggestion to fix an error.
///
/// Suggestions provide concrete steps users can take to resolve an error,
/// optionally including a command or code snippet.
///
/// # Example
///
/// ```rust
/// use masterror::diagnostics::{DiagnosticVisibility, Suggestion};
///
/// let suggestion = Suggestion {
///     message:    "Check if PostgreSQL is running".into(),
///     command:    Some("systemctl status postgresql".into()),
///     visibility: DiagnosticVisibility::DevOnly
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// Human-readable description of the suggestion.
    pub message: Cow<'static, str>,

    /// Optional command or code snippet.
    ///
    /// When present, displayed in a distinct style (monospace, highlighted)
    /// to indicate it can be copied and executed.
    pub command: Option<Cow<'static, str>>,

    /// Where this suggestion should be displayed.
    pub visibility: DiagnosticVisibility
}

impl Suggestion {
    /// Creates a new suggestion without a command.
    #[must_use]
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message:    message.into(),
            command:    None,
            visibility: DiagnosticVisibility::DevOnly
        }
    }

    /// Creates a new suggestion with a command.
    #[must_use]
    pub fn with_command(
        message: impl Into<Cow<'static, str>>,
        command: impl Into<Cow<'static, str>>
    ) -> Self {
        Self {
            message:    message.into(),
            command:    Some(command.into()),
            visibility: DiagnosticVisibility::DevOnly
        }
    }

    /// Sets the visibility for this suggestion.
    #[must_use]
    pub fn visibility(mut self, visibility: DiagnosticVisibility) -> Self {
        self.visibility = visibility;
        self
    }
}

/// A link to documentation explaining the error.
///
/// Documentation links provide detailed explanations and context that
/// don't fit in hint messages. They typically point to error catalogs,
/// API documentation, or troubleshooting guides.
///
/// # Example
///
/// ```rust
/// use masterror::diagnostics::{DiagnosticVisibility, DocLink};
///
/// let doc = DocLink {
///     url:        "https://docs.example.com/errors/E001".into(),
///     title:      Some("Connection Errors".into()),
///     visibility: DiagnosticVisibility::Public
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocLink {
    /// URL to documentation.
    pub url: Cow<'static, str>,

    /// Optional human-readable title for the link.
    pub title: Option<Cow<'static, str>>,

    /// Where this link should be displayed.
    ///
    /// Documentation links are typically `Public` since they help
    /// end users understand errors.
    pub visibility: DiagnosticVisibility
}

impl DocLink {
    /// Creates a new documentation link with public visibility.
    #[must_use]
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            url:        url.into(),
            title:      None,
            visibility: DiagnosticVisibility::Public
        }
    }

    /// Creates a new documentation link with a title.
    #[must_use]
    pub fn with_title(
        url: impl Into<Cow<'static, str>>,
        title: impl Into<Cow<'static, str>>
    ) -> Self {
        Self {
            url:        url.into(),
            title:      Some(title.into()),
            visibility: DiagnosticVisibility::Public
        }
    }

    /// Sets the visibility for this link.
    #[must_use]
    pub fn visibility(mut self, visibility: DiagnosticVisibility) -> Self {
        self.visibility = visibility;
        self
    }
}

/// Complete diagnostic information for an error.
///
/// This structure collects all diagnostic information associated with an
/// error. It is stored in `Option<Box<Diagnostics>>` to ensure zero cost
/// when diagnostics are not used.
///
/// # Memory Layout
///
/// The structure uses [`InlineVec`] for hints and suggestions, which stores
/// up to 4 elements inline without heap allocation. This optimizes for the
/// common case of 1-2 hints/suggestions per error.
///
/// # Example
///
/// ```rust
/// use masterror::diagnostics::{Diagnostics, DocLink, Hint, Suggestion};
///
/// let mut diag = Diagnostics::new();
/// diag.hints.push(Hint::new("Check configuration"));
/// diag.suggestions.push(Suggestion::with_command(
///     "Restart the service",
///     "systemctl restart myapp"
/// ));
/// diag.doc_link = Some(DocLink::new("https://docs.example.com/errors"));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostics {
    /// Contextual hints explaining the error.
    ///
    /// Hints provide context without necessarily offering a solution.
    /// Multiple hints can be attached to explain different aspects.
    pub hints: InlineVec<Hint>,

    /// Actionable suggestions to fix the error.
    ///
    /// Suggestions provide concrete steps to resolve the error.
    /// Usually 0-2 suggestions are most helpful.
    pub suggestions: InlineVec<Suggestion>,

    /// Link to detailed documentation.
    ///
    /// Only one documentation link is supported per error to avoid
    /// overwhelming users with choices.
    pub doc_link: Option<DocLink>,

    /// Related error codes for cross-reference.
    ///
    /// Helps users discover related errors that might provide additional
    /// context or alternative explanations.
    pub related_codes: InlineVec<Cow<'static, str>>
}

impl Diagnostics {
    /// Creates an empty diagnostics container.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hints:         InlineVec::new(),
            suggestions:   InlineVec::new(),
            doc_link:      None,
            related_codes: InlineVec::new()
        }
    }

    /// Returns `true` if no diagnostic information is present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
            && self.suggestions.is_empty()
            && self.doc_link.is_none()
            && self.related_codes.is_empty()
    }

    /// Returns `true` if any diagnostic information has the given visibility
    /// or higher.
    #[must_use]
    pub fn has_visible_content(&self, min_visibility: DiagnosticVisibility) -> bool {
        self.hints.iter().any(|h| h.visibility >= min_visibility)
            || self
                .suggestions
                .iter()
                .any(|s| s.visibility >= min_visibility)
            || self
                .doc_link
                .as_ref()
                .is_some_and(|d| d.visibility >= min_visibility)
    }

    /// Returns an iterator over hints visible at the given level.
    pub fn visible_hints(
        &self,
        min_visibility: DiagnosticVisibility
    ) -> impl Iterator<Item = &Hint> {
        self.hints
            .iter()
            .filter(move |h| h.visibility >= min_visibility)
    }

    /// Returns an iterator over suggestions visible at the given level.
    pub fn visible_suggestions(
        &self,
        min_visibility: DiagnosticVisibility
    ) -> impl Iterator<Item = &Suggestion> {
        self.suggestions
            .iter()
            .filter(move |s| s.visibility >= min_visibility)
    }

    /// Returns the documentation link if visible at the given level.
    #[must_use]
    pub fn visible_doc_link(&self, min_visibility: DiagnosticVisibility) -> Option<&DocLink> {
        self.doc_link
            .as_ref()
            .filter(|d| d.visibility >= min_visibility)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_ordering() {
        assert!(DiagnosticVisibility::DevOnly < DiagnosticVisibility::Internal);
        assert!(DiagnosticVisibility::Internal < DiagnosticVisibility::Public);
    }

    #[test]
    fn hint_constructors() {
        let hint = Hint::new("test");
        assert_eq!(hint.visibility, DiagnosticVisibility::DevOnly);

        let public = Hint::public("test");
        assert_eq!(public.visibility, DiagnosticVisibility::Public);

        let internal = Hint::internal("test");
        assert_eq!(internal.visibility, DiagnosticVisibility::Internal);
    }

    #[test]
    fn suggestion_constructors() {
        let suggestion = Suggestion::new("do this");
        assert!(suggestion.command.is_none());

        let with_cmd = Suggestion::with_command("do this", "some command");
        assert_eq!(with_cmd.command.as_deref(), Some("some command"));
    }

    #[test]
    fn doc_link_constructors() {
        let link = DocLink::new("https://example.com");
        assert!(link.title.is_none());
        assert_eq!(link.visibility, DiagnosticVisibility::Public);

        let titled = DocLink::with_title("https://example.com", "Example");
        assert_eq!(titled.title.as_deref(), Some("Example"));
    }

    #[test]
    fn diagnostics_is_empty() {
        let diag = Diagnostics::new();
        assert!(diag.is_empty());

        let mut diag = Diagnostics::new();
        diag.hints.push(Hint::new("test"));
        assert!(!diag.is_empty());
    }

    #[test]
    fn diagnostics_visibility_filtering() {
        let mut diag = Diagnostics::new();
        diag.hints.push(Hint::new("dev hint"));
        diag.hints.push(Hint::internal("internal hint"));
        diag.hints.push(Hint::public("public hint"));

        // DevOnly level sees all
        let dev_hints: Vec<_> = diag.visible_hints(DiagnosticVisibility::DevOnly).collect();
        assert_eq!(dev_hints.len(), 3);

        // Internal level sees internal + public
        let internal_hints: Vec<_> = diag.visible_hints(DiagnosticVisibility::Internal).collect();
        assert_eq!(internal_hints.len(), 2);

        // Public level sees only public
        let public_hints: Vec<_> = diag.visible_hints(DiagnosticVisibility::Public).collect();
        assert_eq!(public_hints.len(), 1);
        assert_eq!(public_hints[0].message, "public hint");
    }

    #[test]
    fn diagnostics_has_visible_content() {
        let mut diag = Diagnostics::new();
        assert!(!diag.has_visible_content(DiagnosticVisibility::DevOnly));

        diag.hints.push(Hint::new("dev only"));
        assert!(diag.has_visible_content(DiagnosticVisibility::DevOnly));
        assert!(!diag.has_visible_content(DiagnosticVisibility::Internal));
        assert!(!diag.has_visible_content(DiagnosticVisibility::Public));

        diag.suggestions
            .push(Suggestion::new("fix it").visibility(DiagnosticVisibility::Public));
        assert!(diag.has_visible_content(DiagnosticVisibility::Public));
    }

    #[test]
    fn diagnostics_doc_link_visibility() {
        let mut diag = Diagnostics::new();
        diag.doc_link = Some(DocLink::new("https://example.com"));

        assert!(
            diag.visible_doc_link(DiagnosticVisibility::Public)
                .is_some()
        );
        assert!(
            diag.visible_doc_link(DiagnosticVisibility::Internal)
                .is_some()
        );
        assert!(
            diag.visible_doc_link(DiagnosticVisibility::DevOnly)
                .is_some()
        );

        // Change to internal visibility
        diag.doc_link =
            Some(DocLink::new("https://example.com").visibility(DiagnosticVisibility::Internal));
        assert!(
            diag.visible_doc_link(DiagnosticVisibility::Internal)
                .is_some()
        );
        assert!(
            diag.visible_doc_link(DiagnosticVisibility::Public)
                .is_none()
        );
    }

    #[test]
    fn cow_static_str() {
        let hint = Hint::new("static string");
        assert!(matches!(hint.message, Cow::Borrowed(_)));

        let hint = Hint::new(alloc::string::String::from("owned string"));
        assert!(matches!(hint.message, Cow::Owned(_)));
    }
}
