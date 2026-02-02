// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::{borrow::Cow, boxed::Box};
use core::error::Error as CoreError;

#[cfg(feature = "backtrace")]
use {alloc::sync::Arc, std::backtrace::Backtrace};

#[cfg(feature = "backtrace")]
use super::backtrace::capture_backtrace_snapshot;
use super::{
    error::Error,
    types::{CapturedBacktrace, ErrorChain}
};
use crate::app_error::metadata::Metadata;

impl Error {
    /// Borrow the attached metadata.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, field};
    ///
    /// let err = AppError::internal("test").with_field(field::str("key", "value"));
    /// let metadata = err.metadata();
    /// assert!(!metadata.is_empty());
    /// ```
    #[must_use]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Borrow the backtrace, capturing it lazily when the `backtrace` feature
    /// is enabled.
    ///
    /// If a backtrace was previously attached via `with_backtrace()`, returns
    /// that. Otherwise, lazily captures a new backtrace based on
    /// `RUST_BACKTRACE` configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "backtrace")]
    /// # {
    /// use masterror::AppError;
    ///
    /// let err = AppError::internal("test");
    /// let bt = err.backtrace();
    /// # }
    /// ```
    #[must_use]
    pub fn backtrace(&self) -> Option<&CapturedBacktrace> {
        self.capture_backtrace()
    }

    /// Returns a shared Arc reference to the backtrace.
    ///
    /// Internal method for efficient backtrace sharing between errors.
    #[cfg(feature = "backtrace")]
    pub(crate) fn backtrace_shared(&self) -> Option<Arc<Backtrace>> {
        if let Some(backtrace) = self.backtrace.as_ref() {
            return Some(Arc::clone(backtrace));
        }
        self.captured_backtrace
            .get_or_init(capture_backtrace_snapshot)
            .as_ref()
            .map(Arc::clone)
    }

    /// Returns a filtered, human-readable backtrace string.
    ///
    /// Filters out internal masterror frames and standard library runtime
    /// frames, showing only application code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "backtrace")]
    /// # {
    /// use masterror::AppError;
    ///
    /// let err = AppError::internal("test");
    /// if let Some(bt) = err.backtrace_short() {
    ///     println!("{}", bt);
    /// }
    /// # }
    /// ```
    #[cfg(feature = "backtrace")]
    #[must_use]
    pub fn backtrace_short(&self) -> Option<alloc::string::String> {
        use alloc::{string::String, vec::Vec};

        let bt = self.backtrace()?;
        let bt_str = alloc::format!("{}", bt);

        let mut frames: Vec<(String, String)> = Vec::new();
        let mut current_fn: Option<String> = None;

        for line in bt_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Skip internal frames
            if Self::is_internal_frame(trimmed) {
                current_fn = None;
                continue;
            }

            // Function line: "   4: crate::module::function"
            if let Some(fn_name) = Self::parse_function_line(trimmed) {
                current_fn = Some(fn_name);
            }
            // Location line: "             at ./src/main.rs:16:13"
            else if let Some(location) = Self::parse_location_line(trimmed)
                && let Some(fn_name) = current_fn.take()
            {
                frames.push((fn_name, location));
            }
        }

        if frames.is_empty() {
            return None;
        }

        let result: Vec<String> = frames
            .into_iter()
            .map(|(func, loc)| alloc::format!("→ {} at {}", func, loc))
            .collect();

