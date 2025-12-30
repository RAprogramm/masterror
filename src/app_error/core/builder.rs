// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::{borrow::Cow, string::String, sync::Arc};
use core::error::Error as CoreError;
#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

#[cfg(feature = "serde_json")]
use serde::Serialize;
#[cfg(feature = "serde_json")]
use serde_json::{Value as JsonValue, to_value};

use super::{
    error::Error,
    types::{CapturedBacktrace, ContextAttachment, MessageEditPolicy}
};
use crate::{
    AppCode, AppErrorKind, RetryAdvice,
    app_error::metadata::{Field, FieldRedaction, Metadata}
};

impl Error {
    /// Create a new [`Error`] with a kind and message.
    ///
    /// This is equivalent to [`Error::with`], provided for API symmetry and to
    /// keep doctests readable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid payload");
    /// assert!(err.message.is_some());
    /// ```
    #[must_use]
    pub fn new(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(kind, msg)
    }

    /// Create an error with the given kind and message.
    ///
    /// Prefer named helpers (e.g. [`Error::not_found`]) where it clarifies
    /// intent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::with(AppErrorKind::Validation, "bad input");
    /// assert_eq!(err.kind, AppErrorKind::Validation);
    /// ```
    #[must_use]
    pub fn with(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        let err = Self::new_raw(kind, Some(msg.into()));
        err.emit_telemetry();
        err
    }

    /// Create a message-less error with the given kind.
    ///
    /// Useful when the kind alone conveys sufficient information to the client.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::bare(AppErrorKind::NotFound);
    /// assert!(err.message.is_none());
    /// ```
    #[must_use]
    pub fn bare(kind: AppErrorKind) -> Self {
        let err = Self::new_raw(kind, None);
        err.emit_telemetry();
        err
    }

    /// Override the machine-readable [`AppCode`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppCode, AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "test").with_code(AppCode::NotFound);
    /// assert_eq!(err.code, AppCode::NotFound);
    /// ```
    #[must_use]
    pub fn with_code(mut self, code: AppCode) -> Self {
        self.code = code;
        self.mark_dirty();
        self
    }

    /// Attach retry advice to the error.
    ///
    /// When mapped to HTTP, this becomes the `Retry-After` header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::RateLimited, "slow down").with_retry_after_secs(60);
    /// assert_eq!(err.retry.map(|r| r.after_seconds), Some(60));
    /// ```
    #[must_use]
    pub fn with_retry_after_secs(mut self, secs: u64) -> Self {
        self.retry = Some(RetryAdvice {
            after_seconds: secs
        });
        self.mark_dirty();
        self
    }

    /// Attach a `WWW-Authenticate` challenge string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::Unauthorized, "auth required")
    ///     .with_www_authenticate("Bearer realm=\"api\"");
    /// assert!(err.www_authenticate.is_some());
    /// ```
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self.mark_dirty();
        self
    }

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

    /// Mark the message as redactable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind, MessageEditPolicy};
    ///
    /// let err = AppError::new(AppErrorKind::Internal, "secret").redactable();
    /// assert_eq!(err.edit_policy, MessageEditPolicy::Redact);
    /// ```
    #[must_use]
    pub fn redactable(mut self) -> Self {
        self.edit_policy = MessageEditPolicy::Redact;
        self.mark_dirty();
        self
    }

    /// Attach upstream diagnostics using [`with_source`](Self::with_source) or
    /// an existing [`Arc`].
    ///
    /// This is the preferred alias for capturing upstream errors. It accepts
    /// either an owned error implementing [`core::error::Error`] or a
    /// shared [`Arc`] produced by other APIs, reusing the allocation when
    /// possible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::AppError;
    ///
    /// let err = AppError::service("downstream degraded")
    ///     .with_context(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
    /// assert!(err.source_ref().is_some());
    /// # }
    /// ```
    #[must_use]
    pub fn with_context(self, context: impl Into<ContextAttachment>) -> Self {
        match context.into() {
            ContextAttachment::Owned(source) => {
                match source.downcast::<Arc<dyn CoreError + Send + Sync + 'static>>() {
                    Ok(shared) => self.with_source_arc(*shared),
                    Err(source) => self.with_source_arc(Arc::from(source))
                }
            }
            ContextAttachment::Shared(source) => self.with_source_arc(source)
        }
    }

    /// Attach a source error for diagnostics.
    ///
    /// Prefer [`with_context`](Self::with_context) when capturing upstream
    /// diagnostics without additional `Arc` allocations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let io_err = std::io::Error::new(std::io::ErrorKind::Other, "boom");
    /// let err = AppError::internal("boom").with_source(io_err);
    /// assert!(err.source_ref().is_some());
    /// # }
    /// ```
    #[must_use]
    pub fn with_source(mut self, source: impl CoreError + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self.mark_dirty();
        self
    }

    /// Attach a shared source error without cloning the underlying `Arc`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::sync::Arc;
    ///
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let source = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
    /// let err = AppError::internal("boom").with_source_arc(source.clone());
    /// assert!(err.source_ref().is_some());
    /// assert_eq!(Arc::strong_count(&source), 2);
    /// # }
    /// ```
    #[must_use]
    pub fn with_source_arc(mut self, source: Arc<dyn CoreError + Send + Sync + 'static>) -> Self {
        self.source = Some(source);
        self.mark_dirty();
        self
    }

    /// Attach a captured backtrace.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "backtrace")]
    /// # {
    /// use std::backtrace::Backtrace;
    ///
    /// use masterror::AppError;
    ///
    /// let bt = Backtrace::capture();
    /// let err = AppError::internal("test").with_backtrace(bt);
    /// # }
    /// ```
    #[must_use]
    pub fn with_backtrace(mut self, backtrace: CapturedBacktrace) -> Self {
        #[cfg(feature = "backtrace")]
        {
            self.set_backtrace_slot(Arc::new(backtrace));
        }
        #[cfg(not(feature = "backtrace"))]
        {
            self.set_backtrace_slot(backtrace);
        }
        self.mark_dirty();
        self
    }

    /// Attach a shared backtrace without cloning.
    ///
    /// Internal method for sharing backtraces between errors.
    #[cfg(feature = "backtrace")]
    pub(crate) fn with_shared_backtrace(mut self, backtrace: Arc<Backtrace>) -> Self {
        self.set_backtrace_slot(backtrace);
        self.mark_dirty();
        self
    }

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
        let details = to_value(payload).map_err(|err| Self::bad_request(err.to_string()))?;
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
