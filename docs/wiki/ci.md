# Continuous integration recipes

The workspace exposes reusable GitHub composite actions so multiple workflows can
share the same cargo setup without copying command bodies. Each composite lives
under [`.github/actions`](../../.github/actions) and assumes the caller already
prepared the toolchain, caches, and workspace checkout.

## Available composites

| Action | Purpose | Required inputs |
| ------ | ------- | ---------------- |
| `./.github/actions/cargo-fmt` | Runs `cargo fmt -- --check` with the requested toolchain. | `toolchain` (defaults to `nightly`). |
| `./.github/actions/cargo-clippy` | Executes `cargo clippy --workspace --all-targets`, optionally enabling `--all-features`. | `toolchain` (MSRV or other installed toolchain). |
| `./.github/actions/cargo-test` | Executes `cargo test --workspace` with optional `--all-features` and `--no-fail-fast`. | `toolchain` (MSRV or other installed toolchain). |
| `./.github/actions/cargo-audit` | Installs (if required) and runs `cargo audit` with `--deny warnings` by default. | None. |

## Usage example

After the shared setup steps (checkout, Rust toolchain install, cache restore),
call the composites in sequence:

```yaml
jobs:
  lint-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: 1.70.0
      - name: Cargo cache
        uses: Swatinem/rust-cache@v2
      - name: Run fmt
        uses: ./.github/actions/cargo-fmt
        with:
          toolchain: nightly
      - name: Run clippy
        uses: ./.github/actions/cargo-clippy
        with:
          toolchain: 1.70.0
          all-features: true
      - name: Run tests
        uses: ./.github/actions/cargo-test
        with:
          toolchain: 1.70.0
      - name: Security audit
        uses: ./.github/actions/cargo-audit
```

Each composite forwards optional inputs like `extra-args` so teams can tailor
command flags without duplicating the shell logic.
