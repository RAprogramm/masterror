# Feature Flags

`masterror` keeps the default build lean: only `std` is enabled out of the box. Everything else — web transports, telemetry, library integrations — is opt-in. This page is the complete reference for every flag declared in `Cargo.toml`.

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }
# or with features:
# masterror = { version = "0.28", features = [
#   "std", "axum", "actix", "openapi",
#   "serde_json", "tracing", "metrics", "backtrace",
#   "colored", "sqlx", "sqlx-migrate", "reqwest",
#   "redis", "validator", "config", "tokio",
#   "multipart", "teloxide", "init-data", "tonic",
#   "frontend", "turnkey", "benchmarks"
# ] }
```

## Core

| Flag | What it enables | Extra deps |
|---|---|---|
| `std` *(default)* | Standard library support; required by all runtime integrations. Disable for `no_std` (see [No-Std](No-Std-en)) | — |

## Web transports

| Flag | What it enables | Extra deps |
|---|---|---|
| `axum` | `IntoResponse` for `AppError` and `ProblemJson` with RFC 7807 JSON bodies; `AppErrorKind::status_code()` | `axum` (json, multipart), `serde_json` |
| `actix` | Actix Web `ResponseError` for `AppError` and `Responder` for `ProblemJson` | `actix-web` |
| `multipart` | Maps `axum::extract::multipart::MultipartError` → `BadRequest` (implies `axum`) | via `axum` |
| `openapi` | `utoipa::ToSchema` for `ErrorResponse` and `AppCode` so error payloads appear in OpenAPI specs | `utoipa` |
| `serde_json` | Structured JSON `details` on `AppError`/`ErrorResponse`/`ProblemJson`; `FieldValue::Json` and `field::json` | `serde_json` |
| `tonic` | Conversion of errors into `tonic::Status` with sanitized metadata; exports `StatusConversionError` | `tonic` |

## Telemetry and observability

| Flag | What it enables | Extra deps |
|---|---|---|
| `tracing` | Structured `tracing` events emitted when errors are constructed | `tracing`, `log`, `log-mdc` |
| `metrics` | Increments an `error_total{code,category}` counter for each `AppError` | `metrics` |
| `backtrace` | Lazy `std::backtrace::Backtrace` capture (honours `RUST_BACKTRACE`), `with_backtrace()` builder | — |
| `colored` | Colored multi-line terminal output with automatic TTY detection; richer `Display` for `AppError` | `owo-colors` |

## Async and IO integrations

Each integration flag adds a `From<...> for AppError` conversion that classifies the library error into the taxonomy:

| Flag | Conversion | Extra deps |
|---|---|---|
| `sqlx` | `sqlx_core::Error` → `NotFound` / `Database` (lean `sqlx-core`, no drivers or TLS) | `sqlx-core` |
| `sqlx-migrate` | `sqlx::migrate::MigrateError` → `Database` (full `sqlx` with `migrate` feature only) | `sqlx` |
| `redis` | `redis::RedisError` → `Cache` | `redis` |
| `reqwest` | `reqwest::Error` → `Timeout` / `Network` / `ExternalApi` | `reqwest` |
| `tokio` | `tokio::time::error::Elapsed` → `Timeout` | `tokio` (time) |
| `validator` | `validator::ValidationErrors` → `Validation` | `validator` |
| `config` | `config::ConfigError` → `Config` | `config` |

`sqlx` and `sqlx-migrate` are split deliberately: error classification only needs `sqlx-core`, while migration error mapping pulls the full `sqlx` crate.

## Messaging and bots

| Flag | Conversion | Extra deps |
|---|---|---|
| `teloxide` | `teloxide_core::RequestError` → `RateLimited` / `Network` / `ExternalApi` / `Deserialization` / `Internal` | `teloxide-core` |
| `init-data` | `init_data_rs::InitDataError` → `TelegramAuth` (Telegram Mini Apps init-data validation) | `init-data-rs` |

## Front-end and domain

| Flag | What it enables | Extra deps |
|---|---|---|
| `frontend` | `frontend` module: convert errors to `wasm_bindgen::JsValue` and emit `console.error` logs in WASM/browser contexts | `wasm-bindgen`, `js-sys`, `serde-wasm-bindgen` |
| `turnkey` | `turnkey` module: `TurnkeyErrorKind`, `TurnkeyError`, `classify_turnkey_error` and conversions into `AppError` | — |
| `benchmarks` | Criterion benchmark suite and CI baseline tooling (local profiling only) | — |

## Baseline conversions (always available)

Without any feature flag, `AppError` already converts from:

| Source | Target kind |
|---|---|
| `std::io::Error` | `Internal` |
| `String` | `BadRequest` |

## Recipes

REST API on Axum with problem+json, OpenAPI docs and tracing:

```toml
masterror = { version = "0.28", features = ["axum", "openapi", "serde_json", "tracing"] }
```

Database service with sqlx, migrations and metrics:

```toml
masterror = { version = "0.28", features = ["sqlx", "sqlx-migrate", "metrics", "tracing"] }
```

gRPC service:

```toml
masterror = { version = "0.28", features = ["tonic", "tracing", "backtrace"] }
```

Telegram bot with Mini App auth:

```toml
masterror = { version = "0.28", features = ["teloxide", "init-data", "reqwest", "tokio"] }
```

WASM front-end:

```toml
masterror = { version = "0.28", features = ["frontend", "serde_json"] }
```

`no_std` embedded or library target:

```toml
masterror = { version = "0.28", default-features = false }
```

## Notes

- All integration flags imply `std` except `sqlx` and `sqlx-migrate`, which stay `std`-independent at the flag level.
- `axum` and `actix` pull `serde_json` transitively because their response bodies are JSON.
- Feature flags never change the wire contract of `ErrorResponse`/`ProblemJson` fields that are already enabled — they only add capabilities (e.g. `serde_json` upgrades `details` from plain text to structured JSON) or trait implementations.

---

See also: [Getting Started](Getting-Started-en) · [Web Frameworks](Web-Frameworks-en) · [Integrations](Integrations-en) · [Observability](Observability-en) · [No-Std](No-Std-en)
