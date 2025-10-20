// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Field binding identifier generation.
//!
//! Provides utilities for generating unique binding identifiers for struct
//! and enum fields during pattern matching in Error trait implementations.

use proc_macro2::Ident;
use quote::format_ident;

use crate::input::Field;

/// Generates a unique binding identifier for a field.
///
/// For named fields, returns the field name. For unnamed fields, generates
/// a binding like `__field0`, `__field1`, etc. based on field index.
///
/// # Arguments
///
/// * `field` - The field to generate a binding for
///
/// # Returns
///
/// A unique identifier suitable for pattern matching
///
/// # Examples
///
/// ```ignore
/// let field = Field { ident: Some("value"), index: 0, .. };
/// let binding = binding_ident(&field);
/// assert_eq!(binding.to_string(), "value");
///
/// let unnamed_field = Field { ident: None, index: 2, .. };
/// let binding = binding_ident(&unnamed_field);
/// assert_eq!(binding.to_string(), "__field2");
/// ```
pub(crate) fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::Member;

    use super::*;
    use crate::input::FieldAttrs;

    fn make_field(ident: Option<&str>, index: usize) -> Field {
        Field {
            ident: ident.map(|s| syn::Ident::new(s, Span::call_site())),
            member: if let Some(s) = ident {
                Member::Named(syn::Ident::new(s, Span::call_site()))
            } else {
                Member::Unnamed(syn::Index {
                    index: index as u32,
                    span:  Span::call_site()
                })
            },
            ty: syn::parse_quote!(String),
            index,
            attrs: FieldAttrs::default(),
            span: Span::call_site()
        }
    }

    #[test]
    fn test_binding_ident_named_field() {
        let field = make_field(Some("value"), 0);
        let binding = binding_ident(&field);
        assert_eq!(binding.to_string(), "value");
    }

    #[test]
    fn test_binding_ident_unnamed_field_index_0() {
        let field = make_field(None, 0);
        let binding = binding_ident(&field);
        assert_eq!(binding.to_string(), "__field0");
    }

    #[test]
    fn test_binding_ident_unnamed_field_index_5() {
        let field = make_field(None, 5);
        let binding = binding_ident(&field);
        assert_eq!(binding.to_string(), "__field5");
    }

    #[test]
    fn test_binding_ident_preserves_named_identifier() {
        let field = make_field(Some("source"), 1);
        let binding = binding_ident(&field);
        assert_eq!(binding.to_string(), "source");
    }
}
