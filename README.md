# masterror · Framework-agnostic application error types

<!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
     CI packaging will fail if README.md is stale. -->

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.90-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Cargo Deny](https://img.shields.io/github/actions/workflow/status/RAprogramm/masterror/ci.yml?branch=main&label=Cargo%20Deny)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)

> 🇷🇺 Читайте README на [русском языке](README.ru.md).

Small, pragmatic error model for API-heavy Rust services.
Core is framework-agnostic; integrations are opt-in via feature flags.
Stable categories, conservative HTTP mapping, no `unsafe`.

- Core types: `AppError`, `AppErrorKind`, `AppResult`, `AppCode`, `ErrorResponse`
- Optional Axum/Actix integration
- Optional OpenAPI schema (via `utoipa`)
- Conversions from `sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio`

---

### TL;DR

~~~toml
[dependencies]
masterror = { version = "0.5.1", default-features = false }
# or with features:
# masterror = { version = "0.5.1", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "sqlx", "reqwest", "redis", "validator",
#   "config", "tokio", "multipart", "teloxide",
#   "telegram-webapp-sdk", "frontend", "turnkey"
# ] }
~~~

*Since v0.5.0: derive custom errors via `#[derive(Error)]` (`use masterror::Error;`) and inspect browser logging failures with `BrowserConsoleError::context()`.*
*Since v0.4.0: optional `frontend` feature for WASM/browser console logging.*
*Since v0.3.0: stable `AppCode` enum and extended `ErrorResponse` with retry/authentication metadata.*

---

<details>
  <summary><b>Why this crate?</b></summary>

- **Stable taxonomy.** Small set of `AppErrorKind` categories mapping conservatively to HTTP.
- **Framework-agnostic.** No assumptions, no `unsafe`, MSRV pinned.
- **Opt-in integrations.** Zero default features; you enable what you need.
- **Clean wire contract.** `ErrorResponse { status, code, message, details?, retry?, www_authenticate? }`.
- **One log at boundary.** Log once with `tracing`.
- **Less boilerplate.** Built-in conversions, compact prelude, and the
  native `masterror::Error` derive with `#[from]` / `#[error(transparent)]`
  support.
- **Consistent workspace.** Same error surface across crates.

</details>

<details>
  <summary><b>Installation</b></summary>

~~~toml
[dependencies]
# lean core
masterror = { version = "0.5.1", default-features = false }

# with Axum/Actix + JSON + integrations
# masterror = { version = "0.5.1", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "sqlx", "reqwest", "redis", "validator",
#   "config", "tokio", "multipart", "teloxide",
#   "telegram-webapp-sdk", "frontend", "turnkey"
# ] }
~~~

**MSRV:** 1.90
**No unsafe:** forbidden by crate.

</details>

<details>
  <summary><b>Quick start</b></summary>

Create an error:

~~~rust
use masterror::{AppError, AppErrorKind};

let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
~~~

With prelude:

~~~rust
use masterror::prelude::*;

fn do_work(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("Flag must be set"));
    }
    Ok(())
}
~~~

</details>

<details>
  <summary><b>Derive custom errors</b></summary>

~~~rust
use std::io;

use masterror::Error;

#[derive(Debug, Error)]
#[error("I/O failed: {source}")]
pub struct DomainError {
    #[from]
    #[source]
    source: io::Error,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct WrappedDomainError(
    #[from]
    #[source]
    DomainError
);

fn load() -> Result<(), DomainError> {
    Err(io::Error::other("disk offline").into())
}

let err = load().unwrap_err();
assert_eq!(err.to_string(), "I/O failed: disk offline");

let wrapped = WrappedDomainError::from(err);
assert_eq!(wrapped.to_string(), "I/O failed: disk offline");
~~~

- `use masterror::Error;` brings the crate's derive macro into scope.
- `#[from]` automatically implements `From<...>` while ensuring wrapper shapes are
  valid.
- `#[error(transparent)]` enforces single-field wrappers that forward
  `Display`/`source` to the inner error.
- `masterror::error::template::ErrorTemplate` parses `#[error("...")]`
  strings, exposing literal and placeholder segments so custom derives can be
  implemented without relying on `thiserror`.

```rust
use masterror::error::template::{ErrorTemplate, TemplateIdentifier};

let template = ErrorTemplate::parse("{code}: {message}").expect("parse");
let display = template.display_with(|placeholder, f| match placeholder.identifier() {
    TemplateIdentifier::Named("code") => write!(f, "{}", 404),
    TemplateIdentifier::Named("message") => f.write_str("Not Found"),
    _ => Ok(()),
});

assert_eq!(display.to_string(), "404: Not Found");
```

</details>

<details>
  <summary><b>Error response payload</b></summary>

~~~rust
use masterror::{AppError, AppErrorKind, AppCode, ErrorResponse};
use std::time::Duration;

let app_err = AppError::new(AppErrorKind::Unauthorized, "Token expired");
let resp: ErrorResponse = (&app_err).into()
    .with_retry_after_duration(Duration::from_secs(30))
    .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#);

assert_eq!(resp.status, 401);
~~~

</details>

<details>
  <summary><b>Web framework integrations</b></summary>

<details>
  <summary>Axum</summary>

~~~rust
// features = ["axum", "serde_json"]
...
    assert!(payload.is_object());

