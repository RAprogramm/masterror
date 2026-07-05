// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, parse_quote};

use crate::input::{AppErrorSpec, ErrorData, ErrorInput, Fields, StructData, VariantData};

pub fn expand(input: &ErrorInput) -> Result<Vec<TokenStream>, Error> {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}

fn expand_struct(input: &ErrorInput, data: &StructData) -> Result<Vec<TokenStream>, Error> {
    let mut impls = Vec::new();
    if let Some(spec) = &data.app_error {
        impls.push(struct_app_error_impl(input, spec));
        if spec.code.is_some() {
            impls.push(struct_app_code_impl(input, spec));
        }
    }
    Ok(impls)
}

fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<Vec<TokenStream>, Error> {
    let mut impls = Vec::new();
    if variants.iter().any(|variant| variant.app_error.is_some()) {
        ensure_all_have_app_error(variants)?;
        impls.push(enum_app_error_impl(input, variants)?);
    }
    if variants.iter().any(|variant| {
        variant
            .app_error
            .as_ref()
            .is_some_and(|spec| spec.code.is_some())
    }) {
        ensure_all_have_app_code(variants)?;
        impls.push(enum_app_code_impl(input, variants));
    }
    Ok(impls)
}

fn ensure_all_have_app_error(variants: &[VariantData]) -> Result<(), Error> {
    for variant in variants {
        if variant.app_error.is_none() {
            return Err(Error::new(
                variant.span,
                "all variants must use #[app_error(...)] to derive AppError conversion"
            ));
        }
    }
    Ok(())
}

fn ensure_all_have_app_code(variants: &[VariantData]) -> Result<(), Error> {
    for variant in variants {
        match &variant.app_error {
            Some(spec) if spec.code.is_some() => {}
            Some(spec) => {
                return Err(Error::new(
                    spec.attribute_span,
                    "AppCode conversion requires `code = ...` in #[app_error(...)]"
                ));
            }
            None => {
                return Err(Error::new(
                    variant.span,
                    "all variants must use #[app_error(...)] with `code = ...` to derive AppCode conversion"
                ));
            }
        }
    }
    Ok(())
}

/// Ensures every variant agrees on the `no_source` flag.
///
/// Source attachment is a property of the whole enum conversion, so mixing
/// `no_source` and source-attaching variants is rejected.
fn ensure_consistent_no_source(variants: &[VariantData]) -> Result<bool, Error> {
    let mut specs = variants
        .iter()
        .filter_map(|variant| variant.app_error.as_ref());
    let no_source = specs.next().is_some_and(|spec| spec.no_source);
    for spec in specs {
        if spec.no_source != no_source {
            return Err(Error::new(
                spec.attribute_span,
                "`no_source` must be specified on every #[app_error(...)] variant or on none"
            ));
        }
    }
    Ok(no_source)
}

/// Clones the input generics and adds the bounds required by
/// `AppError::with_source` for the derived type itself.
fn source_bound_generics(input: &ErrorInput) -> syn::Generics {
    let ident = &input.ident;
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let mut generics = input.generics.clone();
    generics.make_where_clause().predicates.push(parse_quote! {
        #ident #ty_generics: core::error::Error + core::marker::Send + core::marker::Sync + 'static
    });
    generics
}

/// Wraps a conversion body into a `From<T> for AppError` implementation.
fn app_error_from_impl(
    ident: &syn::Ident,
    generics: &syn::Generics,
    body: TokenStream
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::AppError #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #body
            }
        }
    }
}

fn struct_app_error_impl(input: &ErrorInput, spec: &AppErrorSpec) -> TokenStream {
    let ident = &input.ident;
    let kind = &spec.kind;
    if spec.no_source {
        let body = if spec.expose_message {
            quote! {
                masterror::AppError::with(#kind, std::string::ToString::to_string(&value))
            }
        } else {
            quote! {
                {
                    let _ = value;
                    masterror::AppError::bare(#kind)
                }
            }
        };
        return app_error_from_impl(ident, &input.generics, body);
    }
    let generics = source_bound_generics(input);
    let body = if spec.expose_message {
        quote! {
            masterror::AppError::with(#kind, std::string::ToString::to_string(&value))
                .with_source(value)
        }
    } else {
        quote! {
            masterror::AppError::bare(#kind).with_source(value)
        }
    };
    app_error_from_impl(ident, &generics, body)
}

fn struct_app_code_impl(input: &ErrorInput, spec: &AppErrorSpec) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let code = spec.code.as_ref().expect("code presence checked");
    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::AppCode #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                let _ = value;
                #code
            }
        }
    }
}

