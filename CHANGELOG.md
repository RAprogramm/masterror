# Changelog
All notable changes to this project will be documented in this file.

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

[0.3.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.0
[0.2.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.1
[0.2.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.0

