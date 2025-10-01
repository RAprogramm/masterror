// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

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
        impls.push(enum_app_error_impl(input, variants));
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

fn struct_app_error_impl(input: &ErrorInput, spec: &AppErrorSpec) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let kind = &spec.kind;

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

    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::AppError #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #body
            }
        }
    }
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

fn enum_app_error_impl(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut arms = Vec::new();
    for variant in variants {
        let spec = variant.app_error.as_ref().expect("presence checked");
        let kind = &spec.kind;
        let pattern = variant_app_error_pattern(ident, variant);
        let body = if spec.expose_message {
            quote! {
                masterror::AppError::with(#kind, std::string::ToString::to_string(&err))
            }
        } else {
            quote! {
                {
                    let _ = err;
                    masterror::AppError::bare(#kind)
                }
            }
        };
        arms.push(quote! { #pattern => #body });
    }

    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::AppError #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                match value {
                    #(#arms),*
                }
            }
        }
    }
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
        Fields::Unit => quote! { err @ #enum_ident::#ident },
        Fields::Named(_) => quote! { err @ #enum_ident::#ident { .. } },
        Fields::Unnamed(_) => quote! { err @ #enum_ident::#ident(..) }
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
