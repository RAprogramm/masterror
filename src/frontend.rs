//! Browser/WASM helpers for converting application errors into JavaScript
//! values.
//!
//! This module is available when the `frontend` feature is enabled. It provides
//! helpers to serialize [`AppError`] and [`ErrorResponse`] into
//! [`wasm_bindgen::JsValue`] and optionally emit structured logs via
//! `console.error` when running inside a browser.
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "frontend")]
//! # {
//! use masterror::{
//!     AppError,
//!     frontend::{BrowserConsoleError, BrowserConsoleExt}
//! };
//!
//! let err = AppError::bad_request("invalid payload");
//!
//! #[cfg(target_arch = "wasm32")]
//! {
//!     let js = err.to_js_value().expect("js value");
//!     assert!(js.is_object());
//!     err.log_to_browser_console().expect("console error log");
//! }
//!
//! #[cfg(not(target_arch = "wasm32"))]
//! assert!(matches!(
//!     err.to_js_value(),
//!     Err(BrowserConsoleError::UnsupportedTarget)
//! ));
//! # }
//! ```

#[cfg(target_arch = "wasm32")]
use js_sys::{Function, Reflect};
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::to_value;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::{AppError, AppResult, Error, ErrorResponse};

/// Error returned when emitting to the browser console fails or is unsupported.
#[derive(Debug, Error, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub enum BrowserConsoleError {
    /// Failed to serialize the payload into [`JsValue`].
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

/// Extensions for serializing errors to JavaScript and logging to the browser
/// console.
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub trait BrowserConsoleExt {
    /// Convert the error into a [`JsValue`] suitable for passing to JavaScript.
    fn to_js_value(&self) -> AppResult<JsValue, BrowserConsoleError>;

    /// Emit the error as a structured payload via `console.error`.
    ///
    /// On non-WASM targets this returns
    /// [`BrowserConsoleError::UnsupportedTarget`].
    fn log_to_browser_console(&self) -> AppResult<(), BrowserConsoleError> {
        let payload = self.to_js_value()?;
        log_js_value(&payload)
    }
}

impl BrowserConsoleExt for ErrorResponse {
    fn to_js_value(&self) -> AppResult<JsValue, BrowserConsoleError> {
        #[cfg(target_arch = "wasm32")]
        {
            to_value(self).map_err(|err| BrowserConsoleError::Serialization {
                message: err.to_string()
            })
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(BrowserConsoleError::UnsupportedTarget)
        }
    }
}

impl BrowserConsoleExt for AppError {
    fn to_js_value(&self) -> AppResult<JsValue, BrowserConsoleError> {
        #[cfg(target_arch = "wasm32")]
        {
            let response: ErrorResponse = self.into();
            response.to_js_value()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(BrowserConsoleError::UnsupportedTarget)
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn log_js_value(value: &JsValue) -> AppResult<(), BrowserConsoleError> {
    let global = js_sys::global();
    let console = Reflect::get(&global, &JsValue::from_str("console")).map_err(|err| {
        BrowserConsoleError::ConsoleUnavailable {
            message: format_js_value(&err)
        }
    })?;

    if console.is_undefined() || console.is_null() {
        return Err(BrowserConsoleError::ConsoleUnavailable {
            message: "console is undefined".into()
        });
    }

    let error_fn = Reflect::get(&console, &JsValue::from_str("error")).map_err(|err| {
        BrowserConsoleError::ConsoleErrorUnavailable {
            message: format_js_value(&err)
        }
    })?;

    if error_fn.is_undefined() || error_fn.is_null() {
        return Err(BrowserConsoleError::ConsoleErrorUnavailable {
            message: "console.error is undefined".into()
        });
    }

    let func = error_fn
        .dyn_into::<Function>()
        .map_err(|_| BrowserConsoleError::ConsoleMethodNotCallable)?;

    func.call1(&console, value)
        .map_err(|err| BrowserConsoleError::ConsoleInvocation {
            message: format_js_value(&err)
        })?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn log_js_value(_value: &JsValue) -> AppResult<(), BrowserConsoleError> {
    Err(BrowserConsoleError::UnsupportedTarget)
}

#[cfg(target_arch = "wasm32")]
fn format_js_value(value: &JsValue) -> String {
    value.as_string().unwrap_or_else(|| format!("{value:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppCode;

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::*;

        #[test]
        fn to_js_value_is_unsupported_on_native_targets() {
            let response =
                ErrorResponse::new(404, AppCode::NotFound, "missing user").expect("status");
            assert!(matches!(
                response.to_js_value(),
                Err(BrowserConsoleError::UnsupportedTarget)
            ));

            let err = AppError::conflict("already exists");
            assert!(matches!(
                err.to_js_value(),
                Err(BrowserConsoleError::UnsupportedTarget)
            ));
        }

        #[test]
        fn console_logging_returns_unsupported_on_native_targets() {
            let err = AppError::internal("boom");
            let result = err.log_to_browser_console();
            assert!(matches!(
                result,
                Err(BrowserConsoleError::UnsupportedTarget)
            ));
        }
    }

    #[cfg(target_arch = "wasm32")]
    mod wasm {
        use serde_wasm_bindgen::from_value;

        use super::*;
        use crate::AppErrorKind;

        #[test]
        fn error_response_to_js_value_roundtrip() {
            let response =
                ErrorResponse::new(404, AppCode::NotFound, "missing user").expect("status");
            let js = response.to_js_value().expect("serialize");
            let decoded: ErrorResponse = from_value(js).expect("decode");
            assert_eq!(decoded.status, 404);
            assert_eq!(decoded.code, AppCode::NotFound);
            assert_eq!(decoded.message, "missing user");
        }

        #[test]
        fn app_error_to_js_value_roundtrip() {
            let err = AppError::conflict("already exists");
            let js = err.to_js_value().expect("serialize");
            let decoded: ErrorResponse = from_value(js).expect("decode");
            assert_eq!(decoded.code, AppCode::Conflict);
            assert_eq!(decoded.message, "already exists");
            assert_eq!(decoded.status, AppErrorKind::Conflict.http_status());
        }
    }
}
