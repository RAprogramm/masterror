// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Formatter code generation for template placeholders.
//!
//! This module handles the generation of formatting code for template
//! placeholders, converting formatter specifications into appropriate
//! `core::fmt` trait calls. It supports all standard Rust format traits
//! including Display, Debug, Pointer, and various numeric formatting options.

use masterror_template::template::{TemplateFormatter, TemplateFormatterKind};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

/// Determines if a formatter requires the pointer value directly.
///
/// Pointer formatters need access to the actual pointer value rather than a
/// reference.
///
/// # Arguments
///
/// * `formatter` - The template formatter specification
///
/// # Returns
///
/// `true` if the formatter is a Pointer formatter, `false` otherwise
pub fn needs_pointer_value(formatter: &TemplateFormatter) -> bool {
    formatter.kind() == TemplateFormatterKind::Pointer
}

/// Checks if a placeholder requires the format macro engine.
///
/// Some formatters can be handled with simple trait calls, while others
/// require using the `write!` macro for proper formatting.
///
/// # Arguments
///
/// * `formatter` - The template formatter specification
///
/// # Returns
///
/// `true` if the formatter requires `write!` macro usage, `false` if a trait
/// call suffices
pub fn placeholder_requires_format_engine(formatter: &TemplateFormatter) -> bool {
    formatter.kind() != TemplateFormatterKind::Display || formatter.has_display_spec()
}

/// Generates formatting code for a resolved placeholder expression.
///
/// This function converts a placeholder and its formatter into the appropriate
/// code that will format the value at runtime. It handles all formatter types
/// and generates either direct trait calls or `write!` macro invocations.
///
/// # Arguments
///
/// * `resolved` - The resolved placeholder expression with pointer value
///   information
/// * `formatter` - The formatter specification (Display, Debug, Pointer, etc.)
///
/// # Returns
///
/// Token stream containing the formatting code for this placeholder
pub fn format_placeholder(
    resolved: super::placeholder::ResolvedPlaceholderExpr,
    formatter: TemplateFormatter
) -> TokenStream {
    let super::placeholder::ResolvedPlaceholderExpr {
        expr,
        pointer_value
    } = resolved;

    match formatter {
        TemplateFormatter::Display {
            spec: Some(spec)
        } => {
            let format_literal = Literal::string(&format!("{{:{spec}}}"));
            quote! {
                ::core::write!(f, #format_literal, #expr)?;
            }
        }
        TemplateFormatter::Display {
            spec: None
        } => {
            format_with_formatter_kind(expr, pointer_value, TemplateFormatterKind::Display, false)
        }
        TemplateFormatter::Debug {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Debug,
            alternate
        ),
        TemplateFormatter::LowerHex {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::LowerHex,
            alternate
        ),
        TemplateFormatter::UpperHex {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::UpperHex,
            alternate
        ),
        TemplateFormatter::Pointer {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Pointer,
            alternate
        ),
        TemplateFormatter::Binary {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Binary,
            alternate
        ),
        TemplateFormatter::Octal {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Octal,
            alternate
        ),
        TemplateFormatter::LowerExp {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::LowerExp,
            alternate
        ),
        TemplateFormatter::UpperExp {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::UpperExp,
            alternate
        )
    }
}

fn format_with_formatter_kind(
    expr: TokenStream,
    pointer_value: bool,
    kind: TemplateFormatterKind,
    alternate: bool
) -> TokenStream {
    let trait_name = formatter_trait_name(kind);
    match kind {
        TemplateFormatterKind::Display => format_with_trait(expr, trait_name),
        TemplateFormatterKind::Pointer => {
            format_pointer(expr, pointer_value, alternate, trait_name)
        }
        _ => {
            if let Some(specifier) = formatter_specifier(kind) {
                format_with_optional_alternate(expr, trait_name, specifier, alternate)
            } else {
                format_with_trait(expr, trait_name)
            }
        }
    }
}

fn formatter_trait_name(kind: TemplateFormatterKind) -> &'static str {
    match kind {
        TemplateFormatterKind::Display => "Display",
        TemplateFormatterKind::Debug => "Debug",
        TemplateFormatterKind::LowerHex => "LowerHex",
        TemplateFormatterKind::UpperHex => "UpperHex",
        TemplateFormatterKind::Pointer => "Pointer",
        TemplateFormatterKind::Binary => "Binary",
        TemplateFormatterKind::Octal => "Octal",
        TemplateFormatterKind::LowerExp => "LowerExp",
        TemplateFormatterKind::UpperExp => "UpperExp"
    }
}

fn formatter_specifier(kind: TemplateFormatterKind) -> Option<char> {
    match kind {
        TemplateFormatterKind::Display | TemplateFormatterKind::Pointer => None,
        TemplateFormatterKind::Debug => Some('?'),
        TemplateFormatterKind::LowerHex => Some('x'),
        TemplateFormatterKind::UpperHex => Some('X'),
        TemplateFormatterKind::Binary => Some('b'),
        TemplateFormatterKind::Octal => Some('o'),
        TemplateFormatterKind::LowerExp => Some('e'),
        TemplateFormatterKind::UpperExp => Some('E')
    }
}

fn format_with_trait(expr: TokenStream, trait_name: &str) -> TokenStream {
    let trait_ident = format_ident!("{}", trait_name);
    quote! {
        ::core::fmt::#trait_ident::fmt(#expr, f)?;
    }
}

