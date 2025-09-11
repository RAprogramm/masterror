# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]
### Added
- `ErrorResponse::with_retry_after_duration` helper for specifying retry advice via `Duration`.

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

[0.3.3]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.3
[0.3.2]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.2
[0.3.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.1
[0.3.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.0
[0.2.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.1
[0.2.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.0

