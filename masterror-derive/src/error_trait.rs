use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, TypePath};

use crate::input::{
    BacktraceField, BacktraceFieldKind, DisplaySpec, ErrorData, ErrorInput, Field, Fields,
    ProvideSpec, StructData, VariantData, is_backtrace_storage, is_option_type
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
    let provide_method = struct_provide_method(&data.fields);
    let backtrace_method = backtrace_method.unwrap_or_default();
    let provide_method = provide_method.unwrap_or_default();

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
    let provide_method = enum_provide_method(variants);
    let backtrace_method = backtrace_method.unwrap_or_default();
    let provide_method = provide_method.unwrap_or_default();

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

fn variant_backtrace_arm(variant: &VariantData) -> TokenStream {
    let variant_ident = &variant.ident;
    let backtrace_field = variant.fields.backtrace_field();

    match (&variant.fields, backtrace_field) {
        (Fields::Unit, _) => quote! { Self::#variant_ident => None },
        (Fields::Named(fields), Some(backtrace)) => {
            let field = backtrace.field();
            let field_ident = field.ident.clone().expect("named field");
            let binding = binding_ident(field);
            let pattern = if fields.len() == 1 {
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

fn struct_provide_method(fields: &Fields) -> Option<TokenStream> {
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

fn enum_provide_method(variants: &[VariantData]) -> Option<TokenStream> {
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

fn variant_provide_arm_tokens(
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

fn variant_provide_named_arm(
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
            entries.push(quote!(#ident: #pattern_binding));

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

fn variant_provide_unnamed_arm(
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

fn provide_custom_tokens(
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

fn provide_backtrace_tokens(
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

fn provide_source_tokens(expr: TokenStream, field: &Field, request: &TokenStream) -> TokenStream {
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

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
