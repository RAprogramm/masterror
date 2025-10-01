// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::input::{
    ErrorData, ErrorInput, Field, Fields, StructData, VariantData, is_option_type
};

pub fn expand(input: &ErrorInput) -> Result<Vec<TokenStream>, Error> {
    let mut impls = Vec::new();

    match &input.data {
        ErrorData::Struct(data) => {
            if let Some(field) = data.fields.first_from_field() {
                impls.push(struct_from_impl(input, data, field)?);
            }
        }
        ErrorData::Enum(variants) => {
            for variant in variants {
                if let Some(field) = variant.fields.first_from_field() {
                    impls.push(enum_from_impl(input, variant, field)?);
                }
            }
        }
    }

    Ok(impls)
}

fn struct_from_impl(
    input: &ErrorInput,
    data: &StructData,
    field: &Field
) -> Result<TokenStream, Error> {
    let ident = &input.ident;
    let ty = &field.ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let constructor = struct_constructor(&data.fields, field)?;

    Ok(quote! {
        impl #impl_generics core::convert::From<#ty> for #ident #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
                #constructor
            }
        }
    })
}

fn enum_from_impl(
    input: &ErrorInput,
    variant: &VariantData,
    field: &Field
) -> Result<TokenStream, Error> {
    let ident = &input.ident;
    let ty = &field.ty;
    let variant_ident = &variant.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let constructor = variant_constructor(variant_ident, &variant.fields, field)?;

    Ok(quote! {
        impl #impl_generics core::convert::From<#ty> for #ident #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
                #constructor
            }
        }
    })
}

fn struct_constructor(fields: &Fields, from_field: &Field) -> Result<TokenStream, Error> {
    match fields {
        Fields::Named(named) => {
            let mut initializers = Vec::new();
            for field in named {
                let field_ident = field.ident.clone().expect("named field");
                let value = field_value_expr(field, from_field)?;
                initializers.push(quote! { #field_ident: #value });
            }
            Ok(quote! { Self { #(#initializers),* } })
        }
        Fields::Unnamed(unnamed) => {
            let mut values = Vec::new();
            for field in unnamed {
                values.push(field_value_expr(field, from_field)?);
            }
            Ok(quote! { Self(#(#values),*) })
        }
        Fields::Unit => Err(Error::new(
            from_field.span,
            "#[from] is not supported on unit structs"
        ))
    }
}

fn variant_constructor(
    variant_ident: &syn::Ident,
    fields: &Fields,
    from_field: &Field
) -> Result<TokenStream, Error> {
    match fields {
        Fields::Named(named) => {
            let mut initializers = Vec::new();
            for field in named {
                let field_ident = field.ident.clone().expect("named field");
                let value = field_value_expr(field, from_field)?;
                initializers.push(quote! { #field_ident: #value });
            }
            Ok(quote! { Self::#variant_ident { #(#initializers),* } })
        }
        Fields::Unnamed(unnamed) => {
            let mut values = Vec::new();
            for field in unnamed {
                values.push(field_value_expr(field, from_field)?);
            }
            Ok(quote! { Self::#variant_ident(#(#values),*) })
        }
        Fields::Unit => Err(Error::new(
            from_field.span,
            "#[from] is not supported on unit variants"
        ))
    }
}

fn field_value_expr(field: &Field, from_field: &Field) -> Result<TokenStream, Error> {
    if field.index == from_field.index {
        return Ok(quote! { value });
    }

    if field.attrs.has_backtrace() {
        return Ok(backtrace_initializer(field));
    }

    if field.attrs.has_source() && field.attrs.from.is_none() {
        return source_initializer(field);
    }

    Err(Error::new(
        field.span,
        "deriving From requires no fields other than source and backtrace"
    ))
}

fn source_initializer(field: &Field) -> Result<TokenStream, Error> {
    if is_option_type(&field.ty) {
        Ok(quote! { ::core::option::Option::None })
    } else {
        Err(Error::new(
            field.span,
            "additional #[source] fields used with #[from] must be Option<_>"
        ))
    }
}

fn backtrace_initializer(field: &Field) -> TokenStream {
    let capture = quote! { ::std::backtrace::Backtrace::capture() };
    if is_option_type(&field.ty) {
        quote! {
            ::core::option::Option::Some(::core::convert::From::from(#capture))
        }
    } else {
        quote! {
            ::core::convert::From::from(#capture)
        }
    }
}
