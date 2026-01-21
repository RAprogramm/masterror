// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Clippy lint suppression for generated code.
//!
//! When generating trait implementations for types with lifetime parameters,
//! clippy may emit `needless_lifetimes` or `elidable_lifetime_names` warnings.
//! These warnings conflict with project-level `forbid` directives, so we
//! conditionally suppress them only when the type actually has lifetimes.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

/// Generates conditional clippy lint allows for lifetime-related warnings.
///
/// Returns `#[allow(...)]` attributes only when the generics contain lifetime
/// parameters. This prevents conflicts with project-level `forbid` directives
/// while still suppressing false positives in generated code.
///
/// # Arguments
///
/// * `generics` - The generics from the type definition
///
/// # Returns
///
/// Token stream with allow attributes if lifetimes present, empty otherwise
pub fn lifetime_lint_allows(generics: &Generics) -> TokenStream {
    if generics.lifetimes().next().is_some() {
        quote! {
            #[allow(clippy::elidable_lifetime_names, clippy::needless_lifetimes)]
        }
    } else {
        TokenStream::new()
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_no_lifetimes_returns_empty() {
        let generics: Generics = parse_quote!();
        let result = lifetime_lint_allows(&generics);
        assert!(result.is_empty());
    }

    #[test]
    fn test_type_params_only_returns_empty() {
        let generics: Generics = parse_quote!(<T, U>);
        let result = lifetime_lint_allows(&generics);
        assert!(result.is_empty());
    }

    #[test]
    fn test_with_lifetime_returns_allows() {
        let generics: Generics = parse_quote!(<'a>);
        let result = lifetime_lint_allows(&generics);
        let output = result.to_string();
        assert!(output.contains("allow"));
        assert!(output.contains("elidable_lifetime_names"));
        assert!(output.contains("needless_lifetimes"));
    }

    #[test]
    fn test_mixed_params_with_lifetime_returns_allows() {
        let generics: Generics = parse_quote!(<'a, T, 'b>);
        let result = lifetime_lint_allows(&generics);
        let output = result.to_string();
        assert!(output.contains("allow"));
    }

    #[test]
    fn test_const_generics_only_returns_empty() {
        let generics: Generics = parse_quote!(<const N: usize>);
        let result = lifetime_lint_allows(&generics);
        assert!(result.is_empty());
    }
}
