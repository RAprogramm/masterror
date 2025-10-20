// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Source method implementation generation for Error trait.
//!
//! Handles generation of `std::error::Error::source()` method implementations
//! for both struct and enum error types, including transparent delegation.

use proc_macro2::TokenStream;
use quote::quote;

use super::binding::binding_ident;
use crate::input::{DisplaySpec, Fields, VariantData, is_option_type};

/// Generates source method body for struct error types.
///
/// For transparent display specs, delegates to the first field. For template
/// or formatter specs, returns the field marked with `#[source]` if present.
///
/// # Arguments
///
/// * `fields` - The struct fields
/// * `display` - The display specification
///
/// # Returns
///
/// Token stream for source method body
pub(crate) fn struct_source_body(fields: &Fields, display: &DisplaySpec) -> TokenStream {
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

/// Generates source match arm for enum variant.
///
/// Delegates to transparent or template implementation based on display spec.
///
/// # Arguments
///
/// * `variant` - The enum variant data
///
/// # Returns
///
/// Token stream for variant match arm
pub(crate) fn variant_source_arm(variant: &VariantData) -> TokenStream {
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
            let field_ident = fields[0].ident.clone().expect("named field");
            let pattern = if fields.len() == 1 {
                quote!(Self::#variant_ident { #field_ident })
            } else {
                quote!(Self::#variant_ident { #field_ident, .. })
            };
            quote! {
                #pattern => std::error::Error::source(#field_ident)
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
            let pattern = if field_ident == binding {
                if fields.len() == 1 {
                    quote!(Self::#variant_ident { #field_ident })
                } else {
                    quote!(Self::#variant_ident { #field_ident, .. })
                }
            } else if fields.len() == 1 {
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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use proc_macro2::Span;
    use syn::Member;

    use super::*;
    use crate::{
        input::{DisplaySpec, Field, FieldAttrs},
        template_support::{DisplayTemplate, TemplateSegmentSpec}
    };

    fn make_field(ident: Option<&str>, index: usize, has_source: bool) -> Field {
        let mut attrs = FieldAttrs::default();
        if has_source {
            attrs.source = Some(syn::parse_quote!(#[source]));
        }
        Field {
            ident: ident.map(|s| syn::Ident::new(s, Span::call_site())),
            member: if let Some(s) = ident {
                Member::Named(syn::Ident::new(s, Span::call_site()))
            } else {
                Member::Unnamed(syn::Index {
                    index: index as u32,
                    span:  Span::call_site()
                })
            },
            ty: syn::parse_quote!(Box<dyn std::error::Error>),
            index,
            attrs,
            span: Span::call_site()
        }
    }

    #[test]
    fn test_struct_source_body_transparent_with_field() {
        let field = make_field(Some("inner"), 0, false);
        let fields = Fields::Named(vec![field]);
        let display = DisplaySpec::Transparent {
            attribute: Box::new(syn::parse_quote!(#[error(transparent)]))
        };

        let result = struct_source_body(&fields, &display);
        let output = result.to_string();
        assert!(output.contains("std :: error :: Error :: source"));
        assert!(output.contains("self . inner"));
    }

    #[test]
    fn test_struct_source_body_transparent_no_fields() {
        let fields = Fields::Unit;
        let display = DisplaySpec::Transparent {
            attribute: Box::new(syn::parse_quote!(#[error(transparent)]))
        };

        let result = struct_source_body(&fields, &display);
        assert_eq!(result.to_string(), "None");
    }

    #[test]
    fn test_struct_source_body_template_with_source_field() {
        let field = make_field(Some("cause"), 0, true);
        let fields = Fields::Named(vec![field]);
        let display = DisplaySpec::Template(DisplayTemplate {
            segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
        });

        let result = struct_source_body(&fields, &display);
        let output = result.to_string();
        assert!(output.contains("self . cause"));
        assert!(output.contains("dyn std :: error :: Error"));
    }

    #[test]
    fn test_struct_source_body_template_no_source_field() {
        let field = make_field(Some("value"), 0, false);
        let fields = Fields::Named(vec![field]);
        let display = DisplaySpec::Template(DisplayTemplate {
            segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
        });

        let result = struct_source_body(&fields, &display);
        assert_eq!(result.to_string(), "None");
    }

    #[test]
    fn test_variant_source_arm_unit() {
        let variant = VariantData {
            ident:       syn::Ident::new("Error", Span::call_site()),
            fields:      Fields::Unit,
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = variant_source_arm(&variant);
        assert!(result.to_string().contains("Self :: Error => None"));
    }

    #[test]
    fn test_variant_source_arm_transparent_named() {
        let field = make_field(Some("inner"), 0, false);
        let variant = VariantData {
            ident:       syn::Ident::new("Wrapped", Span::call_site()),
            fields:      Fields::Named(vec![field]),
            display:     DisplaySpec::Transparent {
                attribute: Box::new(syn::parse_quote!(#[error(transparent)]))
            },
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = variant_source_arm(&variant);
        let output = result.to_string();
        assert!(output.contains("Self :: Wrapped"));
        assert!(output.contains("inner"));
    }
}
