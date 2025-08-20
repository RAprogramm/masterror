# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.2.1] - 2025-08-20
### Changed
- Cleanup of feature flags: clarified `openapi` vs `openapi-*`.
- Simplified error DTOs (`ErrorResponse`) with proper `ToSchema` support.
- Minor code cleanup in Actix and SQLx integration.

### Notes
- MSRV: 1.89
- No unsafe

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

[0.2.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.1
[0.2.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.0

