// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Local/development mode formatting (human-readable).

#[cfg(feature = "colored")]
use alloc::string::ToString;
use core::{
    error::Error as CoreError,
    fmt::{Formatter, Result as FmtResult}
};

use crate::app_error::core::error::Error;

#[allow(dead_code)]
impl Error {
    /// Formats error in local/development mode (human-readable).
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write output to
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::AppError;
    ///
    /// let error = AppError::internal("Database error");
    /// let output = format!("{}", error);
    /// // In local mode: multi-line human-readable format with full context
    /// ```
    #[cfg(not(test))]
    pub(crate) fn fmt_local(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_local_impl(f)
    }

    #[cfg(test)]
    #[allow(missing_docs)]
    pub fn fmt_local(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_local_impl(f)
    }

    pub(super) fn fmt_local_impl(&self, f: &mut Formatter<'_>) -> FmtResult {
        #[cfg(feature = "colored")]
        {
            self.fmt_local_colored(f)
        }
        #[cfg(not(feature = "colored"))]
        {
            self.fmt_local_plain(f)
        }
    }

    #[cfg(feature = "colored")]
    fn fmt_local_colored(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::colored::style;

        writeln!(f, "{}", self.kind)?;
        writeln!(f, "Code: {}", style::error_code(self.code.to_string()))?;
        if let Some(msg) = &self.message {
            writeln!(f, "Message: {}", style::error_message(msg))?;
        }
        self.fmt_source_chain_colored(f)?;
        self.fmt_metadata_colored(f)?;
        self.fmt_diagnostics_colored(f)?;
        Ok(())
    }

    #[cfg(feature = "colored")]
    fn fmt_source_chain_colored(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::colored::style;

        if let Some(source) = &self.source {
            writeln!(f)?;
            let mut current: &dyn CoreError = source.as_ref();
            let mut depth = 0;
            while depth < 10 {
                writeln!(
                    f,
                    "  {}: {}",
                    style::source_context("Caused by"),
                    style::source_context(current.to_string())
                )?;
                if let Some(next) = current.source() {
                    current = next;
                    depth += 1;
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "colored")]
    fn fmt_metadata_colored(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::colored::style;

        if !self.metadata.is_empty() {
            writeln!(f)?;
            writeln!(f, "Context:")?;
            for (key, value) in self.metadata.iter() {
                writeln!(f, "  {}: {}", style::metadata_key(key), value)?;
            }
        }
        Ok(())
    }

    #[cfg(feature = "colored")]
    fn fmt_diagnostics_colored(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::{app_error::diagnostics::DiagnosticVisibility, colored::style};

        if let Some(diag) = &self.diagnostics {
            let min_visibility = DiagnosticVisibility::DevOnly;

            // Hints
            let hints: alloc::vec::Vec<_> = diag.visible_hints(min_visibility).collect();
            if !hints.is_empty() {
                writeln!(f)?;
                for hint in hints {
                    writeln!(
                        f,
                        "  {}: {}",
                        style::hint_label("hint"),
                        style::hint_text(&hint.message)
                    )?;
                }
            }

            // Suggestions
            for suggestion in diag.visible_suggestions(min_visibility) {
                writeln!(f)?;
                write!(
                    f,
                    "  {}: {}",
                    style::suggestion_label("suggestion"),
                    style::suggestion_text(&suggestion.message)
                )?;
                if let Some(cmd) = &suggestion.command {
                    writeln!(f)?;
                    writeln!(f, "              {}", style::command(cmd))?;
                } else {
                    writeln!(f)?;
                }
            }

            // Documentation link
            if let Some(doc) = diag.visible_doc_link(min_visibility) {
                writeln!(f)?;
                if let Some(title) = &doc.title {
                    writeln!(
                        f,
                        "  {}: {} ({})",
                        style::docs_label("docs"),
                        title,
                        style::url(&doc.url)
                    )?;
                } else {
                    writeln!(
                        f,
                        "  {}: {}",
                        style::docs_label("docs"),
                        style::url(&doc.url)
                    )?;
                }
            }

            // Related codes
            if !diag.related_codes.is_empty() {
                writeln!(f)?;
                write!(f, "  {}: ", style::related_label("see also"))?;
                for (i, code) in diag.related_codes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", style::error_code(code))?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "colored"))]
    fn fmt_local_plain(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "{}", self.kind)?;
        writeln!(f, "Code: {}", self.code)?;
        if let Some(msg) = &self.message {
            writeln!(f, "Message: {}", msg)?;
        }
        self.fmt_source_chain_plain(f)?;
        self.fmt_metadata_plain(f)?;
        self.fmt_diagnostics_plain(f)?;
        Ok(())
    }

    #[cfg(not(feature = "colored"))]
    fn fmt_source_chain_plain(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(source) = &self.source {
            writeln!(f)?;
            let mut current: &dyn CoreError = source.as_ref();
            let mut depth = 0;
            while depth < 10 {
                writeln!(f, "  Caused by: {}", current)?;
                if let Some(next) = current.source() {
                    current = next;
                    depth += 1;
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "colored"))]
    fn fmt_metadata_plain(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.metadata.is_empty() {
            writeln!(f)?;
            writeln!(f, "Context:")?;
            for (key, value) in self.metadata.iter() {
                writeln!(f, "  {}: {}", key, value)?;
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "colored"))]
    fn fmt_diagnostics_plain(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::app_error::diagnostics::DiagnosticVisibility;

        if let Some(diag) = &self.diagnostics {
            let min_visibility = DiagnosticVisibility::DevOnly;

            // Hints
            let hints: alloc::vec::Vec<_> = diag.visible_hints(min_visibility).collect();
            if !hints.is_empty() {
                writeln!(f)?;
                for hint in hints {
                    writeln!(f, "  hint: {}", hint.message)?;
                }
            }

            // Suggestions
            for suggestion in diag.visible_suggestions(min_visibility) {
                writeln!(f)?;
                write!(f, "  suggestion: {}", suggestion.message)?;
                if let Some(cmd) = &suggestion.command {
                    writeln!(f)?;
                    writeln!(f, "              {}", cmd)?;
                } else {
                    writeln!(f)?;
                }
            }

            // Documentation link
            if let Some(doc) = diag.visible_doc_link(min_visibility) {
                writeln!(f)?;
                if let Some(title) = &doc.title {
                    writeln!(f, "  docs: {} ({})", title, doc.url)?;
                } else {
                    writeln!(f, "  docs: {}", doc.url)?;
                }
            }

            // Related codes
            if !diag.related_codes.is_empty() {
                writeln!(f)?;
                write!(f, "  see also: ")?;
                for (i, code) in diag.related_codes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", code)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
