<div align="center">

# masterror

**Framework-agnostic application error types with stable codes, HTTP/gRPC mappings and built-in telemetry**

[![English](https://img.shields.io/badge/🇬🇧_English-blue?style=for-the-badge)](#)
[![Русский](https://img.shields.io/badge/🇷🇺_Русский-gray?style=for-the-badge)](Главная)
[![한국어](https://img.shields.io/badge/🇰🇷_한국어-gray?style=for-the-badge)](홈)

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.96-blue)
![License](https://img.shields.io/badge/License-MIT-informational)

</div>

---

## What is masterror?

`masterror` is an error-handling workspace for Rust services that need more than `Display` and `source()`. Where `thiserror` stops at deriving trait implementations and `anyhow` stops at type-erased propagation, `masterror` carries an error all the way to the transport boundary:

- **`AppError`** — a rich error value with a semantic category (`AppErrorKind`), a stable machine-readable code (`AppCode`), an optional safe public message, structured metadata and transport hints (`Retry-After`, `WWW-Authenticate`).
- **Conservative HTTP and gRPC mappings** — every kind and code maps deterministically to an HTTP status, a `tonic::Code` discriminant and an RFC 7807 `type` URI.
- **Typed telemetry** — metadata is stored as typed fields (strings, integers, floats, durations, IPs, UUIDs, JSON) with per-field redaction policies, not ad-hoc `String` maps.
- **Native derives** — `#[derive(Error)]` mirrors `thiserror` syntax, while `#[app_error(...)]` and `#[derive(Masterror)]` wire domain errors into `AppError` with codes, categories, redaction and mapping tables.
- **Redaction by design** — sources are never serialized to clients; messages, details and metadata fields can be redacted, hashed or masked at the boundary.

No `unsafe`, pinned MSRV, `no_std` support with the default `std` feature disabled.

## The problem it solves

| Concern | `thiserror` | `anyhow` | `masterror` |
|---|---|---|---|
| `Display` / `source()` derives | Yes | — | Yes (same syntax) |
| Type-erased propagation with context | — | Yes | Yes (`.ctx()` / `.context()`) |
| Stable machine-readable error codes | Manual | Manual | `AppCode`, part of the wire contract |
| HTTP status mapping | Manual | Manual | `AppErrorKind::http_status()`, stable table |
| gRPC status mapping | Manual | Manual | `CODE_MAPPINGS`, `tonic::Status` conversion |
| RFC 7807 `problem+json` | Manual | Manual | `ProblemJson::from_app_error` |
| Structured, typed metadata | — | — | `Metadata` + `field::*` builders |
| Redaction of secrets at the boundary | — | — | `MessageEditPolicy`, `FieldRedaction` |
| tracing / metrics / backtrace emission | — | — | Feature-gated, automatic on construction |

A `thiserror`-derived enum tells you *what happened*. `masterror` also decides *what the client sees* (status, code, safe message, problem+json), *what operators see* (structured fields, tracing events, counters) and *what never leaks* (sources, redacted fields).

## Feature highlights

| Area | What you get |
|---|---|
| Core taxonomy | `AppError`, `AppErrorKind` (23 stable categories), `AppCode` (SCREAMING_SNAKE_CASE codes, custom codes supported), `AppResult<T>` |
| Derives | `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error(...)]`, `#[masterror(...)]`, `#[provide(...)]` telemetry providers |
| Control flow | `ensure!` / `fail!` — typed, allocation-free early returns |
| Context | `ResultExt::ctx` / `ResultExt::context`, `Context` builder with caller tracking |
| Wire payloads | `ErrorResponse` (legacy JSON), `ProblemJson` (RFC 7807) with retry and auth hints |
| Transports | Axum `IntoResponse`, Actix `ResponseError`/`Responder`, `tonic::Status`, WASM `JsValue`, OpenAPI schema |
| Integrations | `sqlx`, `redis`, `reqwest`, `validator`, `config`, `tokio`, `teloxide`, Telegram Mini Apps init data, Turnkey |
| Observability | `tracing` events, `metrics` counters, lazy `backtrace` capture, colored terminal output, `DisplayMode` (prod/staging/local) |

## Quick example

```rust
use masterror::{AppError, AppErrorKind, AppResult, ProblemJson, field};

fn find_user(id: u64) -> AppResult<()> {
    masterror::ensure!(id != 0, AppError::bad_request("id must be non-zero"));

    Err(AppError::not_found("user not found")
        .with_field(field::u64("user_id", id))
        .with_field(field::str("request_id", "abc123")))
}

let err = find_user(42).unwrap_err();
assert_eq!(err.kind, AppErrorKind::NotFound);
assert_eq!(err.kind.http_status(), 404);

let problem = ProblemJson::from_app_error(err);
assert_eq!(problem.status, 404);
assert_eq!(problem.code.as_str(), "NOT_FOUND");
assert_eq!(problem.grpc.expect("grpc").name, "NOT_FOUND");
```

Or declare a domain error once and let the derive handle the mapping:

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("missing flag: {name}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
struct MissingFlag {
    name: &'static str
}

let app: AppError = MissingFlag { name: "feature" }.into();
assert!(matches!(app.kind, AppErrorKind::BadRequest));
```

## Workspace crates

| Crate | Role |
|---|---|
| [`masterror`](https://crates.io/crates/masterror) | Core error types, metadata, transports, integrations, prelude |
| [`masterror-derive`](https://crates.io/crates/masterror-derive) | Proc-macros behind `#[derive(Error)]` and `#[derive(Masterror)]` (pulled in automatically) |
| [`masterror-template`](https://crates.io/crates/masterror-template) | Shared `#[error("...")]` template parser |

## Documentation

**Getting started**

- [Getting Started](Getting-Started-en) — installation, first errors, macros, first derive
- [Feature Flags](Feature-Flags-en) — complete flag reference with dependencies

**Core concepts**

- [Error Kinds and Codes](Error-Kinds-and-Codes-en) — taxonomy, HTTP/gRPC tables, problem+json
- [Derive Macros](Derive-Macros-en) — `#[derive(Error)]`, `#[derive(Masterror)]` and their attributes
- [Context and Metadata](Context-and-Metadata-en) — `Context`, `ResultExt`, fields, redaction, chains

**Integrations**

- [Web Frameworks](Web-Frameworks-en) — Axum, Actix, tonic
- [Integrations](Integrations-en) — sqlx, redis, reqwest, validator and friends
- [Observability](Observability-en) — tracing, metrics, backtraces, display modes

**Advanced**

- [No-Std](No-Std-en) — running without the standard library
- [Best Practices](Best-Practices-en) — patterns for services and libraries
- [Migration](Migration-en) — moving from `thiserror` / `anyhow`