fn enum_app_error_impl(
    input: &ErrorInput,
    variants: &[VariantData]
) -> Result<TokenStream, Error> {
    let ident = &input.ident;
    let no_source = ensure_consistent_no_source(variants)?;
    let mut arms = Vec::new();
    for variant in variants {
        let spec = variant.app_error.as_ref().expect("presence checked");
        let kind = &spec.kind;
        let pattern = variant_app_error_pattern(ident, variant);
        let body = if spec.expose_message {
            quote! {
                masterror::AppError::with(#kind, std::string::ToString::to_string(&value))
            }
        } else {
            quote! {
                masterror::AppError::bare(#kind)
            }
        };
        arms.push(quote! { #pattern => #body });
    }
    let base = quote! {
        match &value {
            #(#arms),*
        }
    };
    if no_source {
        return Ok(app_error_from_impl(ident, &input.generics, base));
    }
    let generics = source_bound_generics(input);
    let body = quote! {
        let base: masterror::AppError = #base;
        base.with_source(value)
    };
    Ok(app_error_from_impl(ident, &generics, body))
}

fn enum_app_code_impl(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut arms = Vec::new();
    for variant in variants {
        let spec = variant.app_error.as_ref().expect("presence checked");
        let pattern = variant_app_code_pattern(ident, variant);
        let code = spec.code.as_ref().expect("code presence checked");
        arms.push(quote! { #pattern => #code });
    }
    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::AppCode #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                match value {
                    #(#arms),*
                }
            }
        }
    }
}

fn variant_app_error_pattern(enum_ident: &syn::Ident, variant: &VariantData) -> TokenStream {
    let ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => quote! { #enum_ident::#ident },
        Fields::Named(_) => quote! { #enum_ident::#ident { .. } },
        Fields::Unnamed(_) => quote! { #enum_ident::#ident(..) }
    }
}

fn variant_app_code_pattern(enum_ident: &syn::Ident, variant: &VariantData) -> TokenStream {
    let ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => quote! { #enum_ident::#ident },
        Fields::Named(_) => quote! { #enum_ident::#ident { .. } },
        Fields::Unnamed(_) => quote! { #enum_ident::#ident(..) }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;
    use crate::input::parse_input;

    fn parse(input: syn::DeriveInput) -> ErrorInput {
        parse_input(input).expect("parse input")
    }

    #[test]
    fn struct_conversion_attaches_source() {
        let parsed = parse(parse_quote! {
            #[error("boom")]
            #[app_error(kind = AppErrorKind::Internal)]
            struct Boom;
        });
        let impls = expand(&parsed).expect("expand");
        assert_eq!(impls.len(), 1);
        let output = impls[0].to_string();
        assert!(output.contains("with_source"));
        assert!(output.contains("Send"));
        assert!(!output.contains("let _ = value"));
    }

    #[test]
    fn struct_no_source_drops_value() {
        let parsed = parse(parse_quote! {
            #[error("boom")]
            #[app_error(kind = AppErrorKind::Internal, no_source)]
            struct Boom;
        });
        let impls = expand(&parsed).expect("expand");
        assert_eq!(impls.len(), 1);
        let output = impls[0].to_string();
        assert!(!output.contains("with_source"));
        assert!(!output.contains("Send"));
    }

    #[test]
    fn enum_conversion_attaches_source_once() {
        let parsed = parse(parse_quote! {
            enum Failure {
                #[error("a")]
                #[app_error(kind = AppErrorKind::Internal, message)]
                A,
                #[error("b")]
                #[app_error(kind = AppErrorKind::Service)]
                B
            }
        });
        let impls = expand(&parsed).expect("expand");
        assert_eq!(impls.len(), 1);
        let output = impls[0].to_string();
        assert_eq!(output.matches("with_source").count(), 1);
    }

    #[test]
    fn enum_all_no_source_drops_value() {
        let parsed = parse(parse_quote! {
            enum Failure {
                #[error("a")]
                #[app_error(kind = AppErrorKind::Internal, no_source)]
                A,
                #[error("b")]
                #[app_error(kind = AppErrorKind::Service, no_source)]
                B
            }
        });
        let impls = expand(&parsed).expect("expand");
        assert_eq!(impls.len(), 1);
        let output = impls[0].to_string();
        assert!(!output.contains("with_source"));
        assert!(!output.contains("Send"));
    }

    #[test]
    fn enum_mixed_no_source_is_rejected() {
        let parsed = parse(parse_quote! {
            enum Failure {
                #[error("a")]
                #[app_error(kind = AppErrorKind::Internal, no_source)]
                A,
                #[error("b")]
                #[app_error(kind = AppErrorKind::Service)]
                B
            }
        });
        let err = expand(&parsed).expect_err("mixed no_source must fail");
        assert!(err.to_string().contains("no_source"));
    }
}
