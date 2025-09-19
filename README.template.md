# masterror · Framework-agnostic application error types

<!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
     CI packaging will fail if README.md is stale. -->

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-{{MSRV}}-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Security audit](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main&label=Security%20audit)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
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
masterror = { version = "{{CRATE_VERSION}}", default-features = false }
# or with features:
# masterror = { version = "{{CRATE_VERSION}}", features = [
{{FEATURE_SNIPPET}}
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
- `TemplateFormatter` mirrors `thiserror`'s formatter detection so existing
  derives that relied on hexadecimal, pointer or exponential renderers keep
  compiling.
- `TemplateFormatterKind` exposes the formatter trait requested by a
  placeholder, making it easy to branch on the requested rendering behaviour
  without manually matching every enum variant.

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
    TemplateFormatter::Debug { alternate: false }
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
