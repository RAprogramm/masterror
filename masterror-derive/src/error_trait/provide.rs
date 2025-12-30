// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Provide method implementation generation for Error trait.
//!
//! Generates `provide()` method implementations that allow error types to
//! expose additional context through the Error trait's provide API.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::TypePath;

use super::binding::binding_ident;
use crate::input::{
    BacktraceField, BacktraceFieldKind, Field, Fields, ProvideSpec, VariantData, is_option_type
};

/// Generates provide method for struct error types.
///
/// Returns None if no fields require provide support. Otherwise generates
/// method that provides source, backtrace, and custom types as appropriate.
///
/// # Arguments
///
/// * `fields` - The struct fields
///
/// # Returns
///
/// Optional token stream for provide method
pub(crate) fn struct_provide_method(fields: &Fields) -> Option<TokenStream> {
    let backtrace = fields.backtrace_field();
    let source_field = fields.iter().find(|candidate| candidate.attrs.has_source());
    let request = quote!(request);
    let delegates_to_source = backtrace.is_some_and(|backtrace| {
        matches!(backtrace.kind(), BacktraceFieldKind::Explicit) && !backtrace.stores_backtrace()
    });
    let mut statements = Vec::new();
    let mut needs_trait_import = false;
    if let Some(source_field) = source_field {
        needs_trait_import = true;
        let member = &source_field.member;
        statements.push(provide_source_tokens(
            quote!(self.#member),
            source_field,
            &request
        ));
    }
    if let Some(backtrace) = backtrace
        && backtrace.stores_backtrace()
        && source_field.is_none_or(|source| source.index != backtrace.index())
        && !delegates_to_source
    {
        let member = &backtrace.field().member;
        statements.push(provide_backtrace_tokens(
            quote!(self.#member),
            backtrace.field(),
            &request
        ));
    }
    for field in fields.iter() {
        if field.attrs.provides.is_empty() {
            continue;
        }
        let member = &field.member;
        let expr = quote!(self.#member);
        for spec in &field.attrs.provides {
            statements.extend(provide_custom_tokens(expr.clone(), field, spec, &request));
        }
    }
    if statements.is_empty() {
        return None;
    }
    let trait_import = if needs_trait_import {
        quote! { use masterror::provide::ThiserrorProvide as _; }
    } else {
        TokenStream::new()
    };
    Some(quote! {
        #[cfg(masterror_has_error_generic_member_access)]
        fn provide<'a>(&'a self, #request: &mut core::error::Request<'a>) {
            #trait_import
            #(#statements)*
        }
    })
}

/// Generates provide method for enum error types.
///
/// Returns None if no variants require provide support. Otherwise generates
/// match expression delegating to variant-specific logic.
///
/// # Arguments
///
/// * `variants` - The enum variants
///
/// # Returns
///
/// Optional token stream for provide method
pub(crate) fn enum_provide_method(variants: &[VariantData]) -> Option<TokenStream> {
    let mut has_backtrace = false;
    let mut has_custom_provides = false;
    let mut needs_trait_import = false;
    let mut arms = Vec::new();
    let request = quote!(request);
    for variant in variants {
        if variant.fields.backtrace_field().is_some() {
            has_backtrace = true;
        }
        if variant
            .fields
            .iter()
            .any(|field| !field.attrs.provides.is_empty())
        {
            has_custom_provides = true;
        }
        arms.push(variant_provide_arm_tokens(
            variant,
            &request,
            &mut needs_trait_import
        ));
    }
    if !has_backtrace && !has_custom_provides {
        return None;
    }
    let trait_import = if needs_trait_import {
        quote! { use masterror::provide::ThiserrorProvide as _; }
    } else {
        TokenStream::new()
    };
    Some(quote! {
        #[cfg(masterror_has_error_generic_member_access)]
        fn provide<'a>(&'a self, #request: &mut core::error::Request<'a>) {
            #trait_import
            #[allow(deprecated)]
            match self {
                #(#arms),*
            }
        }
    })
}

pub(crate) fn variant_provide_arm_tokens(
    variant: &VariantData,
    request: &TokenStream,
    needs_trait_import: &mut bool
) -> TokenStream {
    let variant_ident = &variant.ident;
    let backtrace = variant.fields.backtrace_field();
    let source_field = variant.fields.iter().find(|field| field.attrs.has_source());
    match &variant.fields {
        Fields::Unit => quote! { Self::#variant_ident => {} },
        Fields::Named(fields) => variant_provide_named_arm(
            variant_ident,
            fields,
            backtrace,
            source_field,
            request,
            needs_trait_import
        ),
        Fields::Unnamed(fields) => variant_provide_unnamed_arm(
            variant_ident,
            fields,
            backtrace,
            source_field,
            request,
            needs_trait_import
        )
    }
}

pub(crate) fn variant_provide_named_arm(
    variant_ident: &Ident,
    fields: &[Field],
    backtrace: Option<BacktraceField<'_>>,
    source: Option<&Field>,
    request: &TokenStream,
    needs_trait_import: &mut bool
) -> TokenStream {
    let same_as_source = if let (Some(backtrace_field), Some(source_field)) = (backtrace, source) {
        source_field.index == backtrace_field.index()
    } else {
        false
    };
    let delegates_to_source = backtrace.is_some_and(|field| {
        matches!(field.kind(), BacktraceFieldKind::Explicit) && !field.stores_backtrace()
    });
    let mut entries = Vec::new();
    let mut backtrace_binding = None;
    let mut source_binding = None;
    let mut provide_bindings: Vec<(Ident, &Field)> = Vec::new();
    for field in fields {
        let ident = field.ident.clone().expect("named field");
        let needs_binding = backtrace.is_some_and(|candidate| candidate.index() == field.index)
            || source.is_some_and(|candidate| candidate.index == field.index)
            || !field.attrs.provides.is_empty();
        if needs_binding {
            let binding = binding_ident(field);
            let pattern_binding = binding.clone();
            if ident == pattern_binding {
                entries.push(quote!(#ident));
            } else {
                entries.push(quote!(#ident: #pattern_binding));
            }
            if backtrace.is_some_and(|candidate| candidate.index() == field.index) {
                backtrace_binding = Some(binding.clone());
            }
            if source.is_some_and(|candidate| candidate.index == field.index) {
                source_binding = Some(binding.clone());
            }
            if !field.attrs.provides.is_empty() {
                provide_bindings.push((binding, field));
            }
        } else {
            entries.push(quote!(#ident: _));
        }
    }
    let mut statements = Vec::new();
    if let Some(source_field) = source {
        *needs_trait_import = true;
        let binding = source_binding.expect("source binding");
        statements.push(provide_source_tokens(
            quote!(#binding),
            source_field,
            request
        ));
    }
    if let Some(backtrace_field) = backtrace
        && backtrace_field.stores_backtrace()
        && !same_as_source
        && !delegates_to_source
    {
        let binding = backtrace_binding.expect("backtrace binding");
        statements.push(provide_backtrace_tokens(
            quote!(#binding),
            backtrace_field.field(),
            request
        ));
    }
    for (binding, field) in provide_bindings {
        let binding_expr = quote!(#binding);
        for spec in &field.attrs.provides {
            statements.extend(provide_custom_tokens(
                binding_expr.clone(),
                field,
                spec,
                request
            ));
        }
    }
    let pattern = quote!(Self::#variant_ident { #(#entries),* });
    if statements.is_empty() {
        quote! { #pattern => {} }
    } else {
        quote! { #pattern => { #(#statements)* } }
    }
}

pub(crate) fn variant_provide_unnamed_arm(
    variant_ident: &Ident,
    fields: &[Field],
    backtrace: Option<BacktraceField<'_>>,
    source: Option<&Field>,
    request: &TokenStream,
    needs_trait_import: &mut bool
) -> TokenStream {
    let same_as_source = if let (Some(backtrace_field), Some(source_field)) = (backtrace, source) {
        source_field.index == backtrace_field.index()
    } else {
        false
    };
    let delegates_to_source = backtrace.is_some_and(|field| {
        matches!(field.kind(), BacktraceFieldKind::Explicit) && !field.stores_backtrace()
    });
    let mut elements = Vec::new();
    let mut backtrace_binding = None;
    let mut source_binding = None;
    let mut provide_bindings: Vec<(Ident, &Field)> = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let needs_binding = backtrace.is_some_and(|candidate| candidate.index() == index)
            || source.is_some_and(|candidate| candidate.index == index)
            || !field.attrs.provides.is_empty();
        if needs_binding {
            let binding = binding_ident(field);
            let pattern_binding = binding.clone();
            elements.push(quote!(#pattern_binding));
            if backtrace.is_some_and(|candidate| candidate.index() == index) {
                backtrace_binding = Some(binding.clone());
            }
            if source.is_some_and(|candidate| candidate.index == index) {
                source_binding = Some(binding.clone());
            }
            if !field.attrs.provides.is_empty() {
                provide_bindings.push((binding, field));
            }
        } else {
            elements.push(quote!(_));
        }
    }
    let mut statements = Vec::new();
    if let Some(source_field) = source {
        *needs_trait_import = true;
        let binding = source_binding.expect("source binding");
        statements.push(provide_source_tokens(
            quote!(#binding),
            source_field,
            request
        ));
    }
    if let Some(backtrace_field) = backtrace
        && backtrace_field.stores_backtrace()
        && !same_as_source
        && !delegates_to_source
    {
        let binding = backtrace_binding.expect("backtrace binding");
        statements.push(provide_backtrace_tokens(
            quote!(#binding),
            backtrace_field.field(),
            request
        ));
    }
    for (binding, field) in provide_bindings {
        let binding_expr = quote!(#binding);
        for spec in &field.attrs.provides {
            statements.extend(provide_custom_tokens(
                binding_expr.clone(),
                field,
                spec,
                request
            ));
        }
    }
    let pattern = if elements.is_empty() {
        quote!(Self::#variant_ident())
    } else {
        quote!(Self::#variant_ident(#(#elements),*))
    };
    if statements.is_empty() {
        quote! { #pattern => {} }
    } else {
        quote! { #pattern => { #(#statements)* } }
    }
}

pub(crate) fn provide_custom_tokens(
    expr: TokenStream,
    field: &Field,
    spec: &ProvideSpec,
    request: &TokenStream
) -> Vec<TokenStream> {
    let mut tokens = Vec::new();
    if let Some(reference) = &spec.reference {
        tokens.push(provide_custom_ref_tokens(
            expr.clone(),
            field,
            reference,
            request
        ));
    }
    if let Some(value) = &spec.value {
        tokens.push(provide_custom_value_tokens(
            expr.clone(),
            field,
            value,
            request
        ));
    }
    tokens
}

fn provide_custom_ref_tokens(
    expr: TokenStream,
    field: &Field,
    ty: &TypePath,
    request: &TokenStream
) -> TokenStream {
    if is_option_type(&field.ty) {
        quote! {
            if let Some(value) = #expr.as_ref() {
                #request.provide_ref::<#ty>(value);
            }
        }
    } else {
        quote! {
            #request.provide_ref::<#ty>(#expr);
        }
    }
}

fn provide_custom_value_tokens(
    expr: TokenStream,
    field: &Field,
    ty: &TypePath,
    request: &TokenStream
) -> TokenStream {
    if is_option_type(&field.ty) {
        quote! {
            if let Some(value) = #expr.clone() {
                #request.provide_value::<#ty>(value);
            }
        }
    } else {
        quote! {
            #request.provide_value::<#ty>(#expr.clone());
        }
    }
}

pub(crate) fn provide_backtrace_tokens(
    expr: TokenStream,
    field: &Field,
    request: &TokenStream
) -> TokenStream {
    if is_option_type(&field.ty) {
        quote! {
            if let Some(backtrace) = #expr.as_ref() {
                #request.provide_ref::<std::backtrace::Backtrace>(backtrace);
            }
        }
    } else {
        quote! {
            #request.provide_ref::<std::backtrace::Backtrace>(#expr);
        }
    }
}

pub(crate) fn provide_source_tokens(
    expr: TokenStream,
    field: &Field,
    request: &TokenStream
) -> TokenStream {
    if is_option_type(&field.ty) {
        quote! {
            if let Some(source) = #expr.as_ref() {
                source.thiserror_provide(#request);
            }
        }
    } else {
        quote! {
            #expr.thiserror_provide(#request);
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use proc_macro2::Span;
    use syn::Member;

    use super::*;
    use crate::{
        input::FieldAttrs,
        template_support::{DisplayTemplate, TemplateSegmentSpec}
    };

    fn make_field(ident: Option<&str>, index: usize) -> Field {
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
            ty: syn::parse_quote!(String),
            index,
            attrs: FieldAttrs::default(),
            span: Span::call_site()
        }
    }

    #[test]
    fn test_struct_provide_method_no_fields() {
        let fields = Fields::Unit;
        let result = struct_provide_method(&fields);
        assert!(result.is_none());
    }

    #[test]
    fn test_enum_provide_method_no_provides() {
        use crate::input::{DisplaySpec, VariantData};
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
        let result = enum_provide_method(&[variant]);
        assert!(result.is_none());
    }

    #[test]
    fn test_provide_source_tokens_option_type() {
        let field = Field {
            ty: syn::parse_quote!(Option<Box<dyn std::error::Error>>),
            ..make_field(Some("source"), 0)
        };
        let request = quote!(req);
        let expr = quote!(self.source);
        let result = provide_source_tokens(expr, &field, &request);
        let output = result.to_string();
        assert!(output.contains("if let Some"));
        assert!(output.contains("thiserror_provide"));
    }

    #[test]
    fn test_provide_source_tokens_non_option() {
        let field = Field {
            ty: syn::parse_quote!(Box<dyn std::error::Error>),
            ..make_field(Some("source"), 0)
        };
        let request = quote!(req);
        let expr = quote!(self.source);
        let result = provide_source_tokens(expr, &field, &request);
        let output = result.to_string();
        assert!(output.contains("thiserror_provide"));
        assert!(!output.contains("if let"));
    }

    #[test]
    fn test_provide_backtrace_tokens_option() {
        let field = Field {
            ty: syn::parse_quote!(Option<std::backtrace::Backtrace>),
            ..make_field(Some("bt"), 0)
        };
        let request = quote!(req);
        let expr = quote!(self.bt);
        let result = provide_backtrace_tokens(expr, &field, &request);
        let output = result.to_string();
        assert!(output.contains("if let Some"));
        assert!(output.contains("provide_ref"));
    }

    #[test]
    fn test_provide_backtrace_tokens_non_option() {
        let field = Field {
            ty: syn::parse_quote!(std::backtrace::Backtrace),
            ..make_field(Some("bt"), 0)
        };
        let request = quote!(req);
        let expr = quote!(self.bt);
        let result = provide_backtrace_tokens(expr, &field, &request);
        let output = result.to_string();
        assert!(output.contains("provide_ref"));
        assert!(!output.contains("if let"));
    }

    #[test]
    fn test_provide_custom_tokens_reference() {
        use crate::input::ProvideSpec;
        let field = make_field(Some("trace_id"), 0);
        let spec = ProvideSpec {
            reference: Some(syn::parse_quote!(TraceId)),
            value:     None
        };
        let request = quote!(req);
        let expr = quote!(self.trace_id);
        let result = provide_custom_tokens(expr, &field, &spec, &request);
        assert_eq!(result.len(), 1);
        let output = result[0].to_string();
        assert!(output.contains("provide_ref"));
        assert!(output.contains("TraceId"));
    }

    #[test]
    fn test_provide_custom_tokens_value() {
        use crate::input::ProvideSpec;
        let field = make_field(Some("span_id"), 0);
        let spec = ProvideSpec {
            reference: None,
            value:     Some(syn::parse_quote!(SpanId))
        };
        let request = quote!(req);
        let expr = quote!(self.span_id);
        let result = provide_custom_tokens(expr, &field, &spec, &request);
        assert_eq!(result.len(), 1);
        let output = result[0].to_string();
        assert!(output.contains("provide_value"));
        assert!(output.contains("SpanId"));
    }

    #[test]
    fn test_provide_custom_tokens_both() {
        use crate::input::ProvideSpec;
        let field = make_field(Some("data"), 0);
        let spec = ProvideSpec {
            reference: Some(syn::parse_quote!(DataRef)),
            value:     Some(syn::parse_quote!(DataVal))
        };
        let request = quote!(req);
        let expr = quote!(self.data);
        let result = provide_custom_tokens(expr, &field, &spec, &request);
        assert_eq!(result.len(), 2);
        let output0 = result[0].to_string();
        let output1 = result[1].to_string();
        assert!(output0.contains("provide_ref") || output1.contains("provide_ref"));
        assert!(output0.contains("provide_value") || output1.contains("provide_value"));
    }

    #[test]
    fn test_provide_custom_tokens_option_type() {
        use crate::input::ProvideSpec;
        let field = Field {
            ty: syn::parse_quote!(Option<String>),
            ..make_field(Some("data"), 0)
        };
        let spec = ProvideSpec {
            reference: Some(syn::parse_quote!(Ref)),
            value:     None
        };
        let request = quote!(req);
        let expr = quote!(self.data);
        let result = provide_custom_tokens(expr, &field, &spec, &request);
        assert_eq!(result.len(), 1);
        let output = result[0].to_string();
        assert!(output.contains("if let Some"));
        assert!(output.contains("provide_ref"));
    }
}
