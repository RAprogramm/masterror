# masterror · Framework-agnostic application error types

<!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
     CI packaging will fail if README.md is stale. -->

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.90-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Security audit](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main&label=Security%20audit)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Cargo Deny](https://img.shields.io/github/actions/workflow/status/RAprogramm/masterror/ci.yml?branch=main&label=Cargo%20Deny)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)

> 🇷🇺 Читайте README на [русском языке](README.ru.md).

Small, pragmatic error model for API-heavy Rust services with native derives
and typed telemetry.
Core is framework-agnostic; integrations are opt-in via feature flags.
Stable categories, conservative HTTP mapping, no `unsafe`.

- Core types: `AppError`, `AppErrorKind`, `AppResult`, `AppCode`, `ErrorResponse`, `Metadata`
- Derive macros: `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]`,
  `#[masterror(...)]`, `#[provide]` for domain mappings and structured
  telemetry
- Optional Axum/Actix integration and browser/WASM console logging
- Optional OpenAPI schema (via `utoipa`)
- Structured metadata helpers via `field::*` builders
- Conversions from `sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio`
- Turnkey domain taxonomy and helpers (`turnkey` feature)

👉 Explore the new [error-handling wiki](docs/wiki/index.md) for step-by-step
guides, comparisons with `thiserror`/`anyhow`, and troubleshooting recipes.

---

### TL;DR

~~~toml
[dependencies]
masterror = { version = "0.13.1", default-features = false }
# or with features:
# masterror = { version = "0.13.1", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "sqlx", "sqlx-migrate", "reqwest", "redis",
#   "validator", "config", "tokio", "multipart",
#   "teloxide", "telegram-webapp-sdk", "frontend", "turnkey"
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
- **Typed telemetry.** `Metadata` preserves structured key/value context without `String` maps.
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
masterror = { version = "0.13.1", default-features = false }

# with Axum/Actix + JSON + integrations
# masterror = { version = "0.13.1", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "sqlx", "sqlx-migrate", "reqwest", "redis",
#   "validator", "config", "tokio", "multipart",
#   "teloxide", "telegram-webapp-sdk", "frontend", "turnkey"
# ] }
~~~

**MSRV:** 1.90
**No unsafe:** forbidden by crate.

</details>

<details>
  <summary><b>Quick start</b></summary>

Create an error:

~~~rust
use masterror::{AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
let err_with_meta = AppError::service("downstream")
    .with_field(field::str("request_id", "abc123"));
assert_eq!(err_with_meta.metadata().len(), 1);
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
- `#[app_error(kind = AppErrorKind::..., code = AppCode::..., message)]` maps the
  derived error into `AppError`/`AppCode`. The optional `code = ...` arm emits an
  `AppCode` conversion, while the `message` flag forwards the derived
  `Display` output as the public message instead of producing a bare error.
- `masterror::error::template::ErrorTemplate` parses `#[error("...")]`
  strings, exposing literal and placeholder segments so custom derives can be
  implemented without relying on `thiserror`.
- `TemplateFormatter` mirrors `thiserror`'s formatter detection so existing
  derives that relied on hexadecimal, pointer or exponential renderers keep
  compiling.
- Display placeholders preserve their raw format specs via
  `TemplateFormatter::display_spec()` and `TemplateFormatter::format_fragment()`,
  so derived code can forward `:>8`, `:.3` and other display-only options
  without reconstructing the original string.
- `TemplateFormatterKind` exposes the formatter trait requested by a
  placeholder, making it easy to branch on the requested rendering behaviour
  without manually matching every enum variant.

#### `#[derive(Masterror)]` and `#[masterror(...)]`

`#[derive(Masterror)]` wires a domain error directly into [`masterror::Error`],
augmenting it with metadata, redaction policy and optional transport mappings.
The accompanying `#[masterror(...)]` attribute mirrors the `#[app_error]`
syntax while remaining explicit about telemetry:

~~~rust
use masterror::{
    mapping::HttpMapping, AppCode, AppErrorKind, Error, Masterror, MessageEditPolicy
};

#[derive(Debug, Masterror)]
#[error("user {user_id} missing flag {flag}")]
#[masterror(
    code = AppCode::NotFound,
    category = AppErrorKind::NotFound,
    message,
    redact(message),
    telemetry(
        Some(masterror::field::str("user_id", user_id.clone())),
        attempt.map(|value| masterror::field::u64("attempt", value))
    ),
    map.grpc = 5,
    map.problem = "https://errors.example.com/not-found"
)]
struct MissingFlag {
    user_id: String,
    flag: &'static str,
    attempt: Option<u64>,
    #[source]
    source: Option<std::io::Error>
}

