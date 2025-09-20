# masterror-template

`masterror-template` packages the template parser shared by the [`masterror`][masterror] runtime crate and the [`masterror-derive`][derive] procedural macros. It understands the `#[error("...")]` formatting language popularised by `thiserror` v2, producing a structured representation that downstream code can inspect or render.

The crate is intentionally small: it exposes just enough API for advanced applications that want to inspect derived error displays, implement custom derive helpers, or perform static analysis over formatting placeholders.

## Installation

Add the crate alongside `masterror` if you need direct access to the parser:

```toml
[dependencies]
masterror-template = { version = "0.3.6" }
```

`masterror-template` targets Rust 1.90 and builds on stable and nightly toolchains alike.

## Parsing templates

Call [`ErrorTemplate::parse`](https://docs.rs/masterror-template/latest/masterror_template/template/struct.ErrorTemplate.html#method.parse) to turn an `&str` into a structured template:

```rust
use masterror_template::template::{ErrorTemplate, TemplateIdentifier};

fn inspect(template: &str) {
    let parsed = ErrorTemplate::parse(template).expect("valid template");

    for placeholder in parsed.placeholders() {
        match placeholder.identifier() {
            TemplateIdentifier::Named(name) => println!("named placeholder: {name}"),
            TemplateIdentifier::Positional(index) => println!("positional placeholder: {index}"),
            TemplateIdentifier::Implicit(index) => println!("implicit placeholder: {index}"),
        }
    }
}
```

The parser preserves literal text and exposes every placeholder with span metadata, making it straightforward to surface diagnostics or transform templates programmatically.

## Formatter metadata

Each [`TemplatePlaceholder`](https://docs.rs/masterror-template/latest/masterror_template/template/struct.TemplatePlaceholder.html) advertises the requested formatter through [`TemplateFormatter`](https://docs.rs/masterror-template/latest/masterror_template/template/enum.TemplateFormatter.html) and [`TemplateFormatterKind`](https://docs.rs/masterror-template/latest/masterror_template/template/enum.TemplateFormatterKind.html):

```rust
use masterror_template::template::{ErrorTemplate, TemplateFormatterKind};

let template = ErrorTemplate::parse("{value:#x}").expect("parse");
let placeholder = template.placeholders().next().expect("placeholder");
let formatter = placeholder.formatter();
assert_eq!(formatter.kind(), TemplateFormatterKind::LowerHex);
assert!(formatter.is_alternate());
```

This mirrors the formatting traits accepted by `core::fmt`, enabling consumers to route values through `Display`, `Debug`, hexadecimal, binary, pointer, or exponential renderers.

## Error reporting

Parsing failures produce [`TemplateError`](https://docs.rs/masterror-template/latest/masterror_template/template/enum.TemplateError.html) variants with precise byte ranges. The metadata simplifies IDE integrations and procedural macros that need to point at the offending part of the template.

```rust
use masterror_template::template::ErrorTemplate;

let err = ErrorTemplate::parse("{foo").unwrap_err();
assert!(matches!(err, masterror_template::template::TemplateError::UnterminatedPlaceholder { .. }));
```

## License

Dual licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[masterror]: https://crates.io/crates/masterror
[derive]: https://crates.io/crates/masterror-derive