    #[cfg(target_arch = "wasm32")]
    {
        if let Err(console_err) = err.log_to_browser_console() {
            eprintln!(
                "failed to log to browser console: {:?}",
                console_err.context()
            );
        }
    }

    Ok(())
}
~~~

- On non-WASM targets `log_to_browser_console` returns
  `BrowserConsoleError::UnsupportedTarget`.
- `BrowserConsoleError::context()` exposes optional browser diagnostics for
  logging/telemetry when console logging fails.

</details>

<details>
  <summary><b>Feature flags</b></summary>

- `axum` — IntoResponse integration with structured JSON bodies
- `actix` — Actix Web ResponseError and Responder implementations
- `openapi` — Generate utoipa OpenAPI schema for ErrorResponse
- `serde_json` — Attach structured JSON details to AppError
- `sqlx` — Classify sqlx::Error variants into AppError kinds
- `reqwest` — Classify reqwest::Error as timeout/network/external API
- `redis` — Map redis::RedisError into cache-aware AppError
- `validator` — Convert validator::ValidationErrors into validation failures
- `config` — Propagate config::ConfigError as configuration issues
- `tokio` — Classify tokio::time::error::Elapsed as timeout
- `multipart` — Handle axum multipart extraction errors
- `teloxide` — Convert teloxide_core::RequestError into domain errors
- `telegram-webapp-sdk` — Surface Telegram WebApp validation failures
- `frontend` — Log to the browser console and convert to JsValue on WASM
- `turnkey` — Ship Turnkey-specific error taxonomy and conversions

</details>

<details>
  <summary><b>Conversions</b></summary>

- `std::io::Error` → Internal
- `String` → BadRequest
- `sqlx::Error` → NotFound/Database
- `redis::RedisError` → Cache
- `reqwest::Error` → Timeout/Network/ExternalApi
- `axum::extract::multipart::MultipartError` → BadRequest
- `validator::ValidationErrors` → Validation
- `config::ConfigError` → Config
- `tokio::time::error::Elapsed` → Timeout
- `teloxide_core::RequestError` → RateLimited/Network/ExternalApi/Deserialization/Internal
- `telegram_webapp_sdk::utils::validate_init_data::ValidationError` → TelegramAuth

</details>

<details>
  <summary><b>Typical setups</b></summary>

Minimal core:

~~~toml
masterror = { version = "0.5.1", default-features = false }
~~~

API (Axum + JSON + deps):

~~~toml
masterror = { version = "0.5.1", features = [
  "axum", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
~~~

API (Actix + JSON + deps):

~~~toml
masterror = { version = "0.5.1", features = [
  "actix", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
~~~

</details>

<details>
  <summary><b>Turnkey</b></summary>

~~~rust
// features = ["turnkey"]
use masterror::turnkey::{classify_turnkey_error, TurnkeyError, TurnkeyErrorKind};
use masterror::{AppError, AppErrorKind};

// Classify a raw SDK/provider error
let kind = classify_turnkey_error("429 Too Many Requests");
assert!(matches!(kind, TurnkeyErrorKind::RateLimited));

// Wrap into AppError
let e = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "throttled upstream");
let app: AppError = e.into();
assert_eq!(app.kind, AppErrorKind::RateLimited);
~~~

</details>

<details>
  <summary><b>Migration 0.2 → 0.3</b></summary>

- Use `ErrorResponse::new(status, AppCode::..., "msg")` instead of legacy
- New helpers: `.with_retry_after_secs`, `.with_retry_after_duration`, `.with_www_authenticate`
- `ErrorResponse::new_legacy` is temporary shim

</details>

<details>
  <summary><b>Versioning & MSRV</b></summary>

Semantic versioning. Breaking API/wire contract → major bump.
MSRV = 1.90 (may raise in minor, never in patch).

</details>

<details>
  <summary><b>Release checklist</b></summary>

1. `cargo +nightly fmt --`
1. `cargo clippy -- -D warnings`
1. `cargo test --all`
1. `cargo build` (regenerates README.md from the template)
1. `cargo doc --no-deps`
1. `cargo package --locked`

</details>

<details>
  <summary><b>Non-goals</b></summary>

- Not a general-purpose error aggregator like `anyhow`
- Not a replacement for your domain errors

</details>

<details>
  <summary><b>License</b></summary>

Apache-2.0 OR MIT, at your option.

</details>
