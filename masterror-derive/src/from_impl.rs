use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::input::{ErrorData, ErrorInput, Field, Fields, StructData, VariantData};

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

    let constructor = match &data.fields {
        Fields::Named(_) => {
            let field_ident = field.ident.clone().expect("named field");
            quote! { Self { #field_ident: value } }
        }
        Fields::Unnamed(_) => quote! { Self(value) },
        Fields::Unit => {
            return Err(Error::new(
                field.span,
                "#[from] is not supported on unit structs"
            ));
        }
    };

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

    let constructor = match &variant.fields {
        Fields::Named(_) => {
            let field_ident = field.ident.clone().expect("named field");
            quote! { Self::#variant_ident { #field_ident: value } }
        }
        Fields::Unnamed(_) => quote! { Self::#variant_ident(value) },
        Fields::Unit => {
            return Err(Error::new(
                field.span,
                "#[from] is not supported on unit variants"
            ));
        }
    };

    Ok(quote! {
        impl #impl_generics core::convert::From<#ty> for #ident #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
                #constructor
            }
        }
    })
}
