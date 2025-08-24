# masterror · Framework-agnostic application error types

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.89-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml)

Small, pragmatic error model for API-heavy Rust services.  
Core is framework-agnostic; integrations are opt-in via feature flags.  
Stable categories, conservative HTTP mapping, no `unsafe`.

- Core types: `AppError`, `AppErrorKind`, `AppResult`, `AppCode`, `ErrorResponse`
- Optional Axum/Actix integration
- Optional OpenAPI schema (via `utoipa`)
- Conversions from `sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio`

---

## Why this crate?

- **Stable, predictable taxonomy.** A small set of error categories (`AppErrorKind`) that map conservatively to HTTP. Easy to reason about, safe to expose, and consistent across services.
- **Framework-agnostic core.** No web framework assumptions. No `unsafe`. MSRV pinned. Works in libraries and binaries alike.
- **Opt-in integrations.** Zero default features. You pull only what you need:
  - `axum` (HTTP `IntoResponse`)
  - `actix` (ready-to-use integration)
  - `serde_json` (JSON details)
  - `openapi` (schemas via `utoipa`)
  - `sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio`, `multipart` (error conversions)
- **Clean wire contract.** A small `ErrorResponse { status, code, message, details?, retry?, www_authenticate? }` payload for HTTP, with optional OpenAPI schema. No leaking of internal sources.
- **One log at the boundary.** Use `tracing` once when converting to HTTP, avoiding duplicate logs and keeping fields stable (`kind`, `status`, `message`).
- **Less boilerplate.** Built-in `From<...>` conversions for common libs and a compact prelude for handler signatures.
- **Consistent across a workspace.** Share the same error surface between services and crates, making clients and tests simpler.

> *Since v0.3.0: stable AppCode enum and extended ErrorResponse with retry/authentication metadata*


---

## Installation

```toml
[dependencies]
# lean core, no extra deps
masterror = { version = "0.3", default-features = false }

# Or with features:
# JSON + Axum/Actix + common integrations
# masterror = { version = "0.3", features = [
#   "axum", "actix", "serde_json", "openapi",
#   "sqlx", "reqwest", "redis", "validator", "config", "tokio"
# ] }
```

**MSRV:** 1.89  
**No unsafe:** this crate forbids `unsafe`.

---

## Quick start

Create an error with a semantic kind and an optional public message:

```rust
use masterror::{AppError, AppErrorKind};

let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
```

Use the prelude to keep signatures tidy:

```rust
use masterror::prelude::*;

fn do_work(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("Flag must be set"));
    }
    Ok(())
}
```

### Error response payload

`ErrorResponse` is a wire-level payload for HTTP APIs. You can build it directly or convert from `AppError`:


```rust
use masterror::{AppError, AppErrorKind, AppCode, ErrorResponse};

let app_err = AppError::new(AppErrorKind::Unauthorized, "Token expired");
let resp: ErrorResponse = (&app_err).into()
    .with_retry_after_secs(30)
    .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#);

assert_eq!(resp.status, 401);
```


---

## Web framework integrations

### Axum

Enable `axum` (and usually `serde_json`) to return errors directly from handlers:

```rust
// requires: features = ["axum", "serde_json"]
use masterror::{AppError, AppResult};
use axum::{routing::get, Router};

async fn handler() -> AppResult<&'static str> {
    Err(AppError::forbidden("No access"))
}

let app = Router::new().route("/demo", get(handler));
```

### Actix

Enable `actix` (and usually `serde_json`) to return errors directly from handlers:

```rust
// requires: features = ["actix", "serde_json"]
use actix_web::{get, App, HttpServer, Responder};
use masterror::prelude::*;

#[get("/err")]
async fn err() -> AppResult<&'static str> {
    Err(AppError::forbidden("No access"))
}


#[get("/payload")]
async fn payload() -> impl Responder {
    ErrorResponse::new(422, AppCode::Validation, "Validation failed")
}

```

---

## OpenAPI

Enable `openapi` to derive an OpenAPI schema for `ErrorResponse` (via `utoipa`).

```toml
[dependencies]
masterror = { version = "0.3", features = ["openapi", "serde_json"] }
utoipa = "5"
```

---

## Feature flags

- `axum` — `IntoResponse` for `AppError` and JSON responses  
- `actix` — `ResponseError`/`Responder` integration  
- `openapi` — schema for `ErrorResponse` via `utoipa`  
- `serde_json` — JSON details support  
- `sqlx` — `From<sqlx::Error>`  
- `redis` — `From<redis::RedisError>`  
- `validator` — `From<validator::ValidationErrors>`  
- `config` — `From<config::ConfigError>`  
- `tokio` — `From<tokio::time::error::Elapsed>`  
- `reqwest` — `From<reqwest::Error>`  
- `multipart` — compatibility flag for projects using multipart in Axum  

---

## Conversions

All mappings are conservative and avoid leaking internals:

- `std::io::Error` → `Internal`  
- `String` → `BadRequest`  
- `sqlx::Error` → `NotFound` (for `RowNotFound`) or `Database`  
- `redis::RedisError` → `Service`  
- `reqwest::Error` → `Timeout` / `Network` / `ExternalApi`  
- `validator::ValidationErrors` → `Validation`  
- `config::ConfigError` → `Config`  
- `tokio::time::error::Elapsed` → `Timeout`  

---

## Typical setups

Minimal core:

```toml
masterror = { version = "0.3", default-features = false }
```

API service (Axum + JSON + common deps):

```toml
masterror = { version = "0.3", features = [
  "axum", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
```

API service (Actix + JSON + common deps):

```toml
masterror = { version = "0.3", features = [
  "actix", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
```

---

## Migration from 0.2.x to 0.3.0

- Replace `ErrorResponse::new(status, "msg")` with
  `ErrorResponse::new(status, AppCode::<Variant>, "msg")`
- Use `.with_retry_after_secs(...)` and `.with_www_authenticate(...)`
  if you want to surface HTTP headers.
- `ErrorResponse::new_legacy` is provided temporarily as a deprecated shim.


---

## Versioning and MSRV

- Semantic versioning. Breaking API or wire-contract changes bump the major version.  
- MSRV: 1.89 (may be raised in a **minor** release with a changelog note, never in a patch).  

---

## Non-goals

- Not a general-purpose error aggregator like `anyhow` for CLIs.  
- Not a replacement for your domain errors. Use it as the public API surface and transport mapping.  

---

## License

Licensed under either of

- Apache License, Version 2.0  
- MIT license  

at your option.
