// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Derive macro for `masterror::Error`.
//!
//! This crate is not intended to be used directly. Re-exported as
//! `masterror::Error`.

mod app_error_impl;
mod display;
mod error_trait;
mod from_impl;
mod input;
mod lint;
mod masterror_impl;
mod span;
mod template_support;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Error, parse_macro_input};

#[proc_macro_derive(Error, attributes(error, source, from, backtrace, app_error, provide))]
pub fn derive_error(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    match expand(input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into()
    }
}

#[proc_macro_derive(
    Masterror,
    attributes(error, source, from, backtrace, masterror, provide)
)]
pub fn derive_masterror(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    match expand_masterror(input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into()
    }
}

fn expand(input: DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let deprecated = references_deprecated(&input);
    let parsed = input::parse_input(input)?;
    let display_impl = display::expand(&parsed)?;
    let error_impl = error_trait::expand(&parsed)?;
    let from_impls = from_impl::expand(&parsed)?;
    let app_error_impls = app_error_impl::expand(&parsed)?;
    Ok(allow_deprecated(
        deprecated,
        quote! {
            #display_impl
            #error_impl
            #(#from_impls)*
            #(#app_error_impls)*
        }
    ))
}

fn expand_masterror(input: DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let deprecated = references_deprecated(&input);
    let parsed = input::parse_input(input)?;
    let display_impl = display::expand(&parsed)?;
    let error_impl = error_trait::expand(&parsed)?;
    let from_impls = from_impl::expand(&parsed)?;
    let masterror_impl = masterror_impl::expand(&parsed)?;
    Ok(allow_deprecated(
        deprecated,
        quote! {
            #display_impl
            #error_impl
            #(#from_impls)*
            #masterror_impl
        }
    ))
}

/// Checks whether generated code will reference a deprecated item.
///
/// Returns `true` when the type itself or any enum variant carries a
/// `#[deprecated]` attribute, in which case the expansion must be shielded
/// from the `deprecated` lint.
fn references_deprecated(input: &DeriveInput) -> bool {
    fn has_deprecated(attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| attr.path().is_ident("deprecated"))
    }
    if has_deprecated(&input.attrs) {
        return true;
    }
    match &input.data {
        Data::Enum(data) => data
            .variants
            .iter()
            .any(|variant| has_deprecated(&variant.attrs)),
        _ => false
    }
}

/// Wraps generated implementations in `#[allow(deprecated)]` when needed.
///
/// Deriving on a `#[deprecated]` type (or an enum with deprecated variants)
/// must not trigger the `deprecated` lint in the expansion.
fn allow_deprecated(deprecated: bool, body: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if deprecated {
        quote! {
            #[allow(deprecated)]
            const _: () = {
                #body
            };
        }
    } else {
        body
    }
}
