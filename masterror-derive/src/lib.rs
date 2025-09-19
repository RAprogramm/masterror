//! Derive macro for `masterror::Error`.
//!
//! This crate is not intended to be used directly. Re-exported as
//! `masterror::Error`.

mod display;
mod error_trait;
mod from_impl;
mod input;
mod span;
mod template_support;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, parse_macro_input};

#[proc_macro_derive(Error, attributes(error, source, from, backtrace))]
pub fn derive_error(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    match expand(input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into()
    }
}

fn expand(input: DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let parsed = input::parse_input(input)?;
    let display_impl = display::expand(&parsed)?;
    let error_impl = error_trait::expand(&parsed)?;
    let from_impls = from_impl::expand(&parsed)?;

    Ok(quote! {
        #display_impl
        #error_impl
        #(#from_impls)*
    })
}
