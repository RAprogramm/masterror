use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

use crate::input::{
    DisplaySpec, ErrorData, ErrorInput, Field, Fields, StructData, VariantData, is_option_type
};

pub fn expand(input: &ErrorInput) -> Result<TokenStream, Error> {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}

fn expand_struct(input: &ErrorInput, data: &StructData) -> Result<TokenStream, Error> {
    let body = struct_source_body(&data.fields, &data.display);

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                #body
            }
        }
    })
}

fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<TokenStream, Error> {
    let mut arms = Vec::new();
    for variant in variants {
        arms.push(variant_source_arm(variant));
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}

fn struct_source_body(fields: &Fields, display: &DisplaySpec) -> TokenStream {
    match display {
        DisplaySpec::Transparent {
            ..
        } => {
            if let Some(field) = fields.iter().next() {
                let member = &field.member;
                quote! { std::error::Error::source(&self.#member) }
            } else {
                quote! { None }
            }
        }
        DisplaySpec::Template(_) => {
            if let Some(field) = fields.iter().find(|field| field.attrs.source.is_some()) {
                let member = &field.member;
                field_source_expr(quote!(self.#member), quote!(&self.#member), &field.ty)
            } else {
                quote! { None }
            }
        }
    }
}

fn variant_source_arm(variant: &VariantData) -> TokenStream {
    match &variant.display {
        DisplaySpec::Transparent {
            ..
        } => variant_transparent_source(variant),
        DisplaySpec::Template(_) => variant_template_source(variant)
    }
}

fn variant_transparent_source(variant: &VariantData) -> TokenStream {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => quote! { Self::#variant_ident => None },
        Fields::Named(fields) => {
            let binding = fields[0].ident.clone().expect("named field");
            let pattern = if fields.len() == 1 {
                quote!(Self::#variant_ident { #binding })
            } else {
                quote!(Self::#variant_ident { #binding, .. })
            };
            quote! {
                #pattern => std::error::Error::source(#binding)
            }
        }
        Fields::Unnamed(fields) => {
            let binding = binding_ident(&fields[0]);
            let mut patterns = Vec::new();
            for (index, _) in fields.iter().enumerate() {
                if index == 0 {
                    patterns.push(quote!(#binding));
                } else {
                    patterns.push(quote!(_));
                }
            }
            quote! {
                Self::#variant_ident(#(#patterns),*) => std::error::Error::source(#binding)
            }
        }
    }
}

fn variant_template_source(variant: &VariantData) -> TokenStream {
    let variant_ident = &variant.ident;
    let source_field = variant
        .fields
        .iter()
        .find(|field| field.attrs.source.is_some());

    match (&variant.fields, source_field) {
        (Fields::Unit, _) => quote! { Self::#variant_ident => None },
        (_, None) => match &variant.fields {
            Fields::Named(_) => quote! { Self::#variant_ident { .. } => None },
            Fields::Unnamed(fields) if fields.is_empty() => {
                quote! { Self::#variant_ident() => None }
            }
            Fields::Unnamed(fields) => {
                let placeholders = vec![quote!(_); fields.len()];
                quote! { Self::#variant_ident(#(#placeholders),*) => None }
            }
            Fields::Unit => quote! { Self::#variant_ident => None }
        },
        (Fields::Named(fields), Some(field)) => {
            let field_ident = field.ident.clone().expect("named field");
            let binding = binding_ident(field);
            let pattern = if fields.len() == 1 {
                quote!(Self::#variant_ident { #field_ident: #binding })
            } else {
                quote!(Self::#variant_ident { #field_ident: #binding, .. })
            };
            let body = field_source_expr(quote!(#binding), quote!(#binding), &field.ty);
            quote! {
                #pattern => { #body }
            }
        }
        (Fields::Unnamed(fields), Some(field)) => {
            let index = field.index;
            let binding = binding_ident(field);
            let pattern_elements: Vec<_> = fields
                .iter()
                .enumerate()
                .map(|(idx, _)| {
                    if idx == index {
                        quote!(#binding)
                    } else {
                        quote!(_)
                    }
                })
                .collect();
            let body = field_source_expr(quote!(#binding), quote!(#binding), &field.ty);
            quote! {
                Self::#variant_ident(#(#pattern_elements),*) => { #body }
            }
        }
    }
}

fn field_source_expr(
    owned_expr: TokenStream,
    referenced_expr: TokenStream,
    ty: &syn::Type
) -> TokenStream {
    if is_option_type(ty) {
        quote! { #owned_expr.as_ref().map(|source| source as &(dyn std::error::Error + 'static)) }
    } else {
        quote! { Some(#referenced_expr as &(dyn std::error::Error + 'static)) }
    }
}

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
