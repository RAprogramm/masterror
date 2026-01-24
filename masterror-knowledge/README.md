<!--
SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

<div align="center">
  <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror-knowledge.png" alt="masterror-knowledge" width="400"/>

  <h1>masterror-knowledge</h1>
  <p><strong>Knowledge base for Rust compiler errors and best practices</strong></p>

  [![Crates.io](https://img.shields.io/crates/v/masterror-knowledge)](https://crates.io/crates/masterror-knowledge)
  [![docs.rs](https://img.shields.io/docsrs/masterror-knowledge)](https://docs.rs/masterror-knowledge)
  ![License](https://img.shields.io/badge/License-MIT-informational)
</div>

---

## Overview

`masterror-knowledge` provides a comprehensive knowledge base of Rust compiler error explanations and best practices. It powers the [masterror-cli](https://github.com/RAprogramm/masterror-cli) tool, enabling developers to quickly understand and fix compiler errors.

## Features

- **31+ Error Explanations** — Detailed explanations for common Rust compiler errors (E0001-E0792)
- **15 RustManifest Best Practices** — Guidelines for writing idiomatic Rust code
- **Multi-language Support** — Available in English, Russian, and Korean
- **Zero Dependencies** — Lightweight, no runtime overhead
- **Compile-time Lookup** — Fast pattern matching using Aho-Corasick algorithm

## Supported Languages

| Feature | Language |
|---------|----------|
| (default) | English |
| `lang-ru` | Russian |
| `lang-ko` | Korean |

## Installation

```toml
[dependencies]
masterror-knowledge = "0.1"
```

With additional languages:

```toml
[dependencies]
masterror-knowledge = { version = "0.1", features = ["lang-ru", "lang-ko"] }
```

## Usage

```rust
use masterror_knowledge::{Lang, lookup_error, lookup_practice};

// Get explanation for error E0382 (borrow of moved value)
if let Some(explanation) = lookup_error("E0382", Lang::En) {
    println!("{}", explanation);
}

// Get best practice by ID
if let Some(practice) = lookup_practice("RM001", Lang::En) {
    println!("{}", practice);
}
```

## Error Codes Covered

The knowledge base includes explanations for errors related to:

- **Ownership & Borrowing** — E0382, E0499, E0502, E0505, E0507
- **Lifetimes** — E0106, E0621, E0759, E0792
- **Type System** — E0277, E0308, E0412, E0425
- **Traits** — E0046, E0119, E0277
- **Patterns** — E0004, E0005, E0026, E0027
- **And more...**

## Related

- [masterror](https://github.com/RAprogramm/masterror) — Framework-agnostic application error types
- [masterror-cli](https://github.com/RAprogramm/masterror-cli) — CLI tool using this knowledge base

## License

MIT
