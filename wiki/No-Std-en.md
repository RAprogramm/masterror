# no_std Support

`masterror` builds without the Rust standard library. The crate root declares
`#![cfg_attr(not(feature = "std"), no_std)]`, and the default `std` feature is
the only thing standing between you and an embedded/WASM-friendly build:

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }
```

## alloc is required

`masterror` is `no_std` but **not** `no_alloc`. The crate unconditionally
declares `extern crate alloc` and uses `Cow<'static, str>`, `String`, `Arc`
and `BTreeMap` for messages, metadata and source chains. Your target needs a
global allocator; pure `core`-only environments are not supported.

## What works without `std`

The entire framework-agnostic core:

| Area | Available in `no_std` |
|---|---|
| Core types | `Error` / `AppError`, `AppResult`, `AppErrorKind`, `AppCode` |
| Metadata | `Metadata`, `Field`, `FieldValue`, `FieldRedaction`, `field::*` helpers |
| Context | `Context`, `ResultExt::{ctx, context}` |
| Control flow | `ensure!`, `fail!` |
| Derives | `#[derive(Error)]`, `#[derive(Masterror)]` with all attributes |
| Wire types | `ProblemJson`, `ErrorResponse`, `CODE_MAPPINGS`, `mapping_for_code` |
| Introspection | `chain()`, `root_cause()`, `is`/`downcast`/`downcast_ref`/`downcast_mut`, `render_message()` |
| Serde | `serde` with `alloc` (JSON serialization of wire types) |

Error sources work through **`core::error::Error`**: the crate implements and
consumes `core::error::Error` (aliased internally as `CoreError`) instead of
`std::error::Error`, so `with_source(...)`, source chains and downcasting are
fully functional in `no_std` builds.

```rust
use masterror::{AppCode, AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::Timeout, "deadline exceeded")
    .with_field(field::u64("attempt", 3));

assert_eq!(err.code, AppCode::Timeout);
assert_eq!(err.metadata().len(), 1);
```

## What requires `std`

Every runtime integration explicitly re-enables `std` in its feature
definition. From `Cargo.toml`:

- `tracing`, `metrics`, `backtrace`, `colored`
- `axum`, `actix`, `multipart`, `tonic`, `openapi`
- `serde_json`, `redis`, `validator`, `config`, `tokio`, `reqwest`,
  `teloxide`, `init-data`, `frontend`, `turnkey`

`backtrace` needs `std::backtrace::Backtrace` and environment access;
`colored` needs TTY detection; the web and client integrations need their
host crates, which are themselves `std`-only.

## CI feature matrix

The `no_std` CI job (`.github/workflows/ci.yml`) checks these combinations on
every pull request and push to `main`:

| Job | Command | Verifies |
|---|---|---|
| `bare` | `cargo check --no-default-features` | true `no_std` + `alloc` build |
| `std-only` | `cargo check --features std` | default std surface |
| `tracing` | `cargo check --no-default-features --features tracing` | single telemetry feature builds standalone |
| `metrics` | `cargo check --no-default-features --features metrics` | same for metrics |
| `colored` | `cargo check --no-default-features --features colored` | same for colored |
| `all-features` | `cargo check --all-features` | full feature union |

Note the semantics: only the `bare` job is a genuine `no_std` compilation.
`tracing = [..., "std"]`, `metrics = [..., "std"]` and
`colored = [..., "std"]` transitively re-enable `std`, so those jobs verify
that each telemetry feature is self-sufficient when defaults are off — not
that telemetry works without the standard library. If you need telemetry, you
need `std`.

## Practical setup

Library crates that want to stay transport-agnostic and `no_std`-compatible:

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }

[features]
std = ["masterror/std"]
```

The binary or service crate then turns on `std` plus whatever integrations it
needs:

```toml
[dependencies]
masterror = { version = "0.28", features = ["axum", "tracing", "metrics"] }
```

Because `AppErrorKind`, `AppCode` and the wire types live in the `no_std`
core, domain crates can classify errors and even build `ProblemJson` payloads
while the HTTP mapping happens only in the service crate — see
[Best Practices](Best-Practices-en).

## Toolchain

The crate targets edition 2024 with `rust-version = "1.96"` in `Cargo.toml`.
`core::error::Error` (the foundation of `no_std` source chains) has been
stable since Rust 1.81, so no nightly features are involved.

See also: [Feature Flags](Feature-Flags-en) · [Getting Started](Getting-Started-en) · [Best Practices](Best-Practices-en)
