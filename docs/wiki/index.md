# Masterror error-handling wiki

This wiki collects step-by-step guides for building reliable error handling in Rust services.
Each page is intentionally short and focused so you can jump straight to the
section that matches your experience level.

- [Rust error handling basics](rust-error-handling-basics.md)
- [Building applications with `masterror`](masterror-application-guide.md)
- [When to reach for `thiserror`, `anyhow`, or `masterror`](error-crate-comparison.md)
- [Patterns and troubleshooting](patterns-and-troubleshooting.md)
- [Continuous integration recipes](ci.md)

## How the wiki is organised

1. **Start with the basics** if you are new to `Result<T, E>` and the `?` operator.
2. **Follow the application guide** to design domain-specific error types with
   consistent wire responses.
3. **Read the comparison** to understand how `masterror` complements `thiserror`
   and `anyhow` instead of replacing them outright.
4. **Review patterns and troubleshooting** when you need concrete recipes for
   mapping third-party errors, logging, and testing.

Each page contains runnable examples. Copy them into a new binary crate or an
`examples/` folder, run `cargo run`, and experiment.

## Related documentation

- [`README.md`](../README.md) and [`docs.rs/masterror`](https://docs.rs/masterror)
  for API reference and feature lists.
- [`masterror-derive`](../../masterror-derive/README.md) to explore the derive
  macro internals and advanced formatting capabilities.
- [`masterror-template`](../../masterror-template/README.md) for the shared
  template parser used by the derive macros.

Feedback and suggestions are welcome — open an issue or discussion on
[GitHub](https://github.com/RAprogramm/masterror).
