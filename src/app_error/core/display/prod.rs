// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Production mode formatting (compact JSON).

use core::fmt::{Formatter, Result as FmtResult};

use super::helpers::{write_json_header, write_metadata_json};
use crate::app_error::core::error::Error;

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
        write_json_header(
            f,
            &self.kind,
            &self.code,
            self.message.as_deref(),
            self.edit_policy
        )?;
        write_metadata_json(f, &self.metadata)?;
        write!(f, "}}")
    }
}
