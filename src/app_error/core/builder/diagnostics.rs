// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Diagnostics builder methods for `AppError`.
//!
//! Provides methods to attach hints, suggestions, documentation links,
//! and related error codes to errors.

use alloc::{borrow::Cow, boxed::Box};

use crate::app_error::{
    core::error::Error,
    diagnostics::{DiagnosticVisibility, Diagnostics, DocLink, Hint, Suggestion}
};

impl Error {
    /// Adds a development-only hint to explain the error.
    ///
    /// Hints provide context about why an error occurred without necessarily
    /// offering a solution. They are only shown in Local (development) mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::not_found("User not found")
    ///     .with_hint("Check if the user ID is correct")
    ///     .with_hint("User might have been deleted");
    /// ```
    #[must_use]
    pub fn with_hint(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.ensure_diagnostics().hints.push(Hint::new(message));
        self.mark_dirty();
        self
    }

    /// Adds a hint with custom visibility.
    ///
    /// Use this when a hint should be visible in Staging or Production.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, DiagnosticVisibility};
    ///
    /// let err = AppError::unauthorized("Invalid token").with_hint_visible(
    ///     "Token may have expired, please login again",
    ///     DiagnosticVisibility::Public
    /// );
    /// ```
    #[must_use]
    pub fn with_hint_visible(
        mut self,
        message: impl Into<Cow<'static, str>>,
        visibility: DiagnosticVisibility
    ) -> Self {
        self.ensure_diagnostics()
            .hints
            .push(Hint::with_visibility(message, visibility));
        self.mark_dirty();
        self
    }

    /// Adds an actionable suggestion to fix the error.
    ///
    /// Suggestions provide concrete steps users can take to resolve an error.
    /// They are only shown in Local (development) mode by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::database_with_message("Connection failed")
    ///     .with_suggestion("Check if the database server is running");
    /// ```
    #[must_use]
    pub fn with_suggestion(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.ensure_diagnostics()
            .suggestions
            .push(Suggestion::new(message));
        self.mark_dirty();
        self
    }

    /// Adds a suggestion with an executable command or code snippet.
    ///
    /// The command is displayed in a distinct style to indicate it can be
    /// copied and executed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::database_with_message("PostgreSQL connection refused")
    ///     .with_suggestion_cmd(
    ///         "Check if PostgreSQL is running",
    ///         "systemctl status postgresql"
    ///     );
    /// ```
    #[must_use]
    pub fn with_suggestion_cmd(
        mut self,
        message: impl Into<Cow<'static, str>>,
        command: impl Into<Cow<'static, str>>
    ) -> Self {
        self.ensure_diagnostics()
            .suggestions
            .push(Suggestion::with_command(message, command));
        self.mark_dirty();
        self
    }

    /// Links to documentation explaining this error.
    ///
    /// Documentation links are publicly visible by default, helping end users
    /// understand and resolve errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::not_found("User not found")
    ///     .with_docs("https://docs.example.com/errors/USER_NOT_FOUND");
    /// ```
    #[must_use]
    pub fn with_docs(mut self, url: impl Into<Cow<'static, str>>) -> Self {
        self.ensure_diagnostics().doc_link = Some(DocLink::new(url));
        self.mark_dirty();
        self
    }

    /// Links to documentation with a human-readable title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::unauthorized("Token expired").with_docs_titled(
    ///     "https://docs.example.com/auth/tokens",
    ///     "Authentication Guide"
    /// );
    /// ```
    #[must_use]
    pub fn with_docs_titled(
        mut self,
        url: impl Into<Cow<'static, str>>,
        title: impl Into<Cow<'static, str>>
    ) -> Self {
        self.ensure_diagnostics().doc_link = Some(DocLink::with_title(url, title));
        self.mark_dirty();
        self
    }

    /// Adds a related error code for cross-reference.
    ///
    /// Related codes help users discover errors that might provide additional
    /// context or alternative explanations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::database_with_message("Connection timeout")
    ///     .with_related_code("DB_POOL_EXHAUSTED")
    ///     .with_related_code("DB_AUTH_FAILED");
    /// ```
    #[must_use]
    pub fn with_related_code(mut self, code: impl Into<Cow<'static, str>>) -> Self {
        self.ensure_diagnostics().related_codes.push(code.into());
        self.mark_dirty();
        self
    }

    /// Returns a mutable reference to diagnostics, initializing if needed.
    fn ensure_diagnostics(&mut self) -> &mut Diagnostics {
        if self.inner.diagnostics.is_none() {
            self.inner.diagnostics = Some(Box::new(Diagnostics::new()));
        }
        self.inner
            .diagnostics
            .as_mut()
            .expect("diagnostics initialized above")
    }

    /// Returns diagnostics if present.
    #[must_use]
    pub fn diagnostics(&self) -> Option<&Diagnostics> {
        self.inner.diagnostics.as_deref()
    }
}