        Some(result.join("\n"))
    }

    #[cfg(feature = "backtrace")]
    fn is_internal_frame(line: &str) -> bool {
        line.contains("masterror::")
            || line.contains("/masterror/src/")
            || line.contains("std::rt::")
            || line.contains("std::panic")
            || line.contains("core::ops::function")
            || line.contains("core::panicking")
            || line.contains("std::sys::backtrace")
            || line.contains("std::backtrace")
            || line.contains("__libc_")
            || line.contains("_start")
            || line.contains("<unknown>")
            || line.contains("/rustc/")
            || line.contains("/rustlib/src/rust/library/")
            || line.ends_with(": main")
            || line == "main"
    }

    #[cfg(feature = "backtrace")]
    fn parse_function_line(line: &str) -> Option<alloc::string::String> {
        // Format: "   4: crate::module::function" or "   4: function"
        let colon_pos = line.find(':')?;
        let prefix = &line[..colon_pos];
        if !prefix.trim().chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let fn_full = line[colon_pos + 1..].trim();
        if fn_full.is_empty() {
            return None;
        }
        // Extract just the function name (last segment after ::)
        let fn_name = fn_full.rsplit("::").next().unwrap_or(fn_full);
        Some(fn_name.into())
    }

    #[cfg(feature = "backtrace")]
    fn parse_location_line(line: &str) -> Option<alloc::string::String> {
        // Format: "             at ./src/main.rs:16:13"
        let at_pos = line.find("at ")?;
        let location = line[at_pos + 3..].trim();
        // Simplify path: remove ./ prefix
        let location = location.strip_prefix("./").unwrap_or(location);
        // Remove column number (keep only file:line)
        if let Some((file_line, _col)) = location.rsplit_once(':')
            && file_line.rsplit_once(':').is_some()
        {
            // file:line format
            return Some(file_line.into());
        }
        Some(location.into())
    }

    /// Borrow the source if present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::AppError;
    ///
    /// let io_err = std::io::Error::new(std::io::ErrorKind::Other, "boom");
    /// let err = AppError::internal("failed").with_context(io_err);
    /// assert!(err.source_ref().is_some());
    /// # }
    /// ```
    #[must_use]
    pub fn source_ref(&self) -> Option<&(dyn CoreError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Human-readable message or the kind fallback.
    ///
    /// Returns the error message if set, otherwise returns the error kind's
    /// default label.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    ///
    /// let err = AppError::new(AppErrorKind::BadRequest, "custom message");
    /// assert_eq!(err.render_message(), "custom message");
    ///
    /// let bare_err = AppError::bare(AppErrorKind::NotFound);
    /// assert!(!bare_err.render_message().is_empty());
    /// ```
    #[must_use]
    pub fn render_message(&self) -> Cow<'_, str> {
        match &self.message {
            Some(msg) => Cow::Borrowed(msg.as_ref()),
            None => Cow::Borrowed(self.kind.label())
        }
    }

    /// Emit telemetry (`tracing` event, metrics counter, backtrace capture).
    ///
    /// Downstream code can call this to guarantee telemetry after mutating the
    /// error. It is automatically invoked by constructors and conversions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::internal("test");
    /// err.log();
    /// ```
    pub fn log(&self) {
        self.emit_telemetry();
    }

    /// Returns an iterator over the error chain, starting with this error.
    ///
    /// The iterator yields references to each error in the source chain,
    /// walking through [`source()`](CoreError::source) until reaching the
    /// root cause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::io::Error as IoError;
    ///
    /// use masterror::AppError;
    ///
    /// let io_err = IoError::other("disk offline");
    /// let app_err = AppError::internal("db down").with_context(io_err);
    ///
    /// let chain: Vec<_> = app_err.chain().collect();
    /// assert_eq!(chain.len(), 2);
    /// # }
    /// ```
    #[must_use]
    pub fn chain(&self) -> ErrorChain<'_> {
        ErrorChain {
            current: Some(self as &(dyn CoreError + 'static))
        }
    }

    /// Returns the lowest-level source error in the chain.
    ///
    /// This traverses the error source chain until it finds an error with no
    /// further source, then returns a reference to it. If this error has no
    /// source, it returns a reference to itself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::io::Error as IoError;
    ///
    /// use masterror::AppError;
    ///
    /// let io_err = IoError::other("disk offline");
    /// let app_err = AppError::internal("db down").with_context(io_err);
    ///
    /// let root = app_err.root_cause();
    /// assert_eq!(root.to_string(), "disk offline");
    /// # }
    /// ```
    #[must_use]
    pub fn root_cause(&self) -> &(dyn CoreError + 'static) {
        self.chain()
            .last()
            .expect("chain always has at least one error")
    }

    /// Attempts to downcast the error source to a concrete type.
    ///
    /// Returns `true` if the error source is of type `E`, `false` otherwise.
    /// This only checks the immediate source, not the entire chain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::io::Error as IoError;
    ///
    /// use masterror::AppError;
    ///
    /// let io_err = IoError::other("disk offline");
    /// let app_err = AppError::internal("db down").with_context(io_err);
    ///
    /// assert!(app_err.is::<IoError>());
    ///
    /// let err_without_source = AppError::not_found("missing");
    /// assert!(!err_without_source.is::<IoError>());
    /// # }
    /// ```
    #[must_use]
    pub fn is<E>(&self) -> bool
    where
        E: CoreError + 'static
    {
        self.source_ref().is_some_and(|source| source.is::<E>())
    }

    /// Attempt to downcast the error source to a concrete type by value.
    ///
    /// **Note:** This method is currently a stub and always returns
    /// `Err(Self)`.
    ///
    /// Use [`downcast_ref`](Self::downcast_ref) for inspecting error sources.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::io::Error as IoError;
    ///
    /// use masterror::AppError;
    ///
    /// let io_err = IoError::other("disk offline");
    /// let err = AppError::internal("boom").with_context(io_err);
    ///
    /// assert!(err.downcast::<IoError>().is_err());
    /// # }
    /// ```
    pub fn downcast<E>(self) -> Result<Box<E>, Self>
    where
        E: CoreError + 'static
    {
        Err(self)
    }

    /// Attempt to downcast the error to a concrete type by immutable
    /// reference.
    ///
    /// Returns `Some(&E)` if this error is of type `E`, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::io::Error as IoError;
    ///
    /// use masterror::AppError;
    ///
    /// let io_err = IoError::other("disk offline");
    /// let err = AppError::internal("boom").with_context(io_err);
    ///
    /// if let Some(io) = err.downcast_ref::<IoError>() {
    ///     assert_eq!(io.to_string(), "disk offline");
    /// }
    /// # }
    /// ```
    #[must_use]
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: CoreError + 'static
    {
        self.source_ref()?.downcast_ref::<E>()
    }

    /// Attempt to downcast the error to a concrete type by mutable reference.
    ///
    /// Returns `Some(&mut E)` if this error is of type `E`, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use std::io::Error as IoError;
    ///
    /// use masterror::AppError;
    ///
    /// let io_err = IoError::other("disk offline");
    /// let mut err = AppError::internal("boom").with_context(io_err);
    ///
    /// if let Some(_io) = err.downcast_mut::<IoError>() {
    ///     // Can modify the IoError if needed
    /// }
    /// # }
    /// ```
    #[must_use]
    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: CoreError + 'static
    {
        None
    }
}
