// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Metadata attachment methods for `AppError`.

use crate::{
    FieldRedaction,
    app_error::{
        core::error::Error,
        metadata::{Field, Metadata}
    }
};

impl Error {
    /// Attach additional metadata to the error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind, field};
    /// let err = AppError::new(AppErrorKind::Validation, "bad field")
    ///     .with_field(field::str("field_name", "email"));
    /// assert!(err.metadata().get("field_name").is_some());
    /// ```
    #[must_use]
    pub fn with_field(mut self, field: Field) -> Self {
        self.metadata.insert(field);
        self.mark_dirty();
        self
    }

    /// Extend metadata from an iterator of fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind, field};
    /// let fields = vec![field::str("key1", "value1"), field::str("key2", "value2")];
    /// let err = AppError::new(AppErrorKind::BadRequest, "test").with_fields(fields);
    /// assert!(err.metadata().get("key1").is_some());
    /// ```
    #[must_use]
    pub fn with_fields(mut self, fields: impl IntoIterator<Item = Field>) -> Self {
        self.metadata.extend(fields);
        self.mark_dirty();
        self
    }

    /// Override the redaction policy for a stored metadata field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind, FieldRedaction, field};
    ///
    /// let err = AppError::new(AppErrorKind::Internal, "test")
    ///     .with_field(field::str("password", "secret"))
    ///     .redact_field("password", FieldRedaction::Redact);
    /// ```
    #[must_use]
    pub fn redact_field(mut self, name: &'static str, redaction: FieldRedaction) -> Self {
        self.metadata.set_redaction(name, redaction);
        self.mark_dirty();
        self
    }

    /// Replace metadata entirely.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind, Metadata};
    ///
    /// let metadata = Metadata::new();
    /// let err = AppError::new(AppErrorKind::Internal, "test").with_metadata(metadata);
    /// ```
    #[must_use]
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self.mark_dirty();
        self
    }
}
