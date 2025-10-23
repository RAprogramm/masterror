// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Browser console logging extensions for errors.
//!
//! This module provides the [`BrowserConsoleExt`] trait for serializing errors
//! to JavaScript values and logging them to the browser console.
//!
//! # Platform Support
//!
//! - **WASM target** (`target_arch = "wasm32"`): Full functionality available
//! - **Native targets**: Returns [`BrowserConsoleError::UnsupportedTarget`]
//!
//! # Examples
//!
//! ```rust,ignore
//! use masterror::{AppError, frontend::BrowserConsoleExt};
//!
//! let err = AppError::not_found("user not found");
//!
//! // Serialize to JsValue (WASM only)
//! let js_value = err.to_js_value()?;
//!
//! // Log to browser console (WASM only)
//! err.log_to_browser_console()?;
//! ```

#[cfg(target_arch = "wasm32")]
use js_sys::{Function, Reflect};
#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::to_value;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use super::BrowserConsoleError;
use crate::{AppError, AppResult, ErrorResponse};

/// Extensions for serializing errors to JavaScript and logging to the browser
/// console.
///
/// # Examples
///
/// ```rust,ignore
/// use masterror::{AppError, frontend::BrowserConsoleExt};
///
/// let err = AppError::not_found("resource missing");
/// let js_value = err.to_js_value()?;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub trait BrowserConsoleExt {
    /// Convert the error into a [`JsValue`] suitable for passing to JavaScript.
    ///
    /// On WASM targets, serializes the error to a JavaScript object.
    /// On non-WASM targets, returns [`BrowserConsoleError::UnsupportedTarget`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{
    ///     AppError,
    ///     frontend::{BrowserConsoleError, BrowserConsoleExt}
    /// };
    ///
    /// let err = AppError::not_found("user not found");
    ///
    /// #[cfg(target_arch = "wasm32")]
    /// {
    ///     let js_value = err.to_js_value().expect("serialize");
    ///     assert!(!js_value.is_undefined());
    /// }
    ///
    /// #[cfg(not(target_arch = "wasm32"))]
    /// {
    ///     assert!(matches!(
    ///         err.to_js_value(),
    ///         Err(BrowserConsoleError::UnsupportedTarget)
    ///     ));
    /// }
    /// ```
    fn to_js_value(&self) -> AppResult<JsValue, BrowserConsoleError>;

    /// Emit the error as a structured payload via `console.error`.
    ///
    /// On WASM targets, logs the error to the browser's developer console.
    /// On non-WASM targets, returns [`BrowserConsoleError::UnsupportedTarget`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{
    ///     AppError,
    ///     frontend::{BrowserConsoleError, BrowserConsoleExt}
    /// };
    ///
    /// let err = AppError::internal("server error");
    ///
    /// #[cfg(not(target_arch = "wasm32"))]
    /// {
    ///     assert!(matches!(
    ///         err.log_to_browser_console(),
    ///         Err(BrowserConsoleError::UnsupportedTarget)
    ///     ));
    /// }
    /// ```
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
