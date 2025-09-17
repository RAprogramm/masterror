#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all, rust_2018_idioms)]

//! Derive macro implementing [`std::error::Error`] with `Display` formatting.
//!
//! The macro mirrors the essential functionality relied upon by `masterror` and
//! consumers of the crate: display strings with named or positional fields,
//! `#[from]` conversions for wrapper types, and a configurable error source via
//! `#[source]` field attributes.

use std::collections::BTreeSet;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, GenericArgument, Generics,
    LitStr, Member, Meta, PathArguments, Type, spanned::Spanned
};

/// Derive [`std::error::Error`] and [`core::fmt::Display`] for structs and
/// enums.
///
/// ```
/// use masterror_derive::Error;
///
/// #[derive(Debug, Error)]
/// #[error("{code}: {message}")]
/// struct MiniError {
///     code:    u16,
///     message: &'static str
/// }
///
/// let err = MiniError {
///     code:    500,
///     message: "boom"
/// };
/// assert_eq!(err.to_string(), "500: boom");
/// assert!(std::error::Error::source(&err).is_none());
/// ```
#[proc_macro_derive(Error, attributes(error, source, from))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    match derive_error_impl(syn::parse_macro_input!(input as DeriveInput)) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into()
    }
}

fn derive_error_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let ident = input.ident;
    let generics = input.generics;

    let display_impl;
    let error_impl;
    let from_impls;

    match input.data {
        Data::Struct(data) => {
            let fields = parse_fields(&data)?;
            let display_attr = parse_display_attr(&input.attrs)?;
            display_impl = build_struct_display(&ident, &generics, &fields, &display_attr)?;
            error_impl = build_struct_error(&ident, &generics, &fields)?;
            from_impls = build_struct_from_impl(&ident, &generics, &fields)?;
        }
        Data::Enum(data) => {
            let variants = parse_enum(&data)?;
            display_impl = build_enum_display(&ident, &generics, &variants)?;
            error_impl = build_enum_error(&ident, &generics, &variants)?;
            from_impls = build_enum_from_impls(&ident, &generics, &variants)?;
        }
        Data::Union(_) => {
            return Err(syn::Error::new(
                ident.span(),
                "#[derive(Error)] does not support unions"
            ));
        }
    }

    Ok(quote! {
        #display_impl
        #error_impl
        #from_impls
    })
}

#[derive(Clone, Copy)]
enum FieldsStyle {
    Unit,
    Named,
    Unnamed
}

#[derive(Clone)]
struct FieldSpec {
    member:  Member,
    ident:   Option<Ident>,
    binding: Ident,
    ty:      Type,
    attrs:   FieldAttributes
}

#[derive(Clone, Default)]
struct FieldAttributes {
    from:   Option<Span>,
    source: Option<Span>
}

impl FieldAttributes {
    fn mark_from(&mut self, span: Span) -> syn::Result<()> {
        if self.from.is_some() {
            return Err(syn::Error::new(span, "duplicate #[from] attribute"));
        }
        self.from = Some(span);
        Ok(())
    }

    fn mark_source(&mut self, span: Span) -> syn::Result<()> {
        if self.source.is_some() {
            return Err(syn::Error::new(span, "duplicate #[source] attribute"));
        }
        self.source = Some(span);
        Ok(())
    }

    fn has_source(&self) -> bool {
        self.source.is_some()
    }

    fn span_of_from_attribute(&self) -> Option<Span> {
        self.from
    }

    fn source_span(&self) -> Option<Span> {
        self.source
    }
}

#[derive(Clone, Copy)]
enum SourceKind {
    Direct { needs_deref: bool },
    Optional { needs_deref: bool }
}

#[derive(Clone, Copy)]
struct SourceField {
    index: usize,
    kind:  SourceKind
}

struct ParsedFields {
    style:  FieldsStyle,
    fields: Vec<FieldSpec>,
    source: Option<SourceField>
}

struct VariantInfo {
    ident:   Ident,
    fields:  ParsedFields,
    display: LitStr
}

struct FromFieldInfo<'a> {
    field: &'a FieldSpec,
    span:  Span
}

struct RewriteResult {
    literal:            LitStr,
    positional_indices: BTreeSet<usize>
}