let err = MissingFlag {
    user_id: "alice".into(),
    flag: "beta",
    attempt: Some(2),
    source: None
};
let converted: Error = err.into();
assert_eq!(converted.code, AppCode::NotFound);
assert_eq!(converted.kind, AppErrorKind::NotFound);
assert_eq!(converted.edit_policy, MessageEditPolicy::Redact);
assert!(converted.metadata().get("user_id").is_some());

assert_eq!(
    MissingFlag::HTTP_MAPPING,
    HttpMapping::new(AppCode::NotFound, AppErrorKind::NotFound)
);
~~~

- `code` / `category` pick the public [`AppCode`] and internal
  [`AppErrorKind`].
- `message` forwards the formatted [`Display`] output as the safe public
  message. Omit it to keep the message private.
- `redact(message)` flips [`MessageEditPolicy`] to redactable at the transport
  boundary.
- `telemetry(...)` accepts expressions that evaluate to
  `Option<masterror::Field>`. Each populated field is inserted into the
  resulting [`Metadata`]; use `telemetry()` when no fields are attached.
- `map.grpc` / `map.problem` capture optional gRPC status codes (as `i32`) and
  RFC 7807 `type` URIs. The derive emits tables such as
  `MyError::HTTP_MAPPING`, `MyError::GRPC_MAPPING` and
  `MyError::PROBLEM_MAPPING` (or slice variants for enums) for downstream
  integrations.

All familiar field-level attributes (`#[from]`, `#[source]`, `#[backtrace]`)
are still honoured. Sources and backtraces are automatically attached to the
generated [`masterror::Error`].

#### Display shorthand projections

`#[error("...")]` supports the same shorthand syntax as `thiserror` for
referencing fields with `.field` or `.0`. The derive now understands chained
segments, so projections like `.limits.lo`, `.0.data` or
`.suggestion.as_ref().map_or_else(...)` keep compiling unchanged. Raw
identifiers and tuple indices are preserved, ensuring keywords such as
`r#type` and tuple fields continue to work even when you call methods on the
projected value.

~~~rust
use masterror::Error;

#[derive(Debug)]
struct Limits {
    lo: i32,
    hi: i32,
}

#[derive(Debug, Error)]
#[error(
    "range {lo}-{hi} suggestion {suggestion}",
    lo = .limits.lo,
    hi = .limits.hi,
    suggestion = .suggestion.as_ref().map_or_else(|| "<none>", |s| s.as_str())
)]
struct RangeError {
    limits: Limits,
    suggestion: Option<String>,
}

#[derive(Debug)]
struct Payload {
    data: &'static str,
}

#[derive(Debug, Error)]
enum UiError {
    #[error("tuple data {data}", data = .0.data)]
    Tuple(Payload),
    #[error(
        "named suggestion {value}",
        value = .suggestion.as_ref().map_or_else(|| "<none>", |s| s.as_str())
    )]
    Named { suggestion: Option<String> },
}
~~~

#### AppError conversions

Annotating structs or enum variants with `#[app_error(...)]` captures the
metadata required to convert the domain error into `AppError` (and optionally
`AppCode`). Every variant in an enum must provide the mapping when any variant
requests it.

~~~rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("missing flag: {name}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
struct MissingFlag {
    name: &'static str,
}

let app: AppError = MissingFlag { name: "feature" }.into();
assert!(matches!(app.kind, AppErrorKind::BadRequest));
assert_eq!(app.message.as_deref(), Some("missing flag: feature"));

let code: AppCode = MissingFlag { name: "feature" }.into();
assert!(matches!(code, AppCode::BadRequest));
~~~

For enums, each variant specifies the mapping while the derive generates a
single `From<Enum>` implementation that matches every variant:

~~~rust
#[derive(Debug, Error)]
enum ApiError {
    #[error("missing resource {id}")]
    #[app_error(
        kind = AppErrorKind::NotFound,
        code = AppCode::NotFound,
        message
    )]
    Missing { id: u64 },
    #[error("backend unavailable")]
    #[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
    Backend,
}

let missing = ApiError::Missing { id: 7 };
let as_app: AppError = missing.into();
assert_eq!(as_app.message.as_deref(), Some("missing resource 7"));
~~~

#### Structured telemetry providers and AppError mappings

`#[provide(...)]` exposes typed context through `std::error::Request`, while
`#[app_error(...)]` records how your domain error translates into `AppError`
and `AppCode`. The derive mirrors `thiserror`'s syntax and extends it with
optional telemetry propagation and direct conversions into the `masterror`
runtime types.

~~~rust
use std::error::request_ref;

use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
struct TelemetrySnapshot {
    name:  &'static str,
    value: u64,
}

#[derive(Debug, Error)]
#[error("structured telemetry {snapshot:?}")]
#[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
struct StructuredTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    snapshot: TelemetrySnapshot,
}

