<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Examples

Comprehensive real-world examples demonstrating masterror integration with popular frameworks and use cases.

## Quick Start Examples

| Example | Description | Run Command |
|---------|-------------|-------------|
| [`basic_usage.rs`](basic_usage.rs) | Core error creation and conversion patterns | `cargo run --example basic_usage` |
| [`derive_error.rs`](derive_error.rs) | Custom error types with derive macro | `cargo run --example derive_error` |
| [`structured_metadata.rs`](structured_metadata.rs) | Attaching JSON metadata and context | `cargo run --example structured_metadata` |
| [`colored_cli.rs`](colored_cli.rs) | Terminal output with color support | `cargo run --example colored_cli` |
| [`redaction.rs`](redaction.rs) | Sensitive data redaction | `cargo run --example redaction` |
| [`migrate_from_anyhow.rs`](migrate_from_anyhow.rs) | Migration guide from anyhow | `cargo run --example migrate_from_anyhow` |
| [`migrate_from_thiserror.rs`](migrate_from_thiserror.rs) | Migration guide from thiserror | `cargo run --example migrate_from_thiserror` |

## Real-World Service Examples

| Example | Description | Status |
|---------|-------------|--------|
| [`axum-rest-api/`](axum-rest-api/) | REST API with RFC 7807 Problem Details | ✅ Available |
| [`sqlx-database/`](sqlx-database/) | Database error handling with SQLx | ✅ Available |
| [`custom-domain-errors/`](custom-domain-errors/) | Payment processing domain errors | ✅ Available |
| [`basic-async/`](basic-async/) | Async error handling with tokio | ✅ Available |
| `multi-transport/` | Shared errors across HTTP + gRPC | 🚧 Planned |
| `tonic-grpc-service/` | gRPC service with tonic | 🚧 Planned |
| `actix-web-service/` | Actix-web integration | 🚧 Planned |
| `telemetry-integration/` | OpenTelemetry + tracing | 🚧 Planned |
| `comparison-thiserror/` | Side-by-side comparison | 🚧 Planned |

## Running Examples

### Simple Examples

```bash
cargo run --example basic_usage
cargo run --example colored_cli
```

### Service Examples

Each service example is a workspace member with its own Cargo.toml:

```bash
cd examples/axum-rest-api
cargo run

cd examples/sqlx-database
cargo test
```

## Example Requirements

Each real-world example includes:

- **Runnable code** - Full working service or application
- **README.md** - Scenario explanation and usage guide
- **Tests** - Integration and unit tests
- **Comments** - Key decisions and patterns explained
- **Minimal dependencies** - Only necessary crates

## Contributing Examples

When adding new examples:

1. Follow project structure conventions
2. Add SPDX headers to all files
3. Include comprehensive tests
4. Update this README.md index
5. Ensure `cargo test` passes
6. Add example to CI workflow

## License

All examples are licensed under MIT, same as masterror.
