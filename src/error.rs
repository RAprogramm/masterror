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
//! use masterror::error::template::{ErrorTemplate, TemplateFormatter};
//!
//! let template = ErrorTemplate::parse("{code:#x} → {payload:?}").expect("parse");
//! let mut placeholders = template.placeholders();
//!
//! let code = placeholders.next().expect("code placeholder");
//! assert!(matches!(
//!     code.formatter(),
//!     TemplateFormatter::LowerHex {
//!         alternate: true
//!     }
//! ));
//!
//! let payload = placeholders.next().expect("payload placeholder");
//! assert_eq!(
//!     payload.formatter(),
//!     TemplateFormatter::Debug {
//!         alternate: false
//!     }
//! );
//! ```

/// Parser and formatter helpers for `#[error("...")]` templates.
pub mod template;
