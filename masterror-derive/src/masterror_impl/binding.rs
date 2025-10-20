// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Field binding and destructuring for error types.
//!
//! This module provides functionality for generating pattern matching and
//! field binding code during error conversion. It handles:
//!
//! - Destructuring struct fields (named, unnamed, unit)
//! - Destructuring enum variant fields
//! - Generating unique binding identifiers
//! - Creating field usage tokens to satisfy borrow checker
//!
//! The binding system ensures that all error fields are properly extracted
//! and available for attachment to the resulting `masterror::Error`.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::input::{Field, Fields, VariantData};

/// Represents a field with its generated binding identifier.
///
/// This structure associates an error field with the identifier used to
/// bind it in pattern matching expressions.
pub struct BoundField<'a> {
    /// Reference to the original field definition
    pub field:   &'a Field,
    /// Identifier used to bind this field in patterns
    pub binding: Ident
}

/// Generates destructuring pattern and bindings for struct fields.
///
/// Creates a pattern that destructures a struct value and binds each field
/// to a unique identifier. Handles named fields, tuple fields, and unit
/// structs.
///
/// # Arguments
///
/// * `ident` - The struct type identifier
/// * `fields` - The struct's field definitions
///
/// # Returns
///
/// A tuple of:
/// - `TokenStream` - The destructuring pattern (e.g., `let MyStruct { field1,
///   field2 } = value;`)
/// - `Vec<BoundField>` - List of bound fields with their bindings
///
/// # Examples
///
/// ```ignore
/// // For named fields:
/// struct Error { code: u32, message: String }
/// // Generates: let Error { code, message } = value;
///
/// // For unnamed fields:
/// struct Error(u32, String);
/// // Generates: let Error(__field0, __field1) = value;
///
/// // For unit struct:
/// struct Error;
/// // Generates: let _ = value;
/// ```
pub fn bind_struct_fields<'a>(
    ident: &Ident,
    fields: &'a Fields
) -> (TokenStream, Vec<BoundField<'a>>) {
    match fields {
        Fields::Unit => (quote!(let _ = value;), Vec::new()),
        Fields::Named(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            let pattern_tokens = quote!(let #ident { #(#pattern),* } = value;);
            (pattern_tokens, bound)
        }
        Fields::Unnamed(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            let pattern_tokens = quote!(let #ident(#(#pattern),*) = value;);
            (pattern_tokens, bound)
        }
    }
}

/// Generates destructuring pattern and bindings for enum variant fields.
///
/// Creates a pattern that matches an enum variant and binds its fields.
/// Similar to `bind_struct_fields` but for enum variants.
///
/// # Arguments
///
/// * `enum_ident` - The enum type identifier
/// * `variant` - The variant definition with its fields
///
/// # Returns
///
/// A tuple of:
/// - `TokenStream` - The match pattern (e.g., `MyEnum::Variant { field }`)
/// - `Vec<BoundField>` - List of bound fields with their bindings
///
/// # Examples
///
/// ```ignore
/// // For named fields:
/// enum Error { Auth { message: String } }
/// // Generates: Error::Auth { message }
///
/// // For unnamed fields:
/// enum Error { Io(std::io::Error) }
/// // Generates: Error::Io(__field0)
///
/// // For unit variant:
/// enum Error { NotFound }
/// // Generates: Error::NotFound
/// ```
pub fn bind_variant_fields<'a>(
    enum_ident: &Ident,
    variant: &'a VariantData
) -> (TokenStream, Vec<BoundField<'a>>) {
    let variant_ident = &variant.ident;

    match &variant.fields {
        Fields::Unit => (quote!(#enum_ident::#variant_ident), Vec::new()),
        Fields::Named(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            (quote!(#enum_ident::#variant_ident { #(#pattern),* }), bound)
        }
        Fields::Unnamed(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            (quote!(#enum_ident::#variant_ident(#(#pattern),*)), bound)
        }
    }
}

/// Generates field usage tokens to prevent unused variable warnings.
///
/// Creates a statement that references all bound fields to satisfy the Rust
/// compiler's unused variable detection. This is necessary when fields are
/// bound but not all are used in the conversion.
///
/// # Arguments
///
/// * `bound_fields` - List of bound fields to reference
///
/// # Returns
///
/// A `TokenStream` containing the field usage statement, or empty if no fields.
///
/// # Examples
///
/// ```ignore
/// // For fields: [field1, field2]
/// // Generates: let _ = (&field1, &field2);
///
/// // For no fields:
/// // Generates: (empty)
/// ```
pub fn field_usage_tokens(bound_fields: &[BoundField<'_>]) -> TokenStream {
    if bound_fields.is_empty() {
        return TokenStream::new();
    }

    let names = bound_fields.iter().map(|field| &field.binding);
    quote! {
        let _ = (#(&#names),*);
    }
}

/// Generates a unique binding identifier for a field.
///
/// For named fields, uses the field's name directly. For unnamed fields,
/// generates a unique identifier based on the field's index.
///
/// # Arguments
///
/// * `field` - The field to generate a binding for
///
/// # Returns
///
/// An `Ident` to use as the binding name in patterns.
///
/// # Examples
///
/// ```ignore
/// // For named field:
/// struct Error { message: String }
/// // Returns: message
///
/// // For unnamed field at index 0:
/// struct Error(String);
/// // Returns: __field0
/// ```
pub fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::parse_quote;

    use super::*;

    fn create_test_field(ident: Option<Ident>, index: usize) -> Field {
        let ty = parse_quote!(String);
        let member = match &ident {
            Some(name) => syn::Member::Named(name.clone()),
            None => syn::Member::Unnamed(syn::Index::from(index))
        };

        Field {
            ident,
            member,
            ty,
            index,
            span: Span::call_site(),
            attrs: Default::default()
        }
    }

    #[test]
    fn test_binding_ident_named_field() {
        let field = create_test_field(Some(format_ident!("message")), 0);
        let binding = binding_ident(&field);
        assert_eq!(binding.to_string(), "message");
    }

    #[test]
    fn test_binding_ident_unnamed_field() {
        let field = create_test_field(None, 2);
        let binding = binding_ident(&field);
        assert_eq!(binding.to_string(), "__field2");
    }

    #[test]
    fn test_field_usage_tokens_empty() {
        let bound_fields = vec![];
        let result = field_usage_tokens(&bound_fields);
        assert!(result.is_empty());
    }

    #[test]
    fn test_field_usage_tokens_single() {
        let field = create_test_field(Some(format_ident!("field1")), 0);
        let binding = binding_ident(&field);
        let bound = vec![BoundField {
            field: &field,
            binding
        }];

        let result = field_usage_tokens(&bound);
        let result_str = result.to_string();
        assert!(result_str.contains("field1"));
    }

    #[test]
    fn test_bind_struct_fields_unit() {
        let ident = format_ident!("MyError");
        let fields = Fields::Unit;

        let (pattern, bound) = bind_struct_fields(&ident, &fields);
        assert_eq!(pattern.to_string(), "let _ = value ;");
        assert!(bound.is_empty());
    }

    #[test]
    fn test_bind_struct_fields_named() {
        let ident = format_ident!("MyError");
        let field = create_test_field(Some(format_ident!("message")), 0);
        let fields = Fields::Named(vec![field]);

        let (pattern, bound) = bind_struct_fields(&ident, &fields);
        let pattern_str = pattern.to_string();

        assert!(pattern_str.contains("MyError"));
        assert!(pattern_str.contains("message"));
        assert_eq!(bound.len(), 1);
        assert_eq!(bound[0].binding.to_string(), "message");
    }

    #[test]
    fn test_bind_struct_fields_unnamed() {
        let ident = format_ident!("MyError");
        let field = create_test_field(None, 0);
        let fields = Fields::Unnamed(vec![field]);

        let (pattern, bound) = bind_struct_fields(&ident, &fields);
        let pattern_str = pattern.to_string();

        assert!(pattern_str.contains("MyError"));
        assert!(pattern_str.contains("__field0"));
        assert_eq!(bound.len(), 1);
        assert_eq!(bound[0].binding.to_string(), "__field0");
    }
}