fn parse_fields(data: &DataStruct) -> syn::Result<ParsedFields> {
    parse_fields_internal(&data.fields)
}

fn parse_enum(data: &DataEnum) -> syn::Result<Vec<VariantInfo>> {
    let mut variants = Vec::with_capacity(data.variants.len());
    for variant in &data.variants {
        let display = parse_display_attr(&variant.attrs)?;
        let mut fields = parse_fields_internal(&variant.fields)?;
        if let Some(span) = parse_variant_from_attr(&variant.attrs)? {
            apply_variant_from_attr(&mut fields, span, &variant.ident)?;
        }
        variants.push(VariantInfo {
            ident: variant.ident.clone(),
            fields,
            display
        });
    }
    Ok(variants)
}

fn parse_fields_internal(fields: &Fields) -> syn::Result<ParsedFields> {
    match fields {
        Fields::Unit => Ok(ParsedFields {
            style:  FieldsStyle::Unit,
            fields: Vec::new(),
            source: None
        }),
        Fields::Named(named) => {
            let mut specs = Vec::with_capacity(named.named.len());
            let mut source = None;
            for (index, field) in named.named.iter().enumerate() {
                let attrs = parse_field_attributes(field)?;
                let ident = field.ident.clone().ok_or_else(|| {
                    syn::Error::new(field.span(), "named field missing identifier")
                })?;
                let member = Member::Named(ident.clone());
                let binding = ident.clone();
                if attrs.has_source() {
                    let kind = detect_source_kind(&field.ty)?;
                    if source.is_some() {
                        return Err(syn::Error::new(
                            attrs.source_span().unwrap_or_else(|| field.span()),
                            "only a single #[source] field is supported"
                        ));
                    }
                    source = Some(SourceField {
                        index,
                        kind
                    });
                }
                specs.push(FieldSpec {
                    member,
                    ident: Some(ident),
                    binding,
                    ty: field.ty.clone(),
                    attrs
                });
            }
            Ok(ParsedFields {
                style: FieldsStyle::Named,
                fields: specs,
                source
            })
        }
        Fields::Unnamed(unnamed) => {
            let mut specs = Vec::with_capacity(unnamed.unnamed.len());
            let mut source = None;
            for (index, field) in unnamed.unnamed.iter().enumerate() {
                let attrs = parse_field_attributes(field)?;
                let member = Member::Unnamed(index.into());
                let binding = format_ident!("__masterror_{index}");
                if attrs.has_source() {
                    let kind = detect_source_kind(&field.ty)?;
                    if source.is_some() {
                        return Err(syn::Error::new(
                            attrs.source_span().unwrap_or_else(|| field.span()),
                            "only a single #[source] field is supported"
                        ));
                    }
                    source = Some(SourceField {
                        index,
                        kind
                    });
                }
                specs.push(FieldSpec {
                    member,
                    ident: None,
                    binding,
                    ty: field.ty.clone(),
                    attrs
                });
            }
            Ok(ParsedFields {
                style: FieldsStyle::Unnamed,
                fields: specs,
                source
            })
        }
    }
}

fn parse_display_attr(attrs: &[Attribute]) -> syn::Result<LitStr> {
    let mut result = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("error")) {
        if result.is_some() {
            return Err(syn::Error::new(
                attr.span(),
                "multiple #[error(...)] attributes found"
            ));
        }
        match &attr.meta {
            Meta::List(_) => {
                let lit: LitStr = attr.parse_args()?;
                result = Some(lit);
            }
            _ => {
                return Err(syn::Error::new(
                    attr.span(),
                    r#"expected #[error("format")]"#
                ));
            }
        }
    }
    result
        .ok_or_else(|| syn::Error::new(Span::call_site(), r#"missing #[error("...")] attribute"#))
}

fn parse_field_attributes(field: &Field) -> syn::Result<FieldAttributes> {
    let mut attrs = FieldAttributes::default();
    for attr in &field.attrs {
        if attr.path().is_ident("source") {
            ensure_path_only(attr, "source")?;
            attrs.mark_source(attr.span())?;
        } else if attr.path().is_ident("from") {
            ensure_path_only(attr, "from")?;
            attrs.mark_from(attr.span())?;
        }
    }
    Ok(attrs)
}

fn ensure_path_only(attr: &Attribute, name: &str) -> syn::Result<()> {
    if !matches!(&attr.meta, Meta::Path(_)) {
        return Err(syn::Error::new(
            attr.span(),
            format!("#[{name}] attribute does not accept arguments")
        ));
    }
    Ok(())
}

fn parse_variant_from_attr(attrs: &[Attribute]) -> syn::Result<Option<Span>> {
    let mut span = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("from")) {
        ensure_path_only(attr, "from")?;
        if span.is_some() {
            return Err(syn::Error::new(attr.span(), "duplicate #[from] attribute"));
        }
        span = Some(attr.span());
    }
    Ok(span)
}

