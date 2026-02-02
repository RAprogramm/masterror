// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Details attachment methods for `AppError`.

#[cfg(not(feature = "serde_json"))]
use alloc::string::String;

#[cfg(feature = "serde_json")]
use serde::Serialize;
#[cfg(feature = "serde_json")]
use serde_json::{Value as JsonValue, to_value};

use crate::app_error::core::error::Error;

impl Error {
    /// Attach structured JSON details for the client payload.
    ///
    /// The details are omitted from responses when the error has been marked as
    /// [`redactable`](Self::redactable).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{AppError, AppErrorKind};
    /// use serde_json::json;
    ///
    /// let err = AppError::new(AppErrorKind::Validation, "invalid input")
    ///     .with_details_json(json!({"field": "email"}));
    /// assert!(err.details.is_some());
    /// # }
    /// ```
    #[must_use]
    #[cfg(feature = "serde_json")]
    pub fn with_details_json(mut self, details: JsonValue) -> Self {
        self.details = Some(details);
        self.mark_dirty();
        self
    }

    /// Serialize and attach structured details.
    ///
    /// Returns [`crate::AppError`] with [`crate::AppErrorKind::BadRequest`] if
    /// serialization fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{AppError, AppErrorKind};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Extra {
    ///     reason: &'static str
    /// }
    ///
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid")
    ///     .with_details(Extra {
    ///         reason: "missing"
    ///     })
    ///     .expect("details should serialize");
    /// assert!(err.details.is_some());
    /// # }
    /// ```
    #[cfg(feature = "serde_json")]
    #[allow(clippy::result_large_err)]
    pub fn with_details<T>(self, payload: T) -> crate::AppResult<Self>
    where
        T: Serialize
    {
        let Ok(details) = to_value(payload) else {
            return Err(Self::bad_request("failed to serialize details"));
        };
        Ok(self.with_details_json(details))
    }

    /// Attach plain-text details for client payloads.
    ///
    /// The text is omitted from responses when the error is
    /// [`redactable`](Self::redactable).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(not(feature = "serde_json"))]
    /// # {
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let err = AppError::new(AppErrorKind::Internal, "boom").with_details_text("retry later");
    /// assert!(err.details.is_some());
    /// # }
    /// ```
    #[must_use]
    #[cfg(not(feature = "serde_json"))]
    pub fn with_details_text(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self.mark_dirty();
        self
    }
}
