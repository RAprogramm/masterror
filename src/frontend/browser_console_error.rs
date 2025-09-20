use crate::Error;

/// Error returned when emitting to the browser console fails or is unsupported.
#[derive(Debug, Error, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub enum BrowserConsoleError {
    /// Failed to serialize the payload into [`wasm_bindgen::JsValue`].
    #[error("failed to serialize payload for browser console: {message}")]
    Serialization {
        /// Human-readable description of the serialization failure.
        message: String
    },
    /// The global `console` object is unavailable or could not be accessed.
    #[error("browser console object is not available: {message}")]
    ConsoleUnavailable {
        /// Additional context explaining the failure.
        message: String
    },
    /// The `console.error` function is missing or not accessible.
    #[error("failed to access browser console `error`: {message}")]
    ConsoleErrorUnavailable {
        /// Additional context explaining the failure.
        message: String
    },
    /// The retrieved `console.error` value is not callable.
    #[error("browser console `error` method is not callable")]
    ConsoleMethodNotCallable,
    /// Invoking `console.error` returned an error.
    #[error("failed to invoke browser console `error`: {message}")]
    ConsoleInvocation {
        /// Textual representation of the JavaScript exception.
        message: String
    },
    /// Logging is not supported on the current compilation target.
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
