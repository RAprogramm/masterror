# Web Frameworks

`masterror` maps errors to HTTP at the transport boundary. Domain code returns
[`AppResult<T>`](Error-Kinds-and-Codes-en); the framework adapter converts the
error into an RFC 7807 `application/problem+json` response, flushes telemetry
and applies redaction. There is exactly one `IntoResponse` /
`ResponseError` implementation for `AppError` in the crate — you never write
the mapping by hand.

## Feature flags

| Feature | Enables |
|---|---|
| `axum` | `IntoResponse` for `AppError`, `ProblemJson`, `ErrorResponse`; pulls `serde_json` |
| `actix` | `ResponseError` for `AppError`; `Responder` for `ProblemJson`, `ErrorResponse` |
| `multipart` | `From<axum::extract::multipart::MultipartError>` for `Error` (implies `axum`) |
| `openapi` | `utoipa` schema for `ErrorResponse` |

```toml
[dependencies]
masterror = { version = "0.28", features = ["axum"] }   # or ["actix"]
```

## Wire format

Both adapters serialize [`ProblemJson`](https://docs.rs/masterror/latest/masterror/struct.ProblemJson.html):

| Field | Type | Notes |
|---|---|---|
| `type` | string URI | Canonical problem class, e.g. `https://errors.masterror.rs/not-found` |
| `title` | string | Short summary derived from `AppErrorKind` |
| `status` | number | HTTP status code |
| `detail` | string? | Public message; **omitted when the error is redactable** |
| `details` | object? | Structured details (`serde_json` feature) |
| `code` | string | Stable machine-readable `AppCode`, e.g. `NOT_FOUND` |
| `grpc` | object? | `{ name, value }` gRPC mapping for multi-protocol clients |
| `metadata` | object? | Sanitized fields from `Metadata`; omitted when redacted |

Transport hints become headers, not body fields:

- `AppError::with_retry_after_secs(n)` → `Retry-After: n`
- `AppError::with_www_authenticate(challenge)` → `WWW-Authenticate: challenge`

Internal sources (`std::error::Error` chain) are logged only and never
serialized to clients.

## Axum

The `axum` feature implements `IntoResponse` for `AppError`, `ProblemJson` and
`ErrorResponse`, plus an inherent `AppError::http_status()` returning
`axum::http::StatusCode` derived from the error kind. Converting to a response
flushes telemetry (tracing event, metrics counter, lazy backtrace) — see
[Observability](Observability-en).

```rust
use axum::{Router, routing::get};
use masterror::{AppError, AppResult};

async fn handler() -> AppResult<&'static str> {
    Err(AppError::forbidden("no access"))
}

let app: Router = Router::new().route("/demo", get(handler));
```

A `401` with hints:

```rust
use masterror::AppError;

let err = AppError::unauthorized("missing token")
    .with_retry_after_secs(7)
    .with_www_authenticate("Bearer realm=\"api\"");
```

produces status `401`, headers `Retry-After: 7` and
`WWW-Authenticate: Bearer realm="api"`, and body:

```json
{
  "type": "https://errors.masterror.rs/unauthorized",
  "title": "Unauthorized",
  "status": 401,
  "detail": "missing token",
  "code": "UNAUTHORIZED",
  "grpc": { "name": "UNAUTHENTICATED", "value": 16 }
}
```

### Domain errors in handlers

The pattern from
[`examples/axum-rest-api`](https://github.com/RAprogramm/masterror/tree/main/examples/axum-rest-api):
derive a domain enum, convert it to `AppError` once, then reuse the crate's
`IntoResponse`.

```rust
use axum::response::{IntoResponse, Response};
use masterror::{AppError, Error};

#[derive(Debug, Error, Clone)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    #[error("email already exists")]
    DuplicateEmail,
    #[error("invalid email format")]
    InvalidEmail
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound => AppError::not_found(err.to_string()),
            UserError::DuplicateEmail => AppError::conflict(err.to_string()),
            UserError::InvalidEmail => AppError::validation(err.to_string())
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        AppError::from(self).into_response()
    }
}
```

With `#[app_error(kind = ..., code = ...)]` on the derive, the `From<UserError>
for AppError` impl is generated for you — see [Derive Macros](Derive-Macros-en).

## Actix Web

The `actix` feature implements `actix_web::ResponseError` for `AppError`, so
handlers returning `AppResult<T>` work out of the box. `error_response()`
emits telemetry and builds the same problem+json payload via
`ProblemJson::from_ref`.

```rust,ignore
use actix_web::{App, HttpServer, get};
use masterror::{AppError, AppResult};

#[get("/forbidden")]
async fn forbidden() -> AppResult<&'static str> {
    Err(AppError::forbidden("no access"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(forbidden))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

The client receives `403` with:

```json
{
  "type": "https://errors.masterror.rs/forbidden",
  "title": "Forbidden",
  "status": 403,
  "detail": "no access",
  "code": "FORBIDDEN",
  "grpc": { "name": "PERMISSION_DENIED", "value": 7 }
}
```

`ProblemJson` and `ErrorResponse` also implement `Responder`, so a handler can
return them directly. Status mapping uses the same stable
`AppErrorKind → StatusCode` table as Axum.

## Multipart

`multipart` (implies `axum`) converts
`axum::extract::multipart::MultipartError` into `Error` with
`AppErrorKind::BadRequest`, preserving the parser message:

```rust,ignore
use axum::extract::multipart::Multipart;
use masterror::{AppErrorKind, Error};

async fn upload(mut multipart: Multipart) -> Result<(), Error> {
    while let Some(field) = multipart.next_field().await? {
        let _ = field.bytes().await?;
    }
    Ok(())
}
```

Malformed client payloads surface as `400 Bad Request` instead of a 500.

## Building responses manually

For tests or custom transports, construct the payload without a framework:

```rust
use masterror::{AppError, ProblemJson};

let problem = ProblemJson::from_app_error(AppError::not_found("resource not found"));
assert_eq!(problem.status, 404);
assert_eq!(problem.code.as_str(), "NOT_FOUND");
```

`ProblemJson::from_ref(&err)` borrows instead of consuming, and
`ProblemJson::from_error_response(resp)` upgrades the legacy `ErrorResponse`
wire type.

See also: [Error Kinds & Codes](Error-Kinds-and-Codes-en) · [Integrations](Integrations-en) · [Observability](Observability-en) · [Feature Flags](Feature-Flags-en)
