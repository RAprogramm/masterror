// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Source and context attachment methods for `AppError`.

use alloc::sync::Arc;
use core::error::Error as CoreError;
#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use crate::app_error::core::{
    error::Error,
    types::{CapturedBacktrace, ContextAttachment}
};

impl Error {
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
}
