# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- `masterror::error::template` module providing a parsed representation of
  `#[error("...")]` strings and a formatter hook for future custom derives.

## [0.5.0] - 2025-09-23

### Added
- Re-exported `thiserror::Error` as `masterror::Error`, making it possible to
  derive domain errors without an extra dependency. The derive supports
  `#[from]` conversions, validates `#[error(transparent)]` wrappers, and mirrors
  `thiserror`'s ergonomics.
- Added `BrowserConsoleError::context()` for retrieving browser-provided
  diagnostics when console logging fails.

### Changed
- README generation now pulls from crate metadata via the build script while
  staying inert during `cargo package`, preventing dirty worktrees in release
  workflows.

### Documentation
- Documented deriving custom errors via `masterror::Error` and expanded the
  browser console section with context-handling guidance.
- Added a release checklist and described the automated README sync process.

### Tests
- Added regression tests covering derive behaviour (including `#[from]` and
  transparent wrappers) and ensuring the README stays in sync with its
  template.
- Added a guard test that enforces the `AppResult<_>` alias over raw
  `Result<_, AppError>` usages within the crate.

## [0.4.0] - 2025-09-15
### Added
- Optional `frontend` feature:
  - Converts [`AppError`] and [`ErrorResponse`] into `wasm_bindgen::JsValue` for browser contexts.
  - Logs structured errors to the browser console via `console.error`.
- `BrowserConsoleError` and `BrowserConsoleExt` API for WASM front-ends.

### Documentation
- Documented browser/WASM support and console logging workflow in the README and crate docs.

## [0.3.5] - 2025-09-12
### Added
- Conversion from `teloxide_core::RequestError` into `AppError` (feature `teloxide`).

## [0.3.4] - 2025-09-12
### Added
- `ErrorResponse::with_retry_after_duration` helper for specifying retry advice via `Duration`.
- Conversion from `telegram_webapp_sdk::utils::validate_init_data::ValidationError` into `AppError` (feature `telegram-webapp-sdk`).

### Changed
- `AppError::log` now includes the stable `code` field alongside `kind`.
- `AppError` stores messages as `Cow<'static, str>` to avoid unnecessary allocations.

### Documentation
- Clarified how `config::ConfigError` converts into `AppErrorKind::Config`.
- Documented that `MultipartError` maps to `AppErrorKind::BadRequest` in the Axum adapter.

### Tests
- Added unit test verifying `config::ConfigError` mapping.
- Added Axum test asserting `MultipartError` becomes `AppErrorKind::BadRequest` and preserves the message.
- Expanded Actix test to check JSON body and `Retry-After`/`WWW-Authenticate` headers.
- Covered fallback classification of unknown messages as `TurnkeyErrorKind::Service`.
- Expanded coverage of `telegram_webapp_sdk` mapping across all `ValidationError` variants.

## [0.3.3] - 2025-09-11
### Added
- `ErrorResponse::status_code()` exposing validated `StatusCode`.
- `ErrorResponse::new` now checks the supplied status code.

### Changed
- Preserve original `reqwest` timeout error text.
- Redis errors map to `AppErrorKind::Cache`.
- Dependencies updated.

### Fixed
- Axum and Actix adapters reuse `status_code()` to avoid type mismatches.

### Documentation
- Clarified `contains_nocase` and `ascii_lower` comments.

## [0.3.2] - 2025-09-08
### Added
- New feature flag `turnkey`:
  - Provides `TurnkeyErrorKind` (stable taxonomy of Turnkey-specific failures).
  - Provides `TurnkeyError` (kind + public message).
  - Adds `classify_turnkey_error` helper for mapping raw SDK/provider messages.
  - Includes conversions into `AppError` / `AppErrorKind`.

### Notes
- Feature is framework-agnostic; no extra dependencies are pulled.

## [0.3.1] - 2025-08-25
### Added
- Implemented `axum::response::IntoResponse` for `AppError` (behind the `axum` feature).
  This allows using `AppError` directly as a rejection type in Axum extractors and handlers.

### Notes
- The implementation delegates to `ErrorResponse` to ensure a single, stable wire contract.

## [0.3.0] - 2025-08-24
### Added
- `AppCode` — stable machine-readable error code (part of the wire contract).
- `ErrorResponse.code`, `ErrorResponse.retry`, `ErrorResponse.www_authenticate` fields.
- Axum/Actix integrations now set `Retry-After` and `WWW-Authenticate` headers when applicable.

### Changed (breaking)
- `ErrorResponse::new` now requires `(status: u16, code: AppCode, message: impl Into<String>)`.

### Migration
- Replace `ErrorResponse::new(status, "msg")` with  
  `ErrorResponse::new(status, AppCode::<Variant>, "msg")`.
- Optionally use `.with_retry_after_secs(...)` and/or `.with_www_authenticate(...)`
  to populate the new fields.

## [0.2.1] - 2025-08-20
### Changed
- Cleaned up feature flags: clarified `openapi` vs `openapi-*`.
- Simplified error DTOs (`ErrorResponse`) with proper `ToSchema` support.
- Minor code cleanup in Actix and SQLx integration.

### Notes
- **MSRV:** 1.89
- **No unsafe**

## [0.2.0] - 2025-08-20
### Added
- Actix integration:
  - `AppError` implements `actix_web::ResponseError`.
  - `ErrorResponse` implements `actix_web::Responder`.

### Changed
- Expanded documentation:
  - Complete `README.md` with installation, usage examples, and feature flags.
  - Improved module-level doc comments and design notes.
- Error conversions: feature-gated submodules (`sqlx`, `reqwest`, `redis`, `tokio`, `validator`, etc.).

### Notes
- **MSRV:** 1.89
- **No unsafe:** the crate forbids `unsafe`.

[0.5.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.5.0
[0.4.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.4.0
[0.3.5]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.5
[0.3.4]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.4
[0.3.3]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.3
[0.3.2]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.2
[0.3.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.1
[0.3.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.0
[0.2.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.1
[0.2.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.0

