// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Public machine-readable error codes (`AppCode`).
//!
//! ## What is `AppCode`?
//! A **stable error code** you can return to clients (mobile apps, frontends,
//! other services). It is part of the public wire contract and is intended
//! to stay stable across versions. Humans see `message`; machines key off
//! `code`.
//!
//! `AppCode` complements (but does not replace) HTTP status codes:
//! - `status` (e.g., 404, 422, 500) tells **transport-level** outcome.
//! - `code` (e.g., `NOT_FOUND`, `VALIDATION`) tells **semantic category**,
//!   which remains stable even if your transport mapping changes.
//!
//! ## Stability and SemVer
//! - New built-in constants **may be added in minor releases** (non-breaking).
//! - The type is marked `#[non_exhaustive]` to allow future metadata additions
//!   without breaking downstream code.
//! - Custom codes can be defined at compile time with [`AppCode::new`] or at
//!   runtime with [`AppCode::try_new`].
//!
//! ## Typical usage
//! Construct an `ErrorResponse` with a code and return it to clients:
//!
//! ```rust
//! use masterror::{AppCode, ErrorResponse};
//!
//! let resp = ErrorResponse::new(404, AppCode::NotFound, "User not found").expect("status");
//! ```
//!
//! Convert from internal taxonomy (`AppErrorKind`) to a public code:
//!
//! ```rust
//! use masterror::{AppCode, AppErrorKind};
//!
//! let code = AppCode::from(AppErrorKind::Validation);
//! assert_eq!(code.as_str(), "VALIDATION");
//! ```
//!
//! Serialize to JSON (uses SCREAMING_SNAKE_CASE):
//!
//! ```rust
//! # #[cfg(feature = "serde_json")]
//! # {
//! use masterror::AppCode;
//! let json = serde_json::to_string(&AppCode::RateLimited).unwrap();
//! assert_eq!(json, r#""RATE_LIMITED""#);
//! # }
//! ```
//!
//! Match codes safely:
//!
//! ```rust
//! use masterror::AppCode;
//!
//! fn is_client_error(code: &AppCode) -> bool {
//!     matches!(
//!         code.as_str(),
//!         "NOT_FOUND"
//!             | "VALIDATION"
//!             | "CONFLICT"
//!             | "UNAUTHORIZED"
//!             | "FORBIDDEN"
//!             | "NOT_IMPLEMENTED"
//!             | "BAD_REQUEST"
//!             | "RATE_LIMITED"
//!             | "TELEGRAM_AUTH"
//!             | "INVALID_JWT"
//!     )
//! }
//! ```
//!
//! Define custom codes:
//!
//! ```rust
//! use masterror::AppCode;
//!
//! const INVALID_JSON: AppCode = AppCode::new("INVALID_JSON");
//! let third_party = AppCode::try_new(String::from("THIRD_PARTY_FAILURE")).expect("valid code");
//! assert_eq!(INVALID_JSON.as_str(), "INVALID_JSON");
//! assert_eq!(third_party.as_str(), "THIRD_PARTY_FAILURE");
//! ```

mod app_code;

pub use app_code::{AppCode, ParseAppCodeError};
