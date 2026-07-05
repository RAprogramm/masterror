# Getting Started

This page walks through installing `masterror`, returning your first `AppError`, short-circuiting with `ensure!`/`fail!`, using the prelude and writing your first derive.

## Installation

The default build enables only the `std` feature — no web framework, no telemetry backends:

```toml
[dependencies]
masterror = "0.28"
```

Enable integrations as you need them (see [Feature Flags](Feature-Flags-en) for the full list):

```toml
[dependencies]
masterror = { version = "0.28", features = ["axum", "serde_json", "tracing"] }
```

MSRV is **1.96**. The crate forbids `unsafe` and supports `no_std` when built with `default-features = false`.

## Your first error

`AppError` couples a semantic category (`AppErrorKind`) with an optional public message. `AppResult<T>` is an alias for `Result<T, AppError>`:

```rust
use masterror::{AppError, AppErrorKind, AppResult};

fn do_work(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::new(AppErrorKind::BadRequest, "Flag must be set"));
    }
    Ok(())
}

let err = do_work(false).unwrap_err();
assert!(matches!(err.kind, AppErrorKind::BadRequest));
assert_eq!(err.kind.http_status(), 400);
```

Every kind has a named constructor, so you rarely spell out `AppErrorKind` at call sites:

```rust
use masterror::AppError;

let _ = AppError::not_found("user not found");        // 404
let _ = AppError::validation("invalid email");        // 422
let _ = AppError::unauthorized("token expired");      // 401
let _ = AppError::forbidden("no access");             // 403
let _ = AppError::conflict("already exists");         // 409
let _ = AppError::rate_limited("slow down");          // 429
let _ = AppError::internal("unexpected failure");     // 500
let _ = AppError::service("orchestration failed");    // 500
let _ = AppError::timeout("upstream timed out");      // 504
let _ = AppError::bare(masterror::AppErrorKind::NotFound); // no message
```

Attach structured metadata and an upstream source without giving up typing:

```rust
use masterror::{AppError, field};

let err = AppError::service("downstream degraded")
    .with_field(field::str("request_id", "abc123"))
    .with_field(field::i64("attempt", 2))
    .with_context(std::io::Error::other("connection reset"));

assert_eq!(err.metadata().len(), 2);
assert!(err.source_ref().is_some());
```

The source is available for logs and `chain()` traversal but is **never serialized to clients**.

## ensure! and fail!

`ensure!` and `fail!` are typed alternatives to `anyhow::ensure!`/`anyhow::bail!`. The error expression is evaluated lazily, so the success path performs no formatting and no allocation:

```rust
use masterror::{AppError, AppErrorKind, AppResult};

fn guard(flag: bool) -> AppResult<()> {
    masterror::ensure!(flag, AppError::bad_request("flag must be set"));
    Ok(())
}

fn bail() -> AppResult<()> {
    masterror::fail!(AppError::unauthorized("token expired"));
}

assert!(guard(true).is_ok());
assert!(matches!(guard(false).unwrap_err().kind, AppErrorKind::BadRequest));
assert!(matches!(bail().unwrap_err().kind, AppErrorKind::Unauthorized));
```

`ensure!` also accepts a verbose form for complex conditions:

```rust
use masterror::{AppError, AppResult};

fn bounded(value: i32, max: i32) -> AppResult<()> {
    masterror::ensure!(
        cond = value <= max,
        else = AppError::service("value too large")
    );
    Ok(())
}
```

## The prelude

`masterror::prelude` re-exports just the core types (`AppError`, `AppErrorKind`, `AppCode`, `AppResult`, `ErrorResponse`, plus the `turnkey` helpers when that feature is on):

```rust
use masterror::prelude::*;

fn handler(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("Flag must be set"));
    }
    Ok(())
}
```

Framework trait implementations (Axum `IntoResponse`, Actix `Responder`) are activated by feature flags and need no extra imports.

## Adding context to foreign errors

`ResultExt` promotes any `Result<T, E: Error>` into `AppResult<T>`:

```rust
use masterror::{AppErrorKind, Context, ResultExt, field};

fn read_config() -> Result<String, std::io::Error> {
    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}

// Simple, anyhow-style message:
let err = read_config().context("Failed to read config file").unwrap_err();
assert!(err.source_ref().is_some());

// Full control over category, code, metadata and redaction:
let err = read_config()
    .ctx(|| Context::new(AppErrorKind::Config).with(field::str("path", "app.toml")))
    .unwrap_err();
assert_eq!(err.kind, AppErrorKind::Config);
```

See [Context and Metadata](Context-and-Metadata-en) for the full `Context` API.

## Your first derive

`#[derive(Error)]` mirrors `thiserror` syntax, and `#[app_error(...)]` adds the conversion into `AppError`:

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("I/O failed: {source}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal, message)]
pub struct DomainError {
    #[from]
    #[source]
    source: std::io::Error
}

fn load() -> Result<(), DomainError> {
    Err(std::io::Error::other("disk offline").into())
}

let err = load().unwrap_err();
assert_eq!(err.to_string(), "I/O failed: disk offline");

let app: AppError = err.into();
assert!(matches!(app.kind, AppErrorKind::Internal));
```

- `#[error("...")]` defines the `Display` template with `{field}` placeholders.
- `#[from]` generates `From<std::io::Error>` for the wrapper.
- `#[source]` forwards the inner error through `source()`.
- `#[app_error(kind = ..., code = ..., message)]` generates `From<DomainError> for AppError` (and `for AppCode`); the `message` flag exposes the `Display` output as the public message.

Enums work the same way with per-variant `#[error]` and `#[app_error]` attributes. When you also need metadata, redaction policy and gRPC/problem+json mapping tables, reach for `#[derive(Masterror)]` — covered in [Derive Macros](Derive-Macros-en).

## Where to go next

- Map errors to HTTP responses in Axum or Actix — [Web Frameworks](Web-Frameworks-en)
- Understand the taxonomy and wire contract — [Error Kinds and Codes](Error-Kinds-and-Codes-en)
- Enable integrations for sqlx, redis, reqwest — [Feature Flags](Feature-Flags-en)

---

See also: [Feature Flags](Feature-Flags-en) · [Error Kinds and Codes](Error-Kinds-and-Codes-en) · [Derive Macros](Derive-Macros-en) · [Context and Metadata](Context-and-Metadata-en) · [Migration](Migration-en)
