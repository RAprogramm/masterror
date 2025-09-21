# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.10.9] - 2025-10-26

### Changed
- Raised the documented MSRV to Rust 1.90 to match the `rust-version`
  requirement.

### Documentation
- Regenerated the README from the template so installation snippets reflect the
  new crate version and MSRV statement.

## [0.10.8] - 2025-10-25

### Fixed
- Updated the release workflow to publish `masterror-template` before
  `masterror-derive`, ensuring crates.io recognises the shared dependency during
  release automation.

## [0.10.7] - 2025-10-24

### Fixed
- Published the shared template parser crate so `masterror-derive` no longer
  depends on a workspace-only package when uploaded to crates.io.

### Documentation
- Added a dedicated README for `masterror-template` describing installation,
  parsing examples and formatter metadata for crates.io readers.
### Tests
- Added regression coverage for long classifier needles to exercise the
  heap-allocation fallback.

### Changed
- Added an owning `From<AppError>` conversion for `ErrorResponse` and updated the
  Axum adapter to use it, eliminating redundant clones when building HTTP error
  bodies.
 - Precomputed lowercase Turnkey classifier needles with a stack-backed buffer
  to remove repeated transformations while keeping the common zero-allocation
  path for short patterns.
- Bumped `masterror-derive` to `0.6.6` and `masterror-template` to `0.3.6` so
  downstream users rely on the newly published parser crate.


## [0.10.6] - 2025-09-21

### Fixed
- Added a crate-local README for `masterror-derive` so `cargo publish` passes
  when crates.io validates the `readme` manifest key.

### Changed
- Bumped `masterror-derive` to `0.6.2` to capture the packaging fix.

### Documentation
- Documented the derive macros and supported attributes in
  `masterror-derive/README.md` for crates.io readers.

## [0.10.5] - 2025-09-20

### Added
- Re-exported `masterror-derive` macros from `masterror` so consumers only depend on a single crate while deriving application errors.

### Changed
- Published `masterror-derive` as a standalone crate (`0.6.1`) and configured the release workflow to publish it before `masterror` with retries and tag/MSRV validation.

