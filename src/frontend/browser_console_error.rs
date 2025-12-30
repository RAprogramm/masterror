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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_creates_identical_copy() {
        let err1 = BrowserConsoleError::Serialization {
            message: "test".to_string()
        };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
        let err3 = BrowserConsoleError::ConsoleMethodNotCallable;
        let err4 = err3.clone();
        assert_eq!(err3, err4);
    }

    #[test]
    fn context_returns_none_for_unit_variants() {
        assert_eq!(
            BrowserConsoleError::ConsoleMethodNotCallable.context(),
            None
        );
        assert_eq!(BrowserConsoleError::UnsupportedTarget.context(), None);
    }

    #[test]
    fn context_returns_message_for_serialization() {
        let err = BrowserConsoleError::Serialization {
            message: "JSON parse error".to_string()
        };
        assert_eq!(err.context(), Some("JSON parse error"));
    }

    #[test]
    fn context_returns_message_for_console_invocation() {
        let err = BrowserConsoleError::ConsoleInvocation {
            message: "TypeError: null reference".to_string()
        };
        assert_eq!(err.context(), Some("TypeError: null reference"));
    }

    #[test]
    fn partial_eq_compares_variants_correctly() {
        let serialization1 = BrowserConsoleError::Serialization {
            message: "error1".to_string()
        };
        let serialization2 = BrowserConsoleError::Serialization {
            message: "error1".to_string()
        };
        let serialization3 = BrowserConsoleError::Serialization {
            message: "error2".to_string()
        };
        assert_eq!(serialization1, serialization2);
        assert_ne!(serialization1, serialization3);
        assert_ne!(serialization1, BrowserConsoleError::UnsupportedTarget);
    }

    #[test]
    fn debug_format_includes_variant_and_message() {
        let err = BrowserConsoleError::ConsoleUnavailable {
            message: "console is undefined".to_string()
        };
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("ConsoleUnavailable"));
        assert!(debug_str.contains("console is undefined"));
    }
}
