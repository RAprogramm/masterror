// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Browser console error types.
//!
//! This module defines [`BrowserConsoleError`], which represents failures
//! when attempting to serialize errors or log them to the browser console.
//!
//! # Error Variants
//!
//! - [`BrowserConsoleError::Serialization`] - Serialization to JsValue failed
//! - [`BrowserConsoleError::ConsoleUnavailable`] - Console object not
//!   accessible
//! - [`BrowserConsoleError::ConsoleErrorUnavailable`] - console.error not
//!   accessible
//! - [`BrowserConsoleError::ConsoleMethodNotCallable`] - console.error not
//!   callable
//! - [`BrowserConsoleError::ConsoleInvocation`] - console.error invocation
//!   failed
//! - [`BrowserConsoleError::UnsupportedTarget`] - Not a WASM target
//!
//! # Examples
//!
//! ```
//! use masterror::frontend::BrowserConsoleError;
//!
//! let err = BrowserConsoleError::Serialization {
//!     message: "invalid JSON".to_owned()
//! };
//! assert_eq!(err.context(), Some("invalid JSON"));
//! ```

use crate::Error;

/// Error returned when emitting to the browser console fails or is unsupported.
///
/// # Examples
///
/// ```
/// use masterror::frontend::BrowserConsoleError;
///
/// let err = BrowserConsoleError::UnsupportedTarget;
/// assert_eq!(
///     err.to_string(),
///     "browser console logging is not supported on this target"
/// );
///
/// let err = BrowserConsoleError::ConsoleMethodNotCallable;
/// assert!(err.to_string().contains("not callable"));
/// ```
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub enum BrowserConsoleError {
    /// Failed to serialize the payload into [`wasm_bindgen::JsValue`].
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::Serialization {
    ///     message: "JSON error".to_owned()
    /// };
    /// assert_eq!(err.context(), Some("JSON error"));
    /// assert!(err.to_string().contains("failed to serialize"));
    /// ```
    #[error("failed to serialize payload for browser console: {message}")]
    Serialization {
        /// Human-readable description of the serialization failure.
        message: String
    },

    /// The global `console` object is unavailable or could not be accessed.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::ConsoleUnavailable {
    ///     message: "console is null".to_owned()
    /// };
    /// assert_eq!(err.context(), Some("console is null"));
    /// ```
    #[error("browser console object is not available: {message}")]
    ConsoleUnavailable {
        /// Additional context explaining the failure.
        message: String
    },

    /// The `console.error` function is missing or not accessible.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::ConsoleErrorUnavailable {
    ///     message: "error method undefined".to_owned()
    /// };
    /// assert_eq!(err.context(), Some("error method undefined"));
    /// ```
    #[error("failed to access browser console `error`: {message}")]
    ConsoleErrorUnavailable {
        /// Additional context explaining the failure.
        message: String
    },

    /// The retrieved `console.error` value is not callable.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::ConsoleMethodNotCallable;
    /// assert_eq!(err.context(), None);
    /// ```
    #[error("browser console `error` method is not callable")]
    ConsoleMethodNotCallable,

    /// Invoking `console.error` returned an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::ConsoleInvocation {
    ///     message: "TypeError".to_owned()
    /// };
    /// assert_eq!(err.context(), Some("TypeError"));
    /// ```
    #[error("failed to invoke browser console `error`: {message}")]
    ConsoleInvocation {
        /// Textual representation of the JavaScript exception.
        message: String
    },

    /// Logging is not supported on the current compilation target.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::UnsupportedTarget;
    /// assert_eq!(err.context(), None);
    /// ```
    #[error("browser console logging is not supported on this target")]
    UnsupportedTarget
}

impl BrowserConsoleError {
    /// Returns the contextual message associated with the error, when
    /// available.
    ///
    /// This is primarily useful for surfacing browser-provided diagnostics in
    /// higher-level logs or telemetry.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "frontend")]
    /// # {
    /// use masterror::frontend::BrowserConsoleError;
    ///
    /// let err = BrowserConsoleError::ConsoleUnavailable {
    ///     message: "console missing".to_owned()
    /// };
    /// assert_eq!(err.context(), Some("console missing"));
    ///
    /// let err = BrowserConsoleError::ConsoleMethodNotCallable;
    /// assert_eq!(err.context(), None);
    /// # }
    /// ```
    pub fn context(&self) -> Option<&str> {
        match self {
            Self::Serialization {
                message
            }
            | Self::ConsoleUnavailable {
                message
            }
            | Self::ConsoleErrorUnavailable {
                message
            }
            | Self::ConsoleInvocation {
                message
            } => Some(message.as_str()),
            Self::ConsoleMethodNotCallable | Self::UnsupportedTarget => None
        }
    }
}
