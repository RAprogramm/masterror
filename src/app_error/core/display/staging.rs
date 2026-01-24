// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Staging mode formatting (JSON with context).

use alloc::string::ToString;
use core::{
    error::Error as CoreError,
    fmt::{Formatter, Result as FmtResult}
};

use super::helpers::{write_json_escaped, write_metadata_value};
use crate::{FieldRedaction, MessageEditPolicy, app_error::core::error::Error};

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
        write!(f, r#"{{"kind":"{:?}","code":"{}""#, self.kind, self.code)?;
        if !matches!(self.edit_policy, MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            write!(f, ",\"message\":\"")?;
            write_json_escaped(f, msg.as_ref())?;
            write!(f, "\"")?;
        }
        self.fmt_staging_source_chain(f)?;
        self.fmt_staging_metadata(f)?;
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

    fn fmt_staging_metadata(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.metadata.is_empty() {
            let has_public_fields = self
                .metadata
                .iter_with_redaction()
                .any(|(_, _, redaction)| !matches!(redaction, FieldRedaction::Redact));
            if has_public_fields {
                write!(f, r#","metadata":{{"#)?;
                let mut first = true;
                for (name, value, redaction) in self.metadata.iter_with_redaction() {
                    if matches!(redaction, FieldRedaction::Redact) {
                        continue;
                    }
                    if !first {
                        write!(f, ",")?;
                    }
                    first = false;
                    write!(f, r#""{}":"#, name)?;
                    write_metadata_value(f, value)?;
                }
                write!(f, "}}")?;
            }
        }
        Ok(())
    }
}
