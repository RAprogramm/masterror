// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Backtrace method implementation generation for Error trait.
//!
//! Generates `backtrace()` method implementations for error types that
//! capture or delegate backtrace information.

use proc_macro2::TokenStream;
use quote::quote;

use super::binding::binding_ident;
use crate::input::{Field, Fields, VariantData, is_backtrace_storage, is_option_type};

/// Generates backtrace method for struct error types.
///
/// Returns None if no backtrace field exists. Otherwise generates method
/// that returns backtrace from the designated field.
///
/// # Arguments
///
/// * `fields` - The struct fields
///
/// # Returns
///
/// Optional token stream for backtrace method
pub(crate) fn struct_backtrace_method(fields: &Fields) -> Option<TokenStream> {
    let backtrace = fields.backtrace_field()?;
    let field = backtrace.field();
    let member = &field.member;
    let body = field_backtrace_expr(quote!(self.#member), quote!(&self.#member), field);
    Some(quote! {
        #[cfg(masterror_has_error_generic_member_access)]
        fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
            #body
        }
    })
}

/// Generates backtrace method for enum error types.
///
/// Returns None if no variants have backtrace fields. Otherwise generates
/// match expression delegating to variant-specific logic.
///
/// # Arguments
///
/// * `variants` - The enum variants
///
/// # Returns
///
/// Optional token stream for backtrace method
pub(crate) fn enum_backtrace_method(variants: &[VariantData]) -> Option<TokenStream> {
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
            #[cfg(masterror_has_error_generic_member_access)]
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

/// Generates backtrace match arm for enum variant.
///
/// Returns None for variants without backtrace, or delegates to field
/// backtrace expression for variants with backtrace fields.
///
/// # Arguments
///
/// * `variant` - The enum variant data
///
/// # Returns
///
/// Token stream for variant match arm
pub(crate) fn variant_backtrace_arm(variant: &VariantData) -> TokenStream {
    let variant_ident = &variant.ident;
    let backtrace_field = variant.fields.backtrace_field();

    match (&variant.fields, backtrace_field) {
        (Fields::Unit, _) => quote! { Self::#variant_ident => None },
        (Fields::Named(fields), Some(backtrace)) => {
            let field = backtrace.field();
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
            let body = field_backtrace_expr(quote!(#binding), quote!(#binding), field);
            quote! {
                #pattern => { #body }
            }
        }
        (Fields::Unnamed(fields), Some(backtrace)) => {
            let field = backtrace.field();
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
            let body = field_backtrace_expr(quote!(#binding), quote!(#binding), field);
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
    field: &Field
) -> TokenStream {
    let ty = &field.ty;
    if is_backtrace_storage(ty) {
        if is_option_type(ty) {
            quote! { #owned_expr.as_ref() }
        } else {
            quote! { Some(#referenced_expr) }
        }
    } else if field.attrs.has_source() {
        if is_option_type(ty) {
            quote! { #owned_expr.as_ref().and_then(std::error::Error::backtrace) }
        } else {
            quote! { std::error::Error::backtrace(#referenced_expr) }
        }
    } else {
        quote! { None }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::Member;

    use super::*;
    use crate::{
        input::{DisplaySpec, FieldAttrs},
        template_support::{DisplayTemplate, TemplateSegmentSpec}
    };

    fn make_field_with_backtrace(ident: Option<&str>, index: usize) -> Field {
        let mut attrs = FieldAttrs::default();
        attrs.backtrace = Some(syn::parse_quote!(#[backtrace]));
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
            ty: syn::parse_quote!(std::backtrace::Backtrace),
            index,
            attrs,
            span: Span::call_site()
        }
    }

    #[test]
    fn test_struct_backtrace_method_with_backtrace_field() {
        let field = make_field_with_backtrace(Some("bt"), 0);
        let fields = Fields::Named(vec![field]);

        let result = struct_backtrace_method(&fields);
        assert!(result.is_some());
        let output = result.expect("backtrace method").to_string();
        assert!(output.contains("fn backtrace"));
        assert!(output.contains("self . bt"));
    }

    #[test]
    fn test_struct_backtrace_method_without_backtrace_field() {
        let fields = Fields::Unit;
        let result = struct_backtrace_method(&fields);
        assert!(result.is_none());
    }

    #[test]
    fn test_enum_backtrace_method_with_backtrace_variants() {
        let field = make_field_with_backtrace(Some("bt"), 0);
        let variant = VariantData {
            ident:       syn::Ident::new("Error", Span::call_site()),
            fields:      Fields::Named(vec![field]),
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = enum_backtrace_method(&[variant]);
        assert!(result.is_some());
        let output = result.expect("backtrace method").to_string();
        assert!(output.contains("fn backtrace"));
        assert!(output.contains("match self"));
    }

    #[test]
    fn test_enum_backtrace_method_without_backtrace() {
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

        let result = enum_backtrace_method(&[variant]);
        assert!(result.is_none());
    }

    #[test]
    fn test_variant_backtrace_arm_unit_variant() {
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

        let result = variant_backtrace_arm(&variant);
        assert!(result.to_string().contains("Self :: Error => None"));
    }

    #[test]
    fn test_variant_backtrace_arm_with_backtrace_field() {
        let field = make_field_with_backtrace(Some("bt"), 0);
        let variant = VariantData {
            ident:       syn::Ident::new("WithBacktrace", Span::call_site()),
            fields:      Fields::Named(vec![field]),
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = variant_backtrace_arm(&variant);
        let output = result.to_string();
        assert!(output.contains("Self :: WithBacktrace"));
        assert!(output.contains("bt"));
    }

    #[test]
    fn test_variant_backtrace_arm_unnamed_with_backtrace() {
        let field = make_field_with_backtrace(None, 0);
        let variant = VariantData {
            ident:       syn::Ident::new("Error", Span::call_site()),
            fields:      Fields::Unnamed(vec![field]),
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = variant_backtrace_arm(&variant);
        let output = result.to_string();
        assert!(output.contains("Self :: Error"));
        assert!(output.contains("__field0"));
    }

    #[test]
    fn test_variant_backtrace_arm_named_no_backtrace() {
        use proc_macro2::Span;
        use syn::parse_quote;

        use crate::input::{Field, FieldAttrs};

        let field = Field {
            ident:  Some(syn::Ident::new("value", Span::call_site())),
            member: syn::Member::Named(syn::Ident::new("value", Span::call_site())),
            ty:     parse_quote!(String),
            index:  0,
            attrs:  FieldAttrs::default(),
            span:   Span::call_site()
        };
        let variant = VariantData {
            ident:       syn::Ident::new("Error", Span::call_site()),
            fields:      Fields::Named(vec![field]),
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = variant_backtrace_arm(&variant);
        let output = result.to_string();
        assert!(output.contains("Self :: Error"));
        assert!(output.contains(".."));
        assert!(output.contains("None"));
    }

    #[test]
    fn test_variant_backtrace_arm_unnamed_no_backtrace() {
        use proc_macro2::Span;
        use syn::parse_quote;

        use crate::input::{Field, FieldAttrs};

        let field = Field {
            ident:  None,
            member: syn::Member::Unnamed(syn::Index {
                index: 0,
                span:  Span::call_site()
            }),
            ty:     parse_quote!(String),
            index:  0,
            attrs:  FieldAttrs::default(),
            span:   Span::call_site()
        };
        let variant = VariantData {
            ident:       syn::Ident::new("Error", Span::call_site()),
            fields:      Fields::Unnamed(vec![field]),
            display:     DisplaySpec::Template(DisplayTemplate {
                segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
            }),
            format_args: Default::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };

        let result = variant_backtrace_arm(&variant);
        let output = result.to_string();
        assert!(output.contains("Self :: Error"));
        assert!(output.contains("_"));
        assert!(output.contains("None"));
    }
}
