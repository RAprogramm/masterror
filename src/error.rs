//! Utilities for building custom error derive infrastructure.
//!
//! This module exposes lower-level building blocks that will eventually power
//! a native replacement for the `thiserror` derive. The initial goal is to
//! parse and validate display templates (`#[error("...")]`) in a reusable
//! and well-tested manner so that future procedural macros can focus on
//! generating code.
//!
//! The API is intentionally low level. It makes no assumptions about how the
//! parsed data is going to be used and instead provides precise spans and
//! formatting metadata that higher-level code can rely on.
//!
//! ## Formatter traits
//!
//! `TemplateFormatter` enumerates the formatting modes supported by
//! `#[error("...")]` placeholders. It mirrors the formatter detection logic in
//! `thiserror` v2 so migrating existing derives is a drop-in change.
//! `TemplateFormatter::is_alternate()` surfaces the `#` flag, and
//! [`TemplateFormatterKind`](crate::error::template::TemplateFormatterKind)
//! describes the underlying `core::fmt` trait with helpers like
//! `specifier()`/`supports_alternate()` for programmatic inspection.
//!
//! ```rust
//! use core::ptr;
//!
//! use masterror::Error;
//!
//! #[derive(Debug, Error)]
//! #[error(
//!     "debug={payload:?}, hex={id:#x}, ptr={ptr:p}, bin={mask:#b}, \
//!      oct={mask:o}, lower={ratio:e}, upper={ratio:E}"
//! )]
//! struct FormatterShowcase {
//!     id:      u32,
//!     payload: String,
//!     ptr:     *const u8,
//!     mask:    u8,
//!     ratio:   f32
//! }
//!
//! let err = FormatterShowcase {
//!     id:      0x2a,
//!     payload: "hello".into(),
//!     ptr:     ptr::null(),
//!     mask:    0b1010_0001,
//!     ratio:   0.15625
//! };
//!
//! let rendered = err.to_string();
//! assert!(rendered.contains("debug=\"hello\""));
//! assert!(rendered.contains("hex=0x2a"));
//! assert!(rendered.contains("ptr=0x0"));
//! assert!(rendered.contains("bin=0b10100001"));
//! assert!(rendered.contains("oct=241"));
//! assert!(rendered.contains("lower=1.5625e-1"));
//! assert!(rendered.contains("upper=1.5625E-1"));
//! ```
//!
//! Programmatic consumers can inspect placeholders and their requested
//! formatters via [`ErrorTemplate`](crate::error::template::ErrorTemplate):
//!
//! ```rust
//! use masterror::error::template::{ErrorTemplate, TemplateFormatter, TemplateFormatterKind};
//!
//! let template = ErrorTemplate::parse("{code:#x} → {payload:?}").expect("parse");
//! let mut placeholders = template.placeholders();
//!
//! let code = placeholders.next().expect("code placeholder");
//! let code_formatter = code.formatter();
//! assert!(matches!(
//!     code_formatter,
//!     TemplateFormatter::LowerHex {
//!         alternate: true
//!     }
//! ));
//! let code_kind = code_formatter.kind();
//! assert_eq!(code_kind, TemplateFormatterKind::LowerHex);
//! assert!(code_formatter.is_alternate());
//! assert_eq!(code_kind.specifier(), Some('x'));
//! assert!(code_kind.supports_alternate());
//! let lowered = TemplateFormatter::from_kind(code_kind, false);
//! assert!(matches!(
//!     lowered,
//!     TemplateFormatter::LowerHex {
//!         alternate: false
//!     }
//! ));
//!
//! let payload = placeholders.next().expect("payload placeholder");
//! let payload_formatter = payload.formatter();
//! assert_eq!(
//!     payload_formatter,
//!     &TemplateFormatter::Debug {
//!         alternate: false
//!     }
//! );
//! let payload_kind = payload_formatter.kind();
//! assert_eq!(payload_kind, TemplateFormatterKind::Debug);
//! assert_eq!(payload_kind.specifier(), Some('?'));
//! assert!(payload_kind.supports_alternate());
//! let pretty_debug = TemplateFormatter::from_kind(payload_kind, true);
//! assert!(matches!(
//!     pretty_debug,
//!     TemplateFormatter::Debug {
//!         alternate: true
//!     }
//! ));
//! assert!(pretty_debug.is_alternate());
//! ```

/// Parser and formatter helpers for `#[error("...")]` templates.
pub mod template;
