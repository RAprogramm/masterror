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

mod browser_console_error;
mod browser_console_ext;

pub use browser_console_error::BrowserConsoleError;
pub use browser_console_ext::BrowserConsoleExt;

#[cfg(test)]
mod tests;
