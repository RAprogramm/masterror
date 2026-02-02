// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Staging mode formatting (JSON with context).

use alloc::string::ToString;
use core::{
    error::Error as CoreError,
    fmt::{Formatter, Result as FmtResult}
};

use super::helpers::{write_json_escaped, write_json_header, write_metadata_json};
use crate::app_error::core::error::Error;

#[allow(dead_code)]
impl Error {
    /// Formats error in staging mode (JSON with context).
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
    /// let error = AppError::service("Service unavailable");
    /// let output = format!("{}", error);
    /// // In staging mode: JSON with source_chain and metadata
    /// ```
    #[cfg(not(test))]
    pub(crate) fn fmt_staging(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_staging_impl(f)
    }

    #[cfg(test)]
    #[allow(missing_docs)]
    pub fn fmt_staging(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_staging_impl(f)
    }

    pub(super) fn fmt_staging_impl(&self, f: &mut Formatter<'_>) -> FmtResult {
        write_json_header(
            f,
            &self.kind,
            &self.code,
            self.message.as_deref(),
            self.edit_policy
        )?;
        self.fmt_staging_source_chain(f)?;
        write_metadata_json(f, &self.metadata)?;
        write!(f, "}}")
    }

    fn fmt_staging_source_chain(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(source) = &self.source {
            write!(f, r#","source_chain":["#)?;
            let mut current: &dyn CoreError = source.as_ref();
            let mut depth = 0;
            let mut first = true;
            while depth < 5 {
                if !first {
                    write!(f, ",")?;
                }
                first = false;
                write!(f, "\"")?;
                write_json_escaped(f, &current.to_string())?;
                write!(f, "\"")?;
                if let Some(next) = current.source() {
                    current = next;
                    depth += 1;
                } else {
                    break;
                }
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}
