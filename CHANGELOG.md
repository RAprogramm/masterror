# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- _Nothing yet._

## [0.5.9] - 2025-10-01

### Added
- `TemplateFormatterKind` enumerating the formatter traits supported by
  `#[error("...")]`, plus `TemplateFormatter::from_kind`/`kind()` helpers for
  constructing and inspecting placeholders programmatically.

### Changed
- Formatter parsing now routes through `TemplateFormatterKind`, ensuring lookup
  tables, `is_alternate` handling and downstream derives share the same
  canonical representation.

### Documentation
- Documented `TemplateFormatterKind` usage and the new inspection helpers
  across README variants.

## [0.5.8] - 2025-09-30

### Changed
- `masterror::Error` now infers sources named `source` and backtrace fields of
  type `std::backtrace::Backtrace`/`Option<std::backtrace::Backtrace>` even
  without explicit attributes, matching `thiserror`'s ergonomics.

### Tests
- Expanded derive tests to cover implicit `source`/`backtrace` detection across
  structs and enums.

## [0.5.7] - 2025-09-29

### Added
- `masterror::error::template` module providing a parsed representation of
  `#[error("...")]` strings and a formatter hook for future custom derives.
- Internal `masterror-derive` crate powering the native `masterror::Error`
  derive macro.
- Template placeholders now accept the same formatter traits as `thiserror`
  (`:?`, `:x`, `:X`, `:p`, `:b`, `:o`, `:e`, `:E`) so existing derives keep
  compiling when hexadecimal, binary, pointer or exponential formatting is
  requested.

### Changed
- `masterror::Error` now uses the in-tree derive, removing the dependency on
  `thiserror` while keeping the same runtime behaviour and diagnostics.

### Documentation
- Documented formatter trait usage across README.md, README.ru.md and the
  `masterror::error` module, noting compatibility with `thiserror` v2 and
  demonstrating programmatic `TemplateFormatter` inspection.

## [0.5.6] - 2025-09-28

### Tests
- Added runtime coverage exercising every derive formatter variant (including
  case-sensitive formatters) and asserted the rendered output.
- Added `trybuild` suites that compile successful formatter usage and verify the
  emitted diagnostics for unsupported specifiers.

## [0.5.5] - 2025-09-27

### Fixed
- Derive formatter generation now matches on every `TemplateFormatter`
  variant and calls the corresponding `::core::fmt` trait (including the
  default `Display` path), mirroring `thiserror`'s placeholder handling.

## [0.5.4] - 2025-09-26

### Fixed
- Template parser mirrors `thiserror`'s formatter trait detection, ensuring
  `:?`, `:x`, `:X`, `:p`, `:b`, `:o`, `:e` and `:E` specifiers resolve to the
  appropriate `TemplateFormatter` variant while still flagging unsupported
  flags precisely.

### Tests
- Added parser-level unit tests that cover every supported formatter specifier
  and assert graceful failures for malformed format strings.

## [0.5.2] - 2025-09-25

### Fixed
- Added a workspace `deny.toml` allow-list for MIT, Apache-2.0 and Unicode-3.0
  licenses so `cargo deny` accepts existing dependencies.
- Declared SPDX license expressions for the internal `masterror-derive` and
  `masterror-template` crates to avoid unlicensed warnings.

## [0.5.1] - 2025-09-24

### Changed
- Replaced the optional `sqlx` dependency with `sqlx-core` so enabling the
  feature no longer pulls in `rsa` via the MySQL driver, fixing the
  `RUSTSEC-2023-0071` advisory reported by `cargo audit`.

### Security
- Added `cargo audit` to the pre-commit hook and CI workflow; published a
  README badge to surface the audit status.

### Added
- Composite GitHub Action (`.github/actions/cargo-deny`) that installs and runs
  `cargo-deny` checks for reuse across workflows.
- `cargo deny` step in the reusable CI pipeline to catch advisories, bans,
  license and source issues automatically.
- README badges surfacing the Cargo Deny status so consumers can quickly verify
  supply-chain checks.

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

[0.5.2]: https://github.com/RAprogramm/masterror/releases/tag/v0.5.2
[0.5.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.5.1
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

