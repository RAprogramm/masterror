// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Production mode formatting (compact JSON).

use core::fmt::{Formatter, Result as FmtResult};

use super::helpers::{write_json_escaped, write_metadata_json};
use crate::{MessageEditPolicy, app_error::core::error::Error};

#[allow(dead_code)]
impl Error {
    /// Formats error in production mode (compact JSON).
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
    /// let error = AppError::not_found("User not found");
    /// let output = format!("{}", error);
    /// // In prod mode: {"kind":"NotFound","code":"NOT_FOUND","message":"User not found"}
    /// ```
    #[cfg(not(test))]
    pub(crate) fn fmt_prod(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_prod_impl(f)
    }

    #[cfg(test)]
    #[allow(missing_docs)]
    pub fn fmt_prod(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_prod_impl(f)
    }

    pub(super) fn fmt_prod_impl(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, r#"{{"kind":"{:?}","code":"{}""#, self.kind, self.code)?;
        if !matches!(self.edit_policy, MessageEditPolicy::Redact)
            && let Some(msg) = &self.message
        {
            write!(f, ",\"message\":\"")?;
            write_json_escaped(f, msg.as_ref())?;
            write!(f, "\"")?;
        }
        write_metadata_json(f, &self.metadata)?;
        write!(f, "}}")
    }
}
