# Integrations

Optional feature flags add `From<...>` conversions from popular third-party
error types into `masterror::Error`, so a single `?` at the call site produces
a classified error with structured metadata. Every conversion picks a stable
[`AppErrorKind`](Error-Kinds-and-Codes-en) and attaches
telemetry fields (never secrets) for observability.

## Conversion matrix

| Feature | Source type | Resulting `AppErrorKind` |
|---|---|---|
| `sqlx` | `sqlx_core::error::Error` | `NotFound`, `Conflict`, `Validation`, `Timeout`, `DependencyUnavailable`, `Config`, `BadRequest`, `Serialization`, `Deserialization`, `Network`, `Database`, `Internal` |
| `sqlx-migrate` | `sqlx::migrate::MigrateError` | `Database` (with migration phase metadata) |
| `redis` | `redis::RedisError` | `Cache` (default), `Timeout`, `DependencyUnavailable` |
| `reqwest` | `reqwest::Error` | `Timeout`, `Network`, `RateLimited`, `DependencyUnavailable`, `ExternalApi` |
| `validator` | `validator::ValidationErrors` | `Validation` |
| `config` | `config::ConfigError` | `Config` (with `config.phase` metadata) |
| `tokio` | `tokio::time::error::Elapsed` | `Timeout` |
| `serde_json` | `serde_json::Error` | `Serialization` (I/O), `Deserialization` (syntax/data/EOF) |
| `teloxide` | `teloxide_core::RequestError` | `ExternalApi`, `Unauthorized`, `RateLimited`, `Network`, `Deserialization`, `Internal` |
| `init-data` | `init_data_rs::InitDataError` | `TelegramAuth` |
| `tonic` | `masterror::Error` → `tonic::Status` | outbound mapping, see below |
| `multipart` | `axum::extract::multipart::MultipartError` | `BadRequest` (see [Web Frameworks](Web-Frameworks-en)) |

## sqlx and sqlx-migrate

`sqlx` depends only on `sqlx-core` (no drivers, no TLS). Key mappings:

- `Error::RowNotFound` → `NotFound`
- Pool timeout → `Timeout`; pool closed and I/O failures →
  `DependencyUnavailable`; TLS errors → `Network`
- Constraint violations are classified by `sqlx` error kind: unique and
  foreign-key violations → `Conflict`, not-null / check violations →
  `Validation`, anything else → `Database`
- Encode → `Serialization`, decode → `Deserialization`

Database errors capture SQLSTATE and constraint names as metadata. Known
SQLSTATE codes override the public [`AppCode`](Error-Kinds-and-Codes-en):
`23505` → `USER_ALREADY_EXISTS`, `23503` → `CONFLICT`, `23502`/`23514` →
`VALIDATION`. Transient SQLSTATEs (`40001` serialization failure, `55P03` lock
not available) attach retry hints.

```rust,ignore
use masterror::{AppErrorKind, Error};

async fn load_user(pool: &sqlx::PgPool, id: i64) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;          // RowNotFound becomes AppErrorKind::NotFound
    Ok(user)
}
```

`sqlx-migrate` pulls full `sqlx` (still without default features) and maps
`sqlx::migrate::MigrateError` to `Database`, recording the migration phase and
version in metadata.

## redis

All `redis::RedisError` values map to `Cache` by default; timeout-flavoured
errors become `Timeout` and connection failures become `DependencyUnavailable`.
Error category and code are preserved as metadata.

## reqwest

Treats `reqwest` as a client of an external HTTP API:

- `is_timeout()` → `Timeout`
- `is_connect()` / `is_request()` → `Network`
- HTTP status errors: `429` → `RateLimited`, `408` → `Timeout`, `5xx` →
  `DependencyUnavailable`, others → `ExternalApi`
- everything else → `ExternalApi`

Metadata records the endpoint, status and low-level flags; the URL is marked
for hashing/redaction in public payloads.

## validator

`validator::ValidationErrors` → `Validation`, with aggregate context in
metadata: failing field names (`validation.fields`), field and error counts,
and the first validation codes (`validation.codes`):

