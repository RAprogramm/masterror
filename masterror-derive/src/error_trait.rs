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
    let backtrace_method = struct_backtrace_method(&data.fields);
    let has_backtrace = backtrace_method.is_some();
    let backtrace_method = backtrace_method.unwrap_or_default();
    let provide_method = if has_backtrace {
        provide_method_tokens()
    } else {
        TokenStream::new()
    };

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                #body
            }
            #backtrace_method
            #provide_method
        }
    })
}

fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<TokenStream, Error> {
    let mut arms = Vec::new();
    for variant in variants {
        arms.push(variant_source_arm(variant));
    }

    let backtrace_method = enum_backtrace_method(variants);
    let has_backtrace = backtrace_method.is_some();
    let backtrace_method = backtrace_method.unwrap_or_default();
    let provide_method = if has_backtrace {
        provide_method_tokens()
    } else {
        TokenStream::new()
    };

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #(#arms),*
                }
            }
            #backtrace_method
            #provide_method
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
        DisplaySpec::Template(_)
        | DisplaySpec::TemplateWithArgs {
            ..
        }
        | DisplaySpec::FormatterPath {
            ..
        } => {
            if let Some(field) = fields.iter().find(|field| field.attrs.has_source()) {
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
        DisplaySpec::Template(_)
        | DisplaySpec::TemplateWithArgs {
            ..
        }
        | DisplaySpec::FormatterPath {
            ..
        } => variant_template_source(variant)
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
    let source_field = variant.fields.iter().find(|field| field.attrs.has_source());

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

fn struct_backtrace_method(fields: &Fields) -> Option<TokenStream> {
    let field = fields.backtrace_field()?;
    let member = &field.member;
    let body = field_backtrace_expr(quote!(self.#member), quote!(&self.#member), &field.ty);
    Some(quote! {
        #[cfg(error_generic_member_access)]
        fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
            #body
        }
    })
}

fn enum_backtrace_method(variants: &[VariantData]) -> Option<TokenStream> {
    let mut has_backtrace = false;
    let mut arms = Vec::new();
    for variant in variants {
        if variant.fields.backtrace_field().is_some() {
            has_backtrace = true;
        }
        arms.push(variant_backtrace_arm(variant));
    }

    if has_backtrace {
        Some(quote! {
            #[cfg(error_generic_member_access)]
            fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
                match self {
                    #(#arms),*
                }
            }
        })
    } else {
        None
    }
}

fn variant_backtrace_arm(variant: &VariantData) -> TokenStream {
    let variant_ident = &variant.ident;
    let backtrace_field = variant.fields.backtrace_field();

    match (&variant.fields, backtrace_field) {
        (Fields::Unit, _) => quote! { Self::#variant_ident => None },
        (Fields::Named(fields), Some(field)) => {
            let field_ident = field.ident.clone().expect("named field");
            let binding = binding_ident(field);
            let pattern = if fields.len() == 1 {
                quote!(Self::#variant_ident { #field_ident: #binding })
            } else {
                quote!(Self::#variant_ident { #field_ident: #binding, .. })
            };
            let body = field_backtrace_expr(quote!(#binding), quote!(#binding), &field.ty);
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
            let body = field_backtrace_expr(quote!(#binding), quote!(#binding), &field.ty);
            quote! {
                Self::#variant_ident(#(#pattern_elements),*) => { #body }
            }
        }
        (Fields::Named(_), None) => quote! { Self::#variant_ident { .. } => None },
        (Fields::Unnamed(fields), None) => {
            if fields.is_empty() {
                quote! { Self::#variant_ident() => None }
            } else {
                let placeholders = vec![quote!(_); fields.len()];
                quote! { Self::#variant_ident(#(#placeholders),*) => None }
            }
        }
    }
}

fn field_backtrace_expr(
    owned_expr: TokenStream,
    referenced_expr: TokenStream,
    ty: &syn::Type
) -> TokenStream {
    if is_option_type(ty) {
        quote! { #owned_expr.as_ref() }
    } else {
        quote! { Some(#referenced_expr) }
    }
}

fn provide_method_tokens() -> TokenStream {
    quote! {
        #[cfg(error_generic_member_access)]
        fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {
            if let Some(backtrace) = std::error::Error::backtrace(self) {
                request.provide_ref::<std::backtrace::Backtrace>(backtrace);
            }
        }
    }
}

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
