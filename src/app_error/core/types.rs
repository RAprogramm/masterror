// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::{boxed::Box, sync::Arc};
use core::error::Error as CoreError;

/// Attachments accepted by
/// [`Error::with_context`](super::error::Error::with_context).
///
/// This enum represents either an owned or shared error attachment that can be
/// used as a source error context. It allows efficient reuse of Arc-wrapped
/// errors without additional allocations.
///
/// # Variants
///
/// * `Owned` - Takes ownership of a boxed error
/// * `Shared` - Shares an existing Arc-wrapped error
#[derive(Debug)]
#[doc(hidden)]
pub enum ContextAttachment {
    Owned(Box<dyn CoreError + Send + Sync + 'static>),
    Shared(Arc<dyn CoreError + Send + Sync + 'static>)
}

impl<E> From<E> for ContextAttachment
where
    E: CoreError + Send + Sync + 'static
{
    fn from(source: E) -> Self {
        Self::Owned(Box::new(source))
    }
}

#[cfg(feature = "std")]
pub type CapturedBacktrace = std::backtrace::Backtrace;

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
#[derive(Debug)]
pub enum CapturedBacktrace {}

/// Controls whether the public message may be redacted before exposure.
///
/// This policy determines if an error message can be modified or hidden when
/// serializing the error for external consumption (e.g., HTTP responses).
///
/// # Examples
///
/// ```rust
/// use masterror::MessageEditPolicy;
///
/// let preserve = MessageEditPolicy::Preserve;
/// let redact = MessageEditPolicy::Redact;
///
/// assert_eq!(MessageEditPolicy::default(), MessageEditPolicy::Preserve);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MessageEditPolicy {
    /// Message must be preserved as-is.
    #[default]
    Preserve,
    /// Message may be redacted or replaced at the transport boundary.
    Redact
}

/// Iterator over an error chain, yielding each error in the source sequence.
///
/// Created by [`Error::chain`](super::error::Error::chain). Walks through the
/// error source chain using [`Error::source()`](CoreError::source) until
/// reaching the root cause.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "std")] {
/// use std::io::Error as IoError;
///
/// use masterror::AppError;
///
/// let io_err = IoError::other("disk error");
/// let app_err = AppError::internal("db failed").with_context(io_err);
///
/// let chain: Vec<_> = app_err.chain().collect();
/// assert_eq!(chain.len(), 2);
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct ErrorChain<'a> {
    pub(super) current: Option<&'a (dyn CoreError + 'static)>
}

impl<'a> Iterator for ErrorChain<'a> {
    type Item = &'a (dyn CoreError + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        self.current = current.source();
        Some(current)
    }
}