```rust,ignore
use masterror::AppResult;
use validator::Validate;

#[derive(Validate)]
struct Payload {
    #[validate(length(min = 5))]
    name: String
}

fn check(p: &Payload) -> AppResult<()> {
    p.validate()?;      // ValidationErrors -> AppErrorKind::Validation
    Ok(())
}
```

## config, tokio, serde_json

- `config::ConfigError` → `Config`, with a `config.phase` metadata field
  (`not_found`, `file_parse`, `type`, ...) identifying the failing
  stage.
- `tokio::time::error::Elapsed` → `Timeout` with a
  `timeout.source = "tokio::time::timeout"` metadata field. The error carries
  no custom message, so clients see the kind's fixed title
  `"Operation timed out"`.
- `serde_json::Error` is classified via `Error::classify()`: I/O →
  `Serialization`; syntax, data and EOF → `Deserialization`.

## teloxide

`teloxide_core::RequestError` mapping:

| Variant | `AppErrorKind` |
|---|---|
| `Api` | `ExternalApi` (invalid token → `Unauthorized`) |
| `MigrateToChatId` | `ExternalApi` |
| `RetryAfter` | `RateLimited` |
| `Network` | `Network` |
| `InvalidJson` | `Deserialization` |
| `Io` | `Internal` |

## init-data (Telegram Mini Apps)

Every `init_data_rs::InitDataError` variant (missing/invalid hash, expired
payload, signature failures) maps to `TelegramAuth`, keeping Mini App
authentication failures distinct from generic bad requests.

## tonic (outbound gRPC)

`tonic` converts in the opposite direction: `masterror::Error` →
`tonic::Status` via `From`. The [`AppCode`](Error-Kinds-and-Codes-en) is mapped
to the canonical `tonic::Code` through the same `CODE_MAPPINGS` table used for
HTTP. The status carries metadata entries `app-code`, `app-http-status` and
`app-problem-type`, plus retry and `www-authenticate` hints when present.
Redactable errors have their message replaced by the kind label and their
metadata stripped.

```rust,ignore
use masterror::AppError;
use tonic::{Code, Status};

let status = Status::from(AppError::not_found("missing"));
assert_eq!(status.code(), Code::NotFound);
```

## frontend (WASM / browser)

The `frontend` feature adds the `masterror::frontend::BrowserConsoleExt` trait
for `AppError` and `ErrorResponse`, backed by `wasm-bindgen`:

- `to_js_value()` — serialize the error into a `wasm_bindgen::JsValue`
- `log_to_browser_console()` — emit it via `console.error`

Both are functional on `wasm32` targets; on native targets they return
`BrowserConsoleError::UnsupportedTarget`. Failure modes (console missing,
`console.error` not callable, serialization failure) are covered by the
`BrowserConsoleError` enum.

```rust,ignore
use masterror::{AppError, frontend::BrowserConsoleExt};

let err = AppError::not_found("user not found");
err.log_to_browser_console()?;
```

## turnkey

The `turnkey` feature exposes a small stable domain taxonomy in
`masterror::turnkey`:

| `TurnkeyErrorKind` | `AppErrorKind` |
|---|---|
| `UniqueLabel` | `Conflict` |
| `RateLimited` | `RateLimited` |
| `Timeout` | `Timeout` |
| `Auth` | `Unauthorized` |
| `Network` | `Network` |
| `Service` | `Turnkey` |

`TurnkeyError::new(kind, msg)` builds a domain error; `From<TurnkeyError> for
AppError` and `From<TurnkeyErrorKind> for AppErrorKind` perform the mapping.
`classify_turnkey_error(&str)` heuristically classifies a raw provider message
(case-insensitive, word-boundary aware) into a `TurnkeyErrorKind`:

```rust
use masterror::turnkey::{TurnkeyError, TurnkeyErrorKind, classify_turnkey_error};
use masterror::{AppError, AppErrorKind};

let kind = classify_turnkey_error("429 rate-limit reached");
assert_eq!(kind, TurnkeyErrorKind::RateLimited);

let app: AppError = TurnkeyError::new(kind, "quota exceeded").into();
assert_eq!(app.kind, AppErrorKind::RateLimited);
```

See also: [Feature Flags](Feature-Flags-en) · [Web Frameworks](Web-Frameworks-en) · [Error Kinds & Codes](Error-Kinds-and-Codes-en) · [Observability](Observability-en)
