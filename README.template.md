<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

<div align="center">
  <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/materror.png" alt="masterror" width="600"/>
  <p><strong>Framework-agnostic application error types</strong></p>

  <!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
       CI packaging will fail if README.md is stale. -->

  [![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
  [![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
  [![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
  ![MSRV](https://img.shields.io/badge/MSRV-{{MSRV}}-blue)
  ![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
  [![codecov](https://codecov.io/gh/RAprogramm/masterror/graph/badge.svg?token=V9JQDTZLXH)](https://codecov.io/gh/RAprogramm/masterror)

  [![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
  [![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/masterror?branch=main)](https://hitsofcode.com/github/RAprogramm/masterror/view?branch=main)
  [![IMIR](https://raw.githubusercontent.com/RAprogramm/infra-metrics-insight-renderer/main/assets/badges/imir-badge-simple-public.svg)](https://github.com/RAprogramm/infra-metrics-insight-renderer)

  > 🇷🇺 [Читайте README на русском языке](README.ru.md)
  > 🇰🇷 [한국어 README](README.ko.md)

</div>

---

## Table of Contents

- [Overview](#overview)
- [Highlights](#highlights)
- [Workspace Crates](#workspace-crates)
- [Feature Flags](#feature-flags)
- [Installation](#installation)
- [Benchmarks](#benchmarks)
- [Code Coverage](#code-coverage)
- [Quick Start](#quick-start)
- [Advanced Usage](#advanced-usage)
- [Resources](#resources)
- [Metrics](#metrics)
- [License](#license)

---

## Overview

`masterror` grew from a handful of helpers into a workspace of composable crates for
building consistent, observable error surfaces across Rust services. The core
crate stays framework-agnostic, while feature flags light up transport adapters,
integrations and telemetry without pulling in heavyweight defaults. No
`unsafe`, MSRV is pinned, and the derive macros keep your domain types in charge
of redaction and metadata.

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Highlights

- **Unified taxonomy.** `AppError`, `AppErrorKind` and `AppCode` model domain and
  transport concerns with conservative HTTP/gRPC mappings, turnkey retry/auth
  hints and RFC7807 output via `ProblemJson`.
- **Native derives.** `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]`,
  `#[masterror(...)]` and `#[provide]` wire custom types into `AppError` while
  forwarding sources, backtraces, telemetry providers and redaction policy.
- **Typed telemetry.** `Metadata` stores structured key/value context (strings,
  integers, floats, durations, IP addresses and optional JSON) with per-field
  redaction controls and builders in `field::*`, so logs stay structured without
  manual `String` maps.
- **Transport adapters.** Optional features expose Actix/Axum responders,
  `tonic::Status` conversions, WASM/browser logging and OpenAPI schema
  generation without contaminating the lean default build.
- **Battle-tested integrations.** Enable focused mappings for `sqlx`,
  `reqwest`, `redis`, `validator`, `config`, `tokio`, `teloxide`, `multipart`,
  Telegram WebApp SDK and more — each translating library errors into the
  taxonomy with telemetry attached.
- **Turnkey defaults.** The `turnkey` module ships a ready-to-use error catalog,
  helper builders and tracing instrumentation for teams that want a consistent
  baseline out of the box.
- **Typed control-flow macros.** `ensure!` and `fail!` short-circuit functions
  with your domain errors without allocating or formatting on the happy path.

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Workspace Crates

| Crate | What it provides | When to depend on it |
| --- | --- | --- |
| [`masterror`](https://crates.io/crates/masterror) | Core error types, metadata builders, transports, integrations and the prelude. | Application crates, services and libraries that want a stable error surface. |
| [`masterror-derive`](masterror-derive/README.md) | Proc-macros backing `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]` and `#[provide]`. | Brought in automatically via `masterror`; depend directly only for macro hacking. |
| [`masterror-template`](masterror-template/README.md) | Shared template parser used by the derive macros for formatter analysis. | Internal dependency; reuse when you need the template parser elsewhere. |

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Feature Flags

Pick only what you need; everything is off by default.

- **Web transports:** `axum`, `actix`, `multipart`, `openapi`, `serde_json`.
- **Telemetry & observability:** `tracing`, `metrics`, `backtrace`.
- **Async & IO integrations:** `tokio`, `reqwest`, `sqlx`, `sqlx-migrate`,
  `redis`, `validator`, `config`.
- **Messaging & bots:** `teloxide`, `telegram-webapp-sdk`.
- **Front-end tooling:** `frontend` for WASM/browser console logging.
- **gRPC:** `tonic` to emit `tonic::Status` responses.
- **Batteries included:** `turnkey` to adopt the pre-built taxonomy and helpers.

The build script keeps the full feature snippet below in sync with
`Cargo.toml`.

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Installation

~~~toml
[dependencies]
masterror = { version = "{{CRATE_VERSION}}", default-features = false }
# or with features:
# masterror = { version = "{{CRATE_VERSION}}", features = [
{{FEATURE_SNIPPET}}
# ] }
~~~

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Benchmarks

Criterion benchmarks cover the hottest conversion paths so regressions are
visible before shipping. Run them locally with:

~~~sh
cargo bench -F benchmarks --bench error_paths
~~~

The suite emits two groups:

- `context_into_error/*` promotes a dummy source error with representative
  metadata (strings, counters, durations, IPs) through `Context::into_error` in
  both redacted and non-redacted modes.
- `problem_json_from_app_error/*` consumes the resulting `AppError` values to
  build RFC 7807 payloads via `ProblemJson::from_app_error`, showing how message
  redaction and field policies impact serialization.

Adjust Criterion CLI flags (for example `--sample-size 200` or `--save-baseline local`) after `--` to trade
throughput for tighter confidence intervals when investigating changes.

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Code Coverage

[![codecov](https://codecov.io/gh/RAprogramm/masterror/branch/main/graph/badge.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

Coverage reports are automatically generated on every CI run and uploaded to [Codecov](https://app.codecov.io/gh/RAprogramm/masterror). The project maintains high test coverage across all modules to ensure reliability and catch regressions early.

<details>
  <summary><b>Coverage Visualizations</b></summary>

#### Sunburst Graph
The inner-most circle represents the entire project, moving outward through folders to individual files. Size and color indicate statement count and coverage percentage.

[![Sunburst](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/sunburst.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

#### Grid View
Each block represents a single file. Block size and color correspond to statement count and coverage percentage.

[![Grid](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/tree.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

#### Icicle Chart
Hierarchical view starting with the entire project at the top, drilling down through folders to individual files. Size and color reflect statement count and coverage.

[![Icicle](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/icicle.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

</details>

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Quick Start

<details>
  <summary><b>Create an error</b></summary>

Create an error:

~~~rust
use masterror::{AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
let err_with_meta = AppError::service("downstream")
    .with_field(field::str("request_id", "abc123"));
assert_eq!(err_with_meta.metadata().len(), 1);

let err_with_context = AppError::internal("db down")
    .with_context(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
assert!(err_with_context.source_ref().is_some());
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

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Advanced Usage

<details>
  <summary><b>Fail fast without sacrificing typing</b></summary>

`ensure!` and `fail!` provide typed alternatives to the formatting-heavy
`anyhow::ensure!`/`anyhow::bail!` helpers. They evaluate the error expression
only when the guard trips, so success paths stay allocation-free.

~~~rust
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
~~~

</details>

<details>
  <summary><b>Derive domain errors and map them to transports</b></summary>

`masterror` ships native derives so your domain types stay expressive while the
crate handles conversions, telemetry and redaction for you.

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

- `use masterror::Error;` brings the derive macro into scope.
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

</details>

<details>
  <summary><b>Attach telemetry, redaction policy and conversions</b></summary>

`#[derive(Masterror)]` wires a domain error into [`masterror::Error`], adds
metadata, redaction policy and optional transport mappings. The accompanying
`#[masterror(...)]` attribute mirrors the `#[app_error]` syntax while staying
explicit about telemetry and redaction.

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
    redact(message, fields("user_id" = hash)),
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
  boundary, `fields("name" = hash, "card" = last4)` overrides metadata
  policies (`hash`, `last4`, `redact`, `none`).
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

</details>

<details>
  <summary><b>Structured telemetry providers and AppError mappings</b></summary>

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
`AppError`/`AppCode` without manual glue.

</details>

<details>
  <summary><b>Problem JSON payloads and retry/authentication hints</b></summary>

~~~rust
use masterror::{AppError, AppErrorKind, ProblemJson};
use std::time::Duration;

let problem = ProblemJson::from_app_error(
    AppError::new(AppErrorKind::Unauthorized, "Token expired")
        .with_retry_after_duration(Duration::from_secs(30))
        .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#)
);

assert_eq!(problem.status, 401);
assert_eq!(problem.retry_after, Some(30));
assert_eq!(problem.grpc.expect("grpc").name, "UNAUTHENTICATED");
~~~

</details>

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Resources

- Explore the [error-handling wiki](docs/wiki/index.md) for step-by-step guides,
  comparisons with `thiserror`/`anyhow`, and troubleshooting recipes.
- Browse the [crate documentation on docs.rs](https://docs.rs/masterror) for API
  details, feature-specific guides and transport tables.
- Check [`CHANGELOG.md`](CHANGELOG.md) for release highlights and migration notes.
- Review [RustManifest](https://github.com/RAprogramm/RustManifest) for the development standards and best practices this project follows.

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Metrics

![Metrics](https://raw.githubusercontent.com/RAprogramm/infra-metrics-insight-renderer/main/metrics/masterror.svg)

<div align="right">

<div align="right">
  <a href="#table-of-contents">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## License

MSRV: **{{MSRV}}** · License: **MIT OR Apache-2.0** · No `unsafe`


