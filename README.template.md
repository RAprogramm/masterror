# masterror · Framework-agnostic application error types

<!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
     CI packaging will fail if README.md is stale. -->

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-{{MSRV}}-blue)
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
masterror = { version = "{{CRATE_VERSION}}", default-features = false }
# or with features:
# masterror = { version = "{{CRATE_VERSION}}", features = [
{{FEATURE_SNIPPET}}
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
- **Less boilerplate.** Built-in conversions, compact prelude,
  derive macro support for transparent wrappers via `#[error(transparent)]`.
- **Consistent workspace.** Same error surface across crates.

</details>

<details>
  <summary><b>Installation</b></summary>

~~~toml
[dependencies]
# lean core
masterror = { version = "{{CRATE_VERSION}}", default-features = false }

# with Axum/Actix + JSON + integrations
# masterror = { version = "{{CRATE_VERSION}}", features = [
{{FEATURE_SNIPPET}}
# ] }
~~~

**MSRV:** {{MSRV}}
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
...
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

{{FEATURE_BULLETS}}

</details>

<details>
  <summary><b>Conversions</b></summary>

{{CONVERSION_BULLETS}}

</details>

<details>
  <summary><b>Typical setups</b></summary>

Minimal core:

~~~toml
masterror = { version = "{{CRATE_VERSION}}", default-features = false }
~~~

API (Axum + JSON + deps):

~~~toml
masterror = { version = "{{CRATE_VERSION}}", features = [
  "axum", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
~~~

API (Actix + JSON + deps):

~~~toml
masterror = { version = "{{CRATE_VERSION}}", features = [
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
MSRV = {{MSRV}} (may raise in minor, never in patch).

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
