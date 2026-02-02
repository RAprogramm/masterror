// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Local/development mode formatting (human-readable).

use alloc::string::ToString;
use core::{
    error::Error as CoreError,
    fmt::{Formatter, Result as FmtResult}
};

use crate::app_error::core::error::Error;
// Internal styling - uses colored when available, identity otherwise
#[cfg(feature = "colored")]
use crate::colored::style;

#[cfg(not(feature = "colored"))]
#[allow(dead_code)]
mod style {
    use alloc::string::{String, ToString};

    macro_rules! identity_style {
        ($($name:ident),* $(,)?) => {
            $(
                #[inline]
                pub fn $name(text: impl AsRef<str>) -> String {
                    text.as_ref().to_string()
                }
            )*
        };
    }

    identity_style! {
        error_code,
        error_message,
        source_context,
        metadata_key,
        hint_label,
        hint_text,
        suggestion_label,
        suggestion_text,
        command,
        docs_label,
        url,
        related_label,
        backtrace_label,
        backtrace_arrow,
        backtrace_function,
        backtrace_location,
    }

    #[inline]
    pub fn backtrace_link(display: &str, _absolute_path: &str, _line: Option<u32>) -> String {
        display.to_string()
    }
}

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
        writeln!(f, "{}", self.kind)?;
        writeln!(f, "Code: {}", style::error_code(self.code.to_string()))?;
        if let Some(msg) = &self.message {
            writeln!(f, "Message: {}", style::error_message(msg))?;
        }
        self.fmt_source_chain(f)?;
        self.fmt_metadata(f)?;
        self.fmt_diagnostics(f)?;
        self.fmt_backtrace(f)?;
        Ok(())
    }

    #[cfg(feature = "backtrace")]
    fn fmt_backtrace(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(bt) = self.backtrace_short() {
            writeln!(f)?;
            writeln!(f, "{}:", style::backtrace_label("Backtrace"))?;

            #[cfg(feature = "std")]
            let cwd = std::env::current_dir().ok();
            #[cfg(not(feature = "std"))]
            let cwd: Option<std::path::PathBuf> = None;

            for line in bt.lines() {
                // Parse "→ function at file:line" format
                if let Some(rest) = line.strip_prefix("→ ")
                    && let Some((func, loc)) = rest.split_once(" at ")
                {
                    // Parse file:line from location
                    let (file, line_num) = if let Some((f, l)) = loc.rsplit_once(':') {
                        (f, l.parse::<u32>().ok())
                    } else {
                        (loc, None)
                    };

                    // Resolve to absolute path
                    #[cfg(feature = "std")]
                    let location_styled = if let Some(ref cwd) = cwd {
                        let abs_path = cwd.join(file);
                        let abs_str = abs_path.to_string_lossy();
                        style::backtrace_link(loc, &abs_str, line_num)
                    } else {
                        style::backtrace_location(loc)
                    };
                    #[cfg(not(feature = "std"))]
                    let location_styled = style::backtrace_location(loc);

                    writeln!(
                        f,
                        "  {} {} {} {}",
                        style::backtrace_arrow("→"),
                        style::backtrace_function(func),
                        style::backtrace_label("at"),
                        location_styled
                    )?;
                    continue;
                }
                writeln!(f, "  {}", line)?;
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "backtrace"))]
    fn fmt_backtrace(&self, _f: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }

    fn fmt_source_chain(&self, f: &mut Formatter<'_>) -> FmtResult {
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

    fn fmt_metadata(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.metadata.is_empty() {
            writeln!(f)?;
            writeln!(f, "Context:")?;
            for (key, value) in self.metadata.iter() {
                writeln!(f, "  {}: {}", style::metadata_key(key), value)?;
            }
        }
        Ok(())
    }

    fn fmt_diagnostics(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::app_error::diagnostics::DiagnosticVisibility;

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
}