fn apply_variant_from_attr(
    fields: &mut ParsedFields,
    span: Span,
    variant_ident: &Ident
) -> syn::Result<()> {
    if fields.fields.is_empty() {
        return Err(syn::Error::new(
            span,
            format!(
                "variant `{variant_ident}` marked with #[from] must contain exactly one field"
            )
        ));
    }
    if fields.fields.len() != 1 {
        return Err(syn::Error::new(
            span,
            format!(
                "variant `{variant_ident}` marked with #[from] must contain exactly one field"
            )
        ));
    }
    let field = fields
        .fields
        .get_mut(0)
        .ok_or_else(|| syn::Error::new(span, "invalid #[from] field index"))?;
    field.attrs.mark_from(span)
}

fn find_from_field<'a>(
    fields: &'a ParsedFields,
    context: &str
) -> syn::Result<Option<FromFieldInfo<'a>>> {
    let mut info = None;
    for field in &fields.fields {
        if let Some(span) = field.attrs.span_of_from_attribute() {
            if info.is_some() {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "multiple #[from] attributes in {context}; only one field may use #[from]"
                    )
                ));
            }
            info = Some(FromFieldInfo {
                field,
                span
            });
        }
    }
    let Some(info) = info else {
        return Ok(None);
    };
    if fields.fields.len() != 1 {
        return Err(syn::Error::new(
            info.span,
            format!("using #[from] in {context} requires exactly one field")
        ));
    }
    Ok(Some(info))
}

fn detect_source_kind(ty: &Type) -> syn::Result<SourceKind> {
    if let Some(inner) = option_inner_type(ty) {
        Ok(SourceKind::Optional {
            needs_deref: needs_deref(inner)
        })
    } else {
        Ok(SourceKind::Direct {
            needs_deref: needs_deref(ty)
        })
    }
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && type_path.qself.is_none()
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(GenericArgument::Type(inner)) = args.args.first()
    {
        return Some(inner);
    }
    None
}

fn needs_deref(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_some() {
            return false;
        }
        if let Some(segment) = type_path.path.segments.last() {
            let ident = segment.ident.to_string();
            return matches!(ident.as_str(), "Box" | "Rc" | "Arc");
        }
    }
    false
}

