# masterror-derive

Procedural macros that power [`masterror`](https://crates.io/crates/masterror)'s
`#[derive(Error)]`. The derive generates ergonomic `std::error::Error` and
`Display` implementations together with seamless integration into
`masterror`'s domain-centric `AppError` type.

> **Tip:** Depend on the `masterror` crate in application code and import the
> macros from there (`use masterror::Error;`). This standalone crate is
> published to make `cargo install`/`cargo package` flows happy and to support
> advanced macro integrations.

## Quick start

```toml
[dependencies]
masterror = "0.10"
```

```rust
use masterror::{AppError, Error};

#[derive(Error)]
#[error(display = "failed to parse payload: {source}")]
#[app_error(kind = "BadRequest")]
pub struct PayloadInvalid {
    #[source]
    pub source: serde_json::Error,
}

fn parse(input: &str) -> Result<(), AppError> {
    serde_json::from_str::<serde_json::Value>(input)
        .map(|_| ())
        .map_err(PayloadInvalid::from)
}
```

The derive implements `Display`, `std::error::Error`, and conversion glue so
you can return rich `AppError` values with a single `?`.

## Supported attributes

- `#[error(display = ...)]` – formats the error message using captured fields.
- `#[source]` / `#[from]` – wires source error propagation and conversion.
- `#[backtrace]` – exposes an optional captured `Backtrace`.
- `#[app_error(...)]` – configures how the error maps into `AppError`
  (kind, HTTP status, telemetry).
- `#[provide(...)]` – attaches structured telemetry providers that surface
  typed context (IDs, domains, tenant information) through tracing layers.

See the main [`masterror` README](https://github.com/RAprogramm/masterror/blob/HEAD/README.md) for an end-to-end guide and
advanced examples covering templated display strings, telemetry providers and
OpenAPI/schema integrations.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](https://github.com/RAprogramm/masterror/blob/HEAD/LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/RAprogramm/masterror/blob/HEAD/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

