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
//! - New variants **may be added in minor releases** (non-breaking).
//! - The enum is marked `#[non_exhaustive]` so downstream users must include a
//!   wildcard arm (`_`) when matching, which keeps them forward-compatible.
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
//! Match codes safely (note the wildcard arm due to `#[non_exhaustive]`):
//!
//! ```rust
//! use masterror::AppCode;
//!
//! fn is_client_error(code: AppCode) -> bool {
//!     match code {
//!         AppCode::NotFound
//!         | AppCode::Validation
//!         | AppCode::Conflict
//!         | AppCode::Unauthorized
//!         | AppCode::Forbidden
//!         | AppCode::NotImplemented
//!         | AppCode::BadRequest
//!         | AppCode::RateLimited
//!         | AppCode::TelegramAuth
//!         | AppCode::InvalidJwt => true,
//!         _ => false // future-proof: treat unknown as not client error
//!     }
//! }
//! ```

mod app_code;

pub use app_code::{AppCode, ParseAppCodeError};