fn build_struct_display(
    ident: &Ident,
    generics: &Generics,
    fields: &ParsedFields,
    display: &LitStr
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let RewriteResult {
        literal,
        positional_indices
    } = rewrite_format_string(display, fields.fields.len())?;

    let body = match fields.style {
        FieldsStyle::Unit => quote! {
            ::core::write!(formatter, #literal)
        },
        FieldsStyle::Named => {
            let field_idents: Vec<_> = fields
                .fields
                .iter()
                .map(|f| f.ident.clone().expect("named fields must have identifiers"))
                .collect();
            let positional_bindings = positional_indices.iter().map(|index| {
                let binding_ident = format_ident!("__masterror_{index}");
                let field_ident = field_idents[*index].clone();
                quote! {
                    #[allow(unused_variables)]
                    let #binding_ident = &*#field_ident;
                }
            });
            quote! {
                let Self { #( ref #field_idents ),* } = *self;
                #[allow(unused_variables)]
                let _ = (#(&#field_idents),*);
                #(#positional_bindings)*
                ::core::write!(formatter, #literal)
            }
        }
        FieldsStyle::Unnamed => {
            let bindings: Vec<_> = fields.fields.iter().map(|f| f.binding.clone()).collect();
            quote! {
                let Self( #( ref #bindings ),* ) = *self;
                #[allow(unused_variables)]
                let _ = (#(&#bindings),*);
                ::core::write!(formatter, #literal)
            }
        }
    };

    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #body
            }
        }
    })
}

fn build_struct_error(
    ident: &Ident,
    generics: &Generics,
    fields: &ParsedFields
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let source_expr = if let Some(source) = fields.source {
        let field = fields
            .fields
            .get(source.index)
            .ok_or_else(|| syn::Error::new(Span::call_site(), "invalid source field index"))?;
        let member = &field.member;
        match source.kind {
            SourceKind::Direct {
                needs_deref: false
            } => quote! {
                ::core::option::Option::Some(&self.#member as &(dyn ::std::error::Error + 'static))
            },
            SourceKind::Direct {
                needs_deref: true
            } => quote! {
                ::core::option::Option::Some(self.#member.as_ref() as &(dyn ::std::error::Error + 'static))
            },
            SourceKind::Optional {
                needs_deref: false
            } => quote! {
                self.#member
                    .as_ref()
                    .map(|source| source as &(dyn ::std::error::Error + 'static))
            },
            SourceKind::Optional {
                needs_deref: true
            } => quote! {
                self.#member
                    .as_ref()
                    .map(|source| source.as_ref() as &(dyn ::std::error::Error + 'static))
            }
        }
    } else {
        quote! { ::core::option::Option::None }
    };

    Ok(quote! {
        impl #impl_generics ::std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> ::core::option::Option<&(dyn ::std::error::Error + 'static)> {
                #source_expr
            }
        }
    })
}

fn build_struct_from_impl(
    ident: &Ident,
    generics: &Generics,
    fields: &ParsedFields
) -> syn::Result<TokenStream2> {
    let context = format!("struct `{ident}`");
    let Some(from_info) = find_from_field(fields, &context)? else {
        return Ok(TokenStream2::new());
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let field_ty = &from_info.field.ty;
    let arg_ident = format_ident!("__masterror_from_value");

    let construct = match fields.style {
        FieldsStyle::Named => {
            let field_ident = from_info.field.ident.clone().ok_or_else(|| {
                syn::Error::new(from_info.span, "named field missing identifier")
            })?;
            quote! { Self { #field_ident: #arg_ident } }
        }
        FieldsStyle::Unnamed => {
            if fields.fields.len() != 1 {
                return Err(syn::Error::new(
                    from_info.span,
                    format!("using #[from] in {context} requires exactly one field")
                ));
            }
            quote! { Self(#arg_ident) }
        }
        FieldsStyle::Unit => {
            return Err(syn::Error::new(
                from_info.span,
                format!("using #[from] in {context} requires at least one field")
            ));
        }
    };

    Ok(quote! {
        impl #impl_generics ::core::convert::From<#field_ty> for #ident #ty_generics #where_clause {
            fn from(#arg_ident: #field_ty) -> Self {
                #construct
            }
        }
    })
}

fn build_enum_display(
    ident: &Ident,
    generics: &Generics,
    variants: &[VariantInfo]
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut arms = Vec::with_capacity(variants.len());
    for variant in variants {
        let variant_ident = &variant.ident;
        let RewriteResult {
            literal,
            positional_indices
        } = rewrite_format_string(&variant.display, variant.fields.fields.len())?;
        let arm = match variant.fields.style {
            FieldsStyle::Unit => quote! {
                Self::#variant_ident => ::core::write!(formatter, #literal)
            },
            FieldsStyle::Named => {
                let bindings: Vec<_> = variant
                    .fields
                    .fields
                    .iter()
                    .map(|f| {
                        f.ident
                            .clone()
                            .expect("named variant field requires identifier")
                    })
                    .collect();
                let positional_bindings = positional_indices.iter().map(|index| {
                    let binding_ident = format_ident!("__masterror_{index}");
                    let field_ident = bindings[*index].clone();
                    quote! {
                        #[allow(unused_variables)]
                        let #binding_ident = &*#field_ident;
                    }
                });
                quote! {
                    Self::#variant_ident { #( #bindings ),* } => {
                        #[allow(unused_variables)]
                        let _ = (#(&#bindings),*);
                        #(#positional_bindings)*
                        ::core::write!(formatter, #literal)
                    }
                }
            }
            FieldsStyle::Unnamed => {
                let bindings: Vec<_> = variant
                    .fields
                    .fields
                    .iter()
                    .map(|f| f.binding.clone())
                    .collect();
                quote! {
                    Self::#variant_ident( #( #bindings ),* ) => {
                        #[allow(unused_variables)]
                        let _ = (#(&#bindings),*);
                        ::core::write!(formatter, #literal)
                    }
                }
            }
        };
        arms.push(arm);
    }

    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}

fn build_enum_error(
    ident: &Ident,
    generics: &Generics,
    variants: &[VariantInfo]
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut arms = Vec::with_capacity(variants.len());
    for variant in variants {
        let variant_ident = &variant.ident;
        let arm = match variant.fields.style {
            FieldsStyle::Unit => quote! {
                Self::#variant_ident => ::core::option::Option::None
            },
            FieldsStyle::Named => {
                let bindings: Vec<_> = variant
                    .fields
                    .fields
                    .iter()
                    .map(|f| {
                        f.ident
                            .clone()
                            .expect("named variant field requires identifier")
                    })
                    .collect();
                let source_expr = if let Some(source) = variant.fields.source {
                    let binding = bindings[source.index].clone();
                    match source.kind {
                        SourceKind::Direct {
                            needs_deref: false
                        } => quote! {
                            ::core::option::Option::Some(#binding as &(dyn ::std::error::Error + 'static))
                        },
                        SourceKind::Direct {
                            needs_deref: true
                        } => quote! {
                            ::core::option::Option::Some(#binding.as_ref() as &(dyn ::std::error::Error + 'static))
                        },
                        SourceKind::Optional {
                            needs_deref: false
                        } => quote! {
                            #binding
                                .as_ref()
                                .map(|source| source as &(dyn ::std::error::Error + 'static))
                        },
                        SourceKind::Optional {
                            needs_deref: true
                        } => quote! {
                            #binding
                                .as_ref()
                                .map(|source| source.as_ref() as &(dyn ::std::error::Error + 'static))
                        }
                    }
                } else {
                    quote! { ::core::option::Option::None }
                };
                quote! {
                    Self::#variant_ident { #( #bindings ),* } => {
                        #source_expr
                    }
                }
            }
            FieldsStyle::Unnamed => {
                let bindings: Vec<_> = variant
                    .fields
                    .fields
                    .iter()
                    .map(|f| f.binding.clone())
                    .collect();
                let source_expr = if let Some(source) = variant.fields.source {
                    let binding = bindings[source.index].clone();
                    match source.kind {
                        SourceKind::Direct {
                            needs_deref: false
                        } => quote! {
                            ::core::option::Option::Some(#binding as &(dyn ::std::error::Error + 'static))
                        },
                        SourceKind::Direct {
                            needs_deref: true
                        } => quote! {
                            ::core::option::Option::Some(#binding.as_ref() as &(dyn ::std::error::Error + 'static))
                        },
                        SourceKind::Optional {
                            needs_deref: false
                        } => quote! {
                            #binding
                                .as_ref()
                                .map(|source| source as &(dyn ::std::error::Error + 'static))
                        },
                        SourceKind::Optional {
                            needs_deref: true
                        } => quote! {
                            #binding
                                .as_ref()
                                .map(|source| source.as_ref() as &(dyn ::std::error::Error + 'static))
                        }
                    }
                } else {
                    quote! { ::core::option::Option::None }
                };
                quote! {
                    Self::#variant_ident( #( #bindings ),* ) => {
                        #source_expr
                    }
                }
            }
        };
        arms.push(arm);
    }

    Ok(quote! {
        impl #impl_generics ::std::error::Error for #ident #ty_generics #where_clause {
            fn source(&self) -> ::core::option::Option<&(dyn ::std::error::Error + 'static)> {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}

fn build_enum_from_impls(
    ident: &Ident,
    generics: &Generics,
    variants: &[VariantInfo]
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut impls = Vec::new();

    for variant in variants {
        let context = format!("variant `{}`", variant.ident);
        if let Some(from_info) = find_from_field(&variant.fields, &context)? {
            let field_ty = &from_info.field.ty;
            let variant_ident = &variant.ident;
            let arg_ident = format_ident!("__masterror_from_value");

            let body = match variant.fields.style {
                FieldsStyle::Named => {
                    let field_ident = from_info.field.ident.clone().ok_or_else(|| {
                        syn::Error::new(from_info.span, "named field missing identifier")
                    })?;
                    quote! { Self::#variant_ident { #field_ident: #arg_ident } }
                }
                FieldsStyle::Unnamed => {
                    if variant.fields.fields.len() != 1 {
                        return Err(syn::Error::new(
                            from_info.span,
                            format!("using #[from] in {context} requires exactly one field")
                        ));
                    }
                    quote! { Self::#variant_ident(#arg_ident) }
                }
                FieldsStyle::Unit => {
                    return Err(syn::Error::new(
                        from_info.span,
                        format!("{context} cannot be unit-like when using #[from]")
                    ));
                }
            };

            impls.push(quote! {
                impl #impl_generics ::core::convert::From<#field_ty> for #ident #ty_generics #where_clause {
                    fn from(#arg_ident: #field_ty) -> Self {
                        #body
                    }
                }
            });
        }
    }

    Ok(quote! { #(#impls)* })
}

fn rewrite_format_string(original: &LitStr, field_count: usize) -> syn::Result<RewriteResult> {
    let src = original.value();
    let mut result = String::with_capacity(src.len());
    let mut positional_indices = BTreeSet::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    let len = bytes.len();
    let mut next_implicit = 0usize;

    while i < len {
        match bytes[i] {
            b'{' => {
                if i + 1 < len && bytes[i + 1] == b'{' {
                    result.push_str("{{");
                    i += 2;
                    continue;
                }
                let start = i + 1;
                let mut j = start;
                while j < len {
                    if bytes[j] == b'}' {
                        break;
                    }
                    if bytes[j] == b'{' {
                        return Err(syn::Error::new(
                            original.span(),
                            "nested '{' inside format string is not supported"
                        ));
                    }
                    j += 1;
                }
                if j == len {
                    return Err(syn::Error::new(
                        original.span(),
                        "unmatched '{' in format string"
                    ));
                }
                let content = &src[start..j];
                let (arg, rest) = if let Some(pos) = content.find(':') {
                    (&content[..pos], Some(&content[pos + 1..]))
                } else {
                    (content, None)
                };
                let trimmed = arg.trim();
                let mut used_index = None;
                if trimmed.is_empty() {
                    used_index = Some(next_implicit);
                    next_implicit += 1;
                } else if trimmed.chars().all(|ch| ch.is_ascii_digit()) {
                    let idx: usize = trimmed.parse().map_err(|_| {
                        syn::Error::new(original.span(), "invalid positional index")
                    })?;
                    used_index = Some(idx);
                }
                result.push('{');
                if let Some(idx) = used_index {
                    if idx >= field_count {
                        return Err(syn::Error::new(
                            original.span(),
                            "format index exceeds field count"
                        ));
                    }
                    positional_indices.insert(idx);
                    let ident = format!("__masterror_{}", idx);
                    result.push_str(&ident);
                } else {
                    result.push_str(arg);
                }
                if let Some(rest) = rest {
                    result.push(':');
                    result.push_str(rest);
                }
                result.push('}');
                i = j + 1;
            }
            b'}' => {
                if i + 1 < len && bytes[i + 1] == b'}' {
                    result.push_str("}}");
                    i += 2;
                } else {
                    return Err(syn::Error::new(
                        original.span(),
                        "unmatched '}' in format string"
                    ));
                }
            }
            _ => {
                let start = i;
                i += 1;
                while i < len && bytes[i] != b'{' && bytes[i] != b'}' {
                    i += 1;
                }
                result.push_str(&src[start..i]);
            }
        }
    }

    Ok(RewriteResult {
        literal: LitStr::new(&result, original.span()),
        positional_indices
    })
}
