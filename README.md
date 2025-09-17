# masterror · Framework-agnostic application error types

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.89-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)

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
masterror = { version = "0.4", default-features = false }
# or with features:
# masterror = { version = "0.4", features = [
#   "axum", "actix", "serde_json", "openapi",
#   "sqlx", "reqwest", "redis", "validator", "config", "tokio", "teloxide"
# ] }
~~~

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
- **Less boilerplate.** Built-in conversions, compact prelude.
- **Consistent workspace.** Same error surface across crates.

</details>

<details>
  <summary><b>Installation</b></summary>

~~~toml
[dependencies]
# lean core
masterror = { version = "0.4", default-features = false }

# with Axum/Actix + JSON + integrations
# masterror = { version = "0.4", features = [
#   "axum", "actix", "serde_json", "openapi",
#   "sqlx", "reqwest", "redis", "validator", "config", "tokio", "teloxide"
# ] }
~~~

**MSRV:** 1.89  
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
use masterror::{AppError, AppResult};
use axum::{routing::get, Router};

async fn handler() -> AppResult<&'static str> {
    Err(AppError::forbidden("No access"))
}

let app = Router::new().route("/demo", get(handler));
~~~

</details>

<details>
  <summary>Actix</summary>

~~~rust
// features = ["actix", "serde_json"]
use actix_web::{get, App, HttpServer, Responder};
use masterror::prelude::*;

#[get("/err")]
async fn err() -> AppResult<&'static str> {
    Err(AppError::forbidden("No access"))
}

#[get("/payload")]
async fn payload() -> impl Responder {
    ErrorResponse::new(422, AppCode::Validation, "Validation failed")
        .expect("status")
}
~~~

</details>

</details>

<details>
  <summary><b>OpenAPI</b></summary>

~~~toml
[dependencies]
masterror = { version = "0.4", features = ["openapi", "serde_json"] }
utoipa = "5"
~~~

</details>

<details>
  <summary><b>Browser (WASM)</b></summary>

~~~rust
// features = ["frontend"]
use masterror::{AppError, AppErrorKind, AppResult};
use masterror::frontend::{BrowserConsoleError, BrowserConsoleExt};

fn report() -> AppResult<(), BrowserConsoleError> {
    let err = AppError::bad_request("missing field");
    let payload = err.to_js_value()?;
    assert!(payload.is_object());

    #[cfg(target_arch = "wasm32")]
    err.log_to_browser_console()?;

    Ok(())
}
~~~

- On non-WASM targets `log_to_browser_console` returns
  `BrowserConsoleError::UnsupportedTarget`.

</details>

<details>
  <summary><b>Feature flags</b></summary>

- `axum` — IntoResponse
- `actix` — ResponseError/Responder
- `openapi` — utoipa schema
- `serde_json` — JSON details
- `sqlx`, `redis`, `reqwest`, `validator`, `config`, `tokio`, `multipart`, `teloxide`, `telegram-webapp-sdk`
- `frontend` — convert errors into `JsValue` and log via `console.error` (WASM)
- `turnkey` — domain taxonomy and conversions for Turnkey errors

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
masterror = { version = "0.4", default-features = false }
~~~

API (Axum + JSON + deps):

~~~toml
masterror = { version = "0.4", features = [
  "axum", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
~~~

API (Actix + JSON + deps):

~~~toml
masterror = { version = "0.4", features = [
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
MSRV = 1.89 (may raise in minor, never in patch).

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

---

[![Open Issues](https://img.shields.io/github/issues/RAprogramm/masterror?label=Open%20Issues&color=informational)](https://github.com/RAprogramm/masterror/issues)
[![Closed Issues](https://img.shields.io/github/issues-closed/RAprogramm/masterror?label=Closed%20Issues&color=success)](https://github.com/RAprogramm/masterror/issues?q=is%3Aissue+is%3Aclosed)
[![Open PRs](https://img.shields.io/github/issues-pr/RAprogramm/masterror?label=Open%20PRs&color=blueviolet)](https://github.com/RAprogramm/masterror/pulls)
[![Closed PRs](https://img.shields.io/github/issues-pr-closed/RAprogramm/masterror?label=Closed%20PRs&color=success)](https://github.com/RAprogramm/masterror/pulls?q=is%3Apr+is%3Aclosed)
[![Last Commit](https://img.shields.io/github/last-commit/RAprogramm/masterror?color=yellowgreen&label=Last%20Commit)](https://github.com/RAprogramm/masterror/commits/main)
[![Repo Size](https://img.shields.io/github/repo-size/RAprogramm/masterror?label=Repo%20Size)](https://github.com/RAprogramm/masterror)
[![License](https://img.shields.io/github/license/RAprogramm/masterror)](./LICENSE)
[![Contributors](https://img.shields.io/github/contributors/RAprogramm/masterror)](https://github.com/RAprogramm/masterror/graphs/contributors)

[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml)

---

## 📊 Project Insights

![Activity Graph](https://github-readme-activity-graph.vercel.app/graph?username=RAprogramm&repo=masterror&theme=github)