fn format_with_optional_alternate(
    expr: TokenStream,
    trait_name: &str,
    specifier: char,
    alternate: bool
) -> TokenStream {
    if alternate {
        format_with_alternate(expr, specifier)
    } else {
        format_with_trait(expr, trait_name)
    }
}

fn format_with_alternate(expr: TokenStream, specifier: char) -> TokenStream {
    let format_string = format!("{{:#{}}}", specifier);
    quote! {
        ::core::write!(f, #format_string, #expr)?;
    }
}

fn format_pointer(
    expr: TokenStream,
    pointer_value: bool,
    alternate: bool,
    trait_name: &str
) -> TokenStream {
    if alternate {
        format_with_alternate(expr, 'p')
    } else if pointer_value {
        let trait_ident = format_ident!("{}", trait_name);
        quote! {{
            let value = #expr;
            ::core::fmt::#trait_ident::fmt(&value, f)?;
        }}
    } else {
        format_with_trait(expr, trait_name)
    }
}

#[cfg(test)]
mod tests {
    use masterror_template::template::TemplateFormatter;
    use quote::quote;

    use super::*;

    #[test]
    fn test_needs_pointer_value_returns_true_for_pointer_formatter() {
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        assert!(needs_pointer_value(&formatter));
    }

    #[test]
    fn test_needs_pointer_value_returns_false_for_display_formatter() {
        let formatter = TemplateFormatter::Display {
            spec: None
        };
        assert!(!needs_pointer_value(&formatter));
    }

    #[test]
    fn test_needs_pointer_value_returns_false_for_debug_formatter() {
        let formatter = TemplateFormatter::Debug {
            alternate: false
        };
        assert!(!needs_pointer_value(&formatter));
    }

    #[test]
    fn test_placeholder_requires_format_engine_for_display_with_spec() {
        let formatter = TemplateFormatter::Display {
            spec: Some(">10".into())
        };
        assert!(placeholder_requires_format_engine(&formatter));
    }

    #[test]
    fn test_placeholder_requires_format_engine_for_simple_display() {
        let formatter = TemplateFormatter::Display {
            spec: None
        };
        assert!(!placeholder_requires_format_engine(&formatter));
    }

    #[test]
    fn test_placeholder_requires_format_engine_for_debug() {
        let formatter = TemplateFormatter::Debug {
            alternate: false
        };
        assert!(placeholder_requires_format_engine(&formatter));
    }

    #[test]
    fn test_format_placeholder_display_with_spec() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Display {
            spec: Some(">10".into())
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:>10}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_display_without_spec() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Display {
            spec: None
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::Display::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_debug_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Debug {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::Debug::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_debug_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Debug {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#?}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_lower_hex_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::LowerHex {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::LowerHex::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_lower_hex_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::LowerHex {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#x}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_upper_hex_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::UpperHex {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::UpperHex::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_upper_hex_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::UpperHex {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#X}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_pointer_normal_with_reference() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::Pointer::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_pointer_normal_with_pointer_value() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::pointer(quote!(value));
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {{
            let value = value;
            ::core::fmt::Pointer::fmt(&value, f)?;
        }};
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_pointer_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Pointer {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#p}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_binary_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Binary {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::Binary::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_binary_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Binary {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#b}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_octal_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Octal {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::Octal::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_octal_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::Octal {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#o}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_lower_exp_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::LowerExp {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::LowerExp::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_lower_exp_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::LowerExp {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#e}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_upper_exp_normal() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::UpperExp {
            alternate: false
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::fmt::UpperExp::fmt(value, f)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_format_placeholder_upper_exp_alternate() {
        let resolved = super::super::placeholder::ResolvedPlaceholderExpr::new(quote!(value));
        let formatter = TemplateFormatter::UpperExp {
            alternate: true
        };
        let result = format_placeholder(resolved, formatter);
        let expected = quote! {
            ::core::write!(f, "{:#E}", value)?;
        };
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_formatter_trait_name_all_kinds() {
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::Display),
            "Display"
        );
        assert_eq!(formatter_trait_name(TemplateFormatterKind::Debug), "Debug");
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::LowerHex),
            "LowerHex"
        );
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::UpperHex),
            "UpperHex"
        );
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::Pointer),
            "Pointer"
        );
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::Binary),
            "Binary"
        );
        assert_eq!(formatter_trait_name(TemplateFormatterKind::Octal), "Octal");
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::LowerExp),
            "LowerExp"
        );
        assert_eq!(
            formatter_trait_name(TemplateFormatterKind::UpperExp),
            "UpperExp"
        );
    }

    #[test]
    fn test_formatter_specifier_all_kinds() {
        assert_eq!(formatter_specifier(TemplateFormatterKind::Display), None);
        assert_eq!(formatter_specifier(TemplateFormatterKind::Pointer), None);
        assert_eq!(formatter_specifier(TemplateFormatterKind::Debug), Some('?'));
        assert_eq!(
            formatter_specifier(TemplateFormatterKind::LowerHex),
            Some('x')
        );
        assert_eq!(
            formatter_specifier(TemplateFormatterKind::UpperHex),
            Some('X')
        );
        assert_eq!(
            formatter_specifier(TemplateFormatterKind::Binary),
            Some('b')
        );
        assert_eq!(formatter_specifier(TemplateFormatterKind::Octal), Some('o'));
        assert_eq!(
            formatter_specifier(TemplateFormatterKind::LowerExp),
            Some('e')
        );
        assert_eq!(
            formatter_specifier(TemplateFormatterKind::UpperExp),
            Some('E')
        );
    }
}