### Documentation
- Described `#[provide]` telemetry providers and `#[app_error]` conversions with
  end-to-end examples in the derive guide ([README](README.md#structured-telemetry-providers-and-apperror-mappings),
  [README.ru](README.ru.md#%D0%B0%D1%82%D1%80%D0%B8%D0%B1%D1%83%D1%82%D1%8B-provide-%D0%B8-apperror)).

## [0.10.4] - 2025-09-20

### Fixed
- Ensured `cargo package --locked` passes by switching workspace dependencies on
  `masterror-derive` / `masterror-template` to registry entries and overriding
  them locally through `.cargo/config`, keeping CI dry runs green without
  breaking local development.

## [0.10.2] - 2025-10-23

### Added
- Forward dynamic width and precision specifiers by emitting every declared
  format argument into the generated `write!` call, so placeholders like
  `{value:>width$}` and `{value:.precision$}` remain valid when deriving
  `Display`.

### Changed
- `FormatArgumentsEnv` now surfaces tokens for all named, positional and
  implicit bindings—even when they are only referenced from format specs—so
  width/precision values reach the formatting engine.
- `render_template`/`build_template_arguments` combine the resolved
  placeholders with the full format argument list, ensuring the macro invocation
  always receives the required bindings.

### Tests
- Added UI fixtures and integration assertions covering dynamic width and
  precision formatting to guard against regressions.

### Documentation
- Documented the dynamic width/precision support alongside the formatting
  guidance (including the Russian translation).

## [0.10.1] - 2025-10-22

### Changed
- Relaxed template formatter parsing so only typed formatters treat `#` as the
  alternate flag, allowing display placeholders such as `{value:#>4}` to round-
  trip without spurious `TemplateError::InvalidFormatter` errors.

### Tests
- Extended formatter unit tests and UI derive coverage to exercise hash-filled
  display specs and ensure they parse correctly.

### Documentation
- Documented the broader display formatter support (including `#` as a fill
  character) in the templating README section.

## [0.10.0] - 2025-10-21

### Added
- Preserved the raw format fragment for display-only placeholders, exposing it
  through `TemplateFormatter::display_spec()`/`format_fragment()` so derived
  implementations can forward `:>8`, `:.3`, and similar specifiers to
  `write!`.

### Changed
- `TemplateFormatter` now owns display specs and `TemplatePlaceholder::formatter`
  returns a reference to reflect the richer formatter representation.

### Tests
- Added a trybuild pass case and runtime assertions covering display alignment,
  precision, and fill specifiers to prevent regressions.

### Documentation
- Documented the new display formatter support in the README (including the
  Russian translation) with examples showing how to recover preserved specs.

## [0.9.0] - 2025-10-20

### Added
- Parsed dot-prefixed display shorthands into a projection AST so `.limits.lo`,
  `.0.data`, and chained method calls like `.suggestion.as_ref().map_or_else(...)`
  resolve against struct fields and variant bindings.
- Extended the `error_derive` integration suite and trybuild fixtures with
  regressions covering nested projections for named and tuple variants.

### Changed
- Shorthand resolution now builds expressions from the projection AST, preserving
  raw identifiers, tuple indices, and method invocations when generating code.

### Documentation
- Documented the richer shorthand projection support in the README and template
  so downstream users know complex field/method chains are available.

## [0.8.0] - 2025-10-14

### Added
- Recognised `#[provide(ref = ..., value = ...)]` on struct and enum fields,
  allowing derived errors to surface domain telemetry through
  `std::error::Request` alongside backtraces.

### Changed
- `masterror-derive` now generates `provide` implementations whenever custom
  telemetry is requested, forwarding `Request` values to sources and invoking
  `provide_ref`/`provide_value` with proper `Option` handling.

### Tests
- Extended the `error_derive` integration suite with regressions covering
  telemetry provided by structs, tuple variants and optional fields, including
  both reference and owned payloads.

### Documentation
- Documented the `#[provide(...)]` attribute in the README with examples showing
  reference and owned telemetry as well as optional fields.

## [0.7.0] - 2025-10-13

### Added
- Recognised `#[app_error(...)]` on derived structs and enum variants, capturing
  the mapped `AppErrorKind`, optional `AppCode` and whether the formatted
  `Display` output should become the public message.
- Generated `From<Error>` implementations that construct `masterror::AppError`
  (and, when requested, `AppCode`) by matching on enum variants and invoking
  `AppError::with`/`AppError::bare`.

### Tests
- Introduced trybuild fixtures covering successful struct/enum conversions and
  compile failures for missing metadata, including message propagation checks in
  the passing cases.

### Documentation
- Documented the `#[app_error(...)]` attribute in the README, outlining the
  struct and enum mapping patterns and the `message` flag behaviour.

## [0.6.6] - 2025-10-24

### Fixed
- Pointed the derive crate at the published `masterror-template` dependency so
  `cargo publish` succeeds without private workspace patches.

## [0.6.5] - 2025-10-12

### Added
- Accepted `.field` and `.0` shorthand expressions in `#[error("...")]` format
  argument lists, resolving them against struct and variant fields without
  moving the original values.

### Changed
- The format argument resolver now tracks whether it operates on a struct or a
  destructured enum variant, allowing field shorthands to reuse local bindings
  and honour pointer formatting requirements.

### Tests
- Added trybuild pass cases covering named, positional and implicit arguments,
  formatter path handlers and the new field shorthand expressions.
- Introduced compile-fail fixtures for duplicate argument names, mixing
  implicit placeholders after explicitly indexed ones and combining
  `transparent` with `fmt` handlers.
- Extended the runtime `error_derive` suite with assertions exercising the
  shorthand field accessors.

## [0.6.4] - 2025-10-11

### Added
- Exposed an internal `provide` shim that mirrors `thiserror`'s
  `ThiserrorProvide`, enabling derived errors to forward
  `core::error::Request` values to their sources.

### Changed
- Allow `#[backtrace]` to be paired with `#[source]`/`#[from]` fields when the
  field type implements `Error`, while retaining diagnostics for incompatible
  non-source fields.
- Track whether backtrace detection is explicit or inferred so generated
  implementations avoid providing the same backtrace twice when delegating to
  sources.
- Update the generated `provide` methods to call `thiserror_provide` on source
  fields before exposing the stored backtrace, ensuring delegated traces reach
  callers.

### Tests
- Added regression tests covering direct and optional sources annotated with
  `#[backtrace]`, validating delegated backtrace propagation and `None`
  handling.

## [0.6.3] - 2025-10-10

### Added
- Invoke custom `#[error(fmt = <path>)]` handlers for structs and enum variants,
  borrowing fields and forwarding the formatter reference just like `thiserror`.

### Changed
- Ensure duplicate `fmt` attributes report a single diagnostic without
  suppressing the derived display implementation.

### Tests
- Extend the formatter trybuild suite with success cases covering struct and
  enum formatter paths.

## [0.6.2] - 2025-10-09

### Added
- Resolve `#[error("...")]` format arguments when generating `Display`
  implementations, supporting named bindings, explicit indices and implicit
  placeholders via a shared argument environment.

### Changed
- Detect additional format arguments, implicit placeholders and non-`Display`
  formatters in `render_template`, delegating complex cases to a single
  `write!` invocation while retaining the lightweight `f.write_str` path for
  literal-only templates. The helper that assembles format arguments now keeps
  positional/implicit bindings ahead of named ones to satisfy the formatting
  macro contract.

### Tests
- Cover named format argument expressions, implicit placeholder ordering and
  enum variants using format arguments.

## [0.6.0] - 2025-10-08

### Added
- Recognised empty placeholder bodies (`{}` / `{:?}`) as implicit positional
  identifiers, numbering them by appearance and exposing the new
  `TemplateIdentifier::Implicit` variant in the template API.
- Propagated the implicit identifier metadata through
  `template_support::TemplateIdentifierSpec`, ensuring derive-generated display
  implementations resolve tuple fields in placeholder order.

### Fixed
- Preserved `TemplateError::EmptyPlaceholder` diagnostics for whitespace-only
  placeholders, matching previous error reporting for invalid bodies.

### Tests
- Added parser regressions covering implicit placeholder sequencing and the
  whitespace-only error path.

## [0.5.15] - 2025-10-07

### Added
- Parse `#[error("...")]` attribute arguments into structured `FormatArg`
  entries, tracking named bindings and positional indices for future
  `format_args!` integration.
- Recognise `#[error(fmt = <path>)]` handlers, capturing the formatter path and
  associated arguments while guarding against duplicate `fmt` specifications.

### Fixed
- Produce dedicated diagnostics when unsupported combinations are used, such as
  providing format arguments alongside `#[error(transparent)]`.

### Tests
- Extend the `trybuild` suite with regression cases covering duplicate `fmt`
  handlers and transparent attributes that erroneously include arguments.

## [0.5.14] - 2025-10-06

### Added
- Prepared the derive input structures for future `format_args!` support by
  introducing display specification variants for templates with arguments and
  `fmt = <path>` handlers, along with `FormatArgsSpec`/`FormatArg` metadata
  scaffolding.

## [0.5.13] - 2025-10-05

### Documentation
- Documented the formatter trait helpers (`TemplateFormatter::is_alternate`,
  `TemplateFormatter::from_kind`, and `TemplateFormatterKind::specifier`/`supports_alternate`)
  across README variants and crate docs, including guidance on the extended
  formatter table and compatibility with `thiserror` v2.

## [0.5.12] - 2025-10-04

### Tests
- Added runtime assertions covering every derive formatter variant and
  validating lowercase versus uppercase rendering differences during error
  formatting.
- Expanded the formatter `trybuild` suite with per-formatter success cases and
  new compile-fail fixtures for unsupported uppercase specifiers to guarantee
  diagnostics remain descriptive.

## [0.5.11] - 2025-10-03

### Changed
- Aligned the derive display generator with `TemplateFormatterKind`, invoking the
  appropriate `core::fmt` trait for every placeholder variant and preserving the
  default `Display` path when no formatter is provided, mirroring `thiserror`'s
  behaviour.

## [0.5.10] - 2025-10-02

### Changed
- Template parser now recognises formatter traits even when alignment, sign or
  width flags precede the type specifier, constructing the matching
  `TemplateFormatter` variant and keeping alternate (`#`) detection aligned with
  `thiserror`.

### Tests
- Extended parser unit tests to cover complex formatter specifiers and
  additional malformed cases to guard diagnostic accuracy.

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

## [0.3.6] - 2025-10-24

### Added
- Wrote a README for crates.io explaining installation and parser usage.

### Fixed
- Removed the `publish = false` flag so the shared template parser can be
  released alongside the derive crate.

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

[0.10.7]: https://github.com/RAprogramm/masterror/releases/tag/v0.10.7
[0.10.6]: https://github.com/RAprogramm/masterror/releases/tag/v0.10.6
[0.6.6]: https://github.com/RAprogramm/masterror/releases/tag/v0.6.6
[0.6.5]: https://github.com/RAprogramm/masterror/releases/tag/v0.6.5
[0.6.4]: https://github.com/RAprogramm/masterror/releases/tag/v0.6.4
[0.6.3]: https://github.com/RAprogramm/masterror/releases/tag/v0.6.3
[0.6.2]: https://github.com/RAprogramm/masterror/releases/tag/v0.6.2
[0.6.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.6.0
[0.5.2]: https://github.com/RAprogramm/masterror/releases/tag/v0.5.2
[0.5.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.5.1
[0.5.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.5.0
[0.4.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.4.0
[0.3.6]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.6
[0.3.5]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.5
[0.3.4]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.4
[0.3.3]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.3
[0.3.2]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.2
[0.3.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.1
[0.3.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.3.0
[0.2.1]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.1
[0.2.0]: https://github.com/RAprogramm/masterror/releases/tag/v0.2.0