let err = StructuredTelemetryError {
    snapshot: TelemetrySnapshot {
        name: "db.query",
        value: 42,
    },
};

let snapshot = request_ref::<TelemetrySnapshot>(&err).expect("telemetry");
assert_eq!(snapshot.value, 42);

let app: AppError = err.into();
let via_app = request_ref::<TelemetrySnapshot>(&app).expect("telemetry");
assert_eq!(via_app.name, "db.query");
~~~

Optional telemetry only surfaces when present, so `None` does not register a
provider. Owned snapshots can still be provided as values when the caller
requests ownership:

~~~rust
use masterror::{AppCode, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("optional telemetry {telemetry:?}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal)]
struct OptionalTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    telemetry: Option<TelemetrySnapshot>,
}

let noisy = OptionalTelemetryError {
    telemetry: Some(TelemetrySnapshot {
        name: "queue.depth",
        value: 17,
    }),
};
let silent = OptionalTelemetryError { telemetry: None };

assert!(request_ref::<TelemetrySnapshot>(&noisy).is_some());
assert!(request_ref::<TelemetrySnapshot>(&silent).is_none());
~~~

Enums support per-variant telemetry and conversion metadata. Each variant chooses
its own `AppErrorKind`/`AppCode` mapping while the derive generates a single
`From<Enum>` implementation:

~~~rust
#[derive(Debug, Error)]
enum EnumTelemetryError {
    #[error("named {label}")]
    #[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound)]
    Named {
        label:    &'static str,
        #[provide(ref = TelemetrySnapshot)]
        snapshot: TelemetrySnapshot,
    },
    #[error("optional tuple")]
    #[app_error(kind = AppErrorKind::Timeout, code = AppCode::Timeout)]
    Optional(#[provide(ref = TelemetrySnapshot)] Option<TelemetrySnapshot>),
    #[error("owned tuple")]
    #[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
    Owned(#[provide(value = TelemetrySnapshot)] TelemetrySnapshot),
}

let owned = EnumTelemetryError::Owned(TelemetrySnapshot {
    name: "redis.latency",
    value: 3,
});
let app: AppError = owned.into();
assert!(matches!(app.kind, AppErrorKind::Service));
~~~

Compared to `thiserror`, you retain the familiar deriving surface while gaining
structured telemetry (`#[provide]`) and first-class conversions into
`AppError`/`AppCode` without writing manual `From` implementations.

#### Formatter traits

Placeholders default to `Display` (`{value}`) but can opt into richer
formatters via the same specifiers supported by `thiserror` v2.
`TemplateFormatter::is_alternate()` tracks the `#` flag, while
`TemplateFormatterKind` exposes the underlying `core::fmt` trait so derived
code can branch on the requested renderer without manual pattern matching.
Unsupported formatters surface a compile error that mirrors `thiserror`'s
diagnostics.

| Specifier        | `core::fmt` trait          | Example output         | Notes |
|------------------|----------------------------|------------------------|-------|
| _default_        | `core::fmt::Display`       | `value`                | User-facing strings; `#` has no effect. |
| `:?` / `:#?`     | `core::fmt::Debug`         | `Struct { .. }` / multi-line | Mirrors `Debug`; `#` pretty-prints structs. |
| `:x` / `:#x`     | `core::fmt::LowerHex`      | `0x2a`                 | Hexadecimal; `#` prepends `0x`. |
| `:X` / `:#X`     | `core::fmt::UpperHex`      | `0x2A`                 | Uppercase hex; `#` prepends `0x`. |
| `:p` / `:#p`     | `core::fmt::Pointer`       | `0x1f00` / `0x1f00`    | Raw pointers; `#` is accepted for compatibility. |
| `:b` / `:#b`     | `core::fmt::Binary`        | `101010` / `0b101010` | Binary; `#` prepends `0b`. |
| `:o` / `:#o`     | `core::fmt::Octal`         | `52` / `0o52`         | Octal; `#` prepends `0o`. |
| `:e` / `:#e`     | `core::fmt::LowerExp`      | `1.5e-2`              | Scientific notation; `#` forces the decimal point. |
| `:E` / `:#E`     | `core::fmt::UpperExp`      | `1.5E-2`              | Uppercase scientific; `#` forces the decimal point. |

- `TemplateFormatterKind::supports_alternate()` reports whether the `#` flag is
  meaningful for the requested trait (pointer accepts it even though the output
  matches the non-alternate form).
- `TemplateFormatterKind::specifier()` returns the canonical format specifier
  character when one exists, enabling custom derives to re-render placeholders
  in their original style.
- `TemplateFormatter::from_kind(kind, alternate)` reconstructs a formatter from
  the lightweight `TemplateFormatterKind`, making it easy to toggle the
  alternate flag in generated code.

~~~rust
use core::ptr;

use masterror::Error;

#[derive(Debug, Error)]
#[error(
    "debug={payload:?}, hex={id:#x}, ptr={ptr:p}, bin={mask:#b}, \
     oct={mask:o}, lower={ratio:e}, upper={ratio:E}"
)]
struct FormattedError {
    id: u32,
    payload: String,
    ptr: *const u8,
    mask: u8,
    ratio: f32,
}

let err = FormattedError {
    id: 0x2a,
    payload: "hello".into(),
    ptr: ptr::null(),
    mask: 0b1010_0001,
    ratio: 0.15625,
};

let rendered = err.to_string();
assert!(rendered.contains("debug=\"hello\""));
assert!(rendered.contains("hex=0x2a"));
assert!(rendered.contains("ptr=0x0"));
assert!(rendered.contains("bin=0b10100001"));
assert!(rendered.contains("oct=241"));
assert!(rendered.contains("lower=1.5625e-1"));
assert!(rendered.contains("upper=1.5625E-1"));
~~~

~~~rust
use masterror::error::template::{
    ErrorTemplate, TemplateFormatter, TemplateFormatterKind
};

let template = ErrorTemplate::parse("{code:#x} → {payload:?}").expect("parse");
let mut placeholders = template.placeholders();

let code = placeholders.next().expect("code placeholder");
let code_formatter = code.formatter();
assert!(matches!(
    code_formatter,
    TemplateFormatter::LowerHex { alternate: true }
));
let code_kind = code_formatter.kind();
assert_eq!(code_kind, TemplateFormatterKind::LowerHex);
assert!(code_formatter.is_alternate());
assert_eq!(code_kind.specifier(), Some('x'));
assert!(code_kind.supports_alternate());
let lowered = TemplateFormatter::from_kind(code_kind, false);
assert!(matches!(
    lowered,
    TemplateFormatter::LowerHex { alternate: false }
));

let payload = placeholders.next().expect("payload placeholder");
let payload_formatter = payload.formatter();
assert_eq!(
    payload_formatter,
    &TemplateFormatter::Debug { alternate: false }
);
let payload_kind = payload_formatter.kind();
assert_eq!(payload_kind, TemplateFormatterKind::Debug);
assert_eq!(payload_kind.specifier(), Some('?'));
assert!(payload_kind.supports_alternate());
let pretty_debug = TemplateFormatter::from_kind(payload_kind, true);
assert!(matches!(
    pretty_debug,
    TemplateFormatter::Debug { alternate: true }
));
assert!(pretty_debug.is_alternate());
~~~

Display-only format specs (alignment, precision, fill — including `#` as a fill
character) are preserved so you can forward them to `write!` without rebuilding
the fragment:

~~~rust
use masterror::error::template::ErrorTemplate;

let aligned = ErrorTemplate::parse("{value:>8}").expect("parse");
let display = aligned.placeholders().next().expect("display placeholder");
assert_eq!(display.formatter().display_spec(), Some(">8"));
assert_eq!(
    display
        .formatter()
        .format_fragment()
        .as_deref(),
    Some(">8")
);

let hashed = ErrorTemplate::parse("{value:#>4}").expect("parse");
let hash_placeholder = hashed
    .placeholders()
    .next()
    .expect("hash-fill display placeholder");
assert_eq!(hash_placeholder.formatter().display_spec(), Some("#>4"));
assert_eq!(
    hash_placeholder
        .formatter()
        .format_fragment()
        .as_deref(),
    Some("#>4")
);
~~~

> **Compatibility with `thiserror` v2:** the derive understands the extended
> formatter set introduced in `thiserror` 2.x and reports identical diagnostics
> for unsupported specifiers, so migrating existing derives is drop-in.

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

</details>

<details>
  <summary><b>Feature flags</b></summary>

- `axum` — IntoResponse integration with structured JSON bodies
- `actix` — Actix Web ResponseError and Responder implementations
- `openapi` — Generate utoipa OpenAPI schema for ErrorResponse
- `serde_json` — Attach structured JSON details to AppError
- `sqlx` — Classify sqlx_core::Error variants into AppError kinds
- `sqlx-migrate` — Map sqlx::migrate::MigrateError into AppError (Database)
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
masterror = { version = "0.13.1", default-features = false }
~~~

API (Axum + JSON + deps):

~~~toml
masterror = { version = "0.13.1", features = [
  "axum", "serde_json", "openapi",
  "sqlx", "reqwest", "redis", "validator", "config", "tokio"
] }
~~~

API (Actix + JSON + deps):

~~~toml
masterror = { version = "0.13.1", features = [
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
