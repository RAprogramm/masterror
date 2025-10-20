// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Placeholder resolution and expression handling.
//!
//! This module provides types and utilities for resolving template placeholders
//! into concrete expressions that can be formatted. It handles the distinction
//! between pointer values and references, ensuring correct formatting behavior
//! for different types.

use proc_macro2::TokenStream;

/// A resolved placeholder expression with metadata about pointer handling.
///
/// This type represents a placeholder that has been resolved to a concrete
/// expression, along with information about whether it should be treated
/// as a pointer value for formatting purposes.
#[derive(Debug)]
pub struct ResolvedPlaceholderExpr {
    /// The token stream representing the resolved expression
    pub expr:          TokenStream,
    /// Whether this expression should be treated as a pointer value
    pub pointer_value: bool
}

impl ResolvedPlaceholderExpr {
    /// Creates a new resolved placeholder expression with a reference.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression tokens to wrap
    ///
    /// # Returns
    ///
    /// A new `ResolvedPlaceholderExpr` configured for reference handling
    pub fn new(expr: TokenStream) -> Self {
        Self::with(expr, false)
    }

    /// Creates a new resolved placeholder expression with a pointer value.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression tokens to wrap
    ///
    /// # Returns
    ///
    /// A new `ResolvedPlaceholderExpr` configured for pointer value handling
    pub fn pointer(expr: TokenStream) -> Self {
        Self::with(expr, true)
    }

    /// Creates a new resolved placeholder expression with explicit pointer
    /// handling.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression tokens to wrap
    /// * `pointer_value` - Whether to treat this as a pointer value
    ///
    /// # Returns
    ///
    /// A new `ResolvedPlaceholderExpr` with the specified configuration
    pub fn with(expr: TokenStream, pointer_value: bool) -> Self {
        Self {
            expr,
            pointer_value
        }
    }

    /// Returns a clone of the expression tokens.
    ///
    /// # Returns
    ///
    /// A clone of the internal expression token stream
    pub fn expr_tokens(&self) -> TokenStream {
        self.expr.clone()
    }
}

/// Determines if a type prefers pointer value formatting.
///
/// Some types like raw pointers, immutable references, and `NonNull` should
/// be formatted by value when using the Pointer formatter, rather than taking
/// a reference to them.
///
/// # Arguments
///
/// * `ty` - The type to check
///
/// # Returns
///
/// `true` if the type should be formatted by value with Pointer formatter,
/// `false` otherwise
pub fn pointer_prefers_value(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Ptr(_) => true,
        syn::Type::Reference(reference) => reference.mutability.is_none(),
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "NonNull")
            .unwrap_or(false),
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_resolved_placeholder_expr_new() {
        let expr = quote!(value);
        let resolved = ResolvedPlaceholderExpr::new(expr.clone());
        assert_eq!(resolved.expr.to_string(), expr.to_string());
        assert!(!resolved.pointer_value);
    }

    #[test]
    fn test_resolved_placeholder_expr_pointer() {
        let expr = quote!(value);
        let resolved = ResolvedPlaceholderExpr::pointer(expr.clone());
        assert_eq!(resolved.expr.to_string(), expr.to_string());
        assert!(resolved.pointer_value);
    }

    #[test]
    fn test_resolved_placeholder_expr_with_false() {
        let expr = quote!(value);
        let resolved = ResolvedPlaceholderExpr::with(expr.clone(), false);
        assert_eq!(resolved.expr.to_string(), expr.to_string());
        assert!(!resolved.pointer_value);
    }

    #[test]
    fn test_resolved_placeholder_expr_with_true() {
        let expr = quote!(value);
        let resolved = ResolvedPlaceholderExpr::with(expr.clone(), true);
        assert_eq!(resolved.expr.to_string(), expr.to_string());
        assert!(resolved.pointer_value);
    }

    #[test]
    fn test_resolved_placeholder_expr_tokens() {
        let expr = quote!(some_value);
        let resolved = ResolvedPlaceholderExpr::new(expr.clone());
        let tokens = resolved.expr_tokens();
        assert_eq!(tokens.to_string(), expr.to_string());
    }

    #[test]
    fn test_pointer_prefers_value_for_raw_pointer() {
        let ty: syn::Type = syn::parse_quote!(*const i32);
        assert!(pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_mutable_raw_pointer() {
        let ty: syn::Type = syn::parse_quote!(*mut i32);
        assert!(pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_immutable_reference() {
        let ty: syn::Type = syn::parse_quote!(&i32);
        assert!(pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_mutable_reference() {
        let ty: syn::Type = syn::parse_quote!(&mut i32);
        assert!(!pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_non_null() {
        let ty: syn::Type = syn::parse_quote!(NonNull<i32>);
        assert!(pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_non_null_qualified() {
        let ty: syn::Type = syn::parse_quote!(std::ptr::NonNull<i32>);
        assert!(pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_regular_type() {
        let ty: syn::Type = syn::parse_quote!(String);
        assert!(!pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_generic_type() {
        let ty: syn::Type = syn::parse_quote!(Vec<i32>);
        assert!(!pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_tuple() {
        let ty: syn::Type = syn::parse_quote!((i32, String));
        assert!(!pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_array() {
        let ty: syn::Type = syn::parse_quote!([i32; 10]);
        assert!(!pointer_prefers_value(&ty));
    }

    #[test]
    fn test_pointer_prefers_value_for_slice() {
        let ty: syn::Type = syn::parse_quote!([i32]);
        assert!(!pointer_prefers_value(&ty));
    }
}
