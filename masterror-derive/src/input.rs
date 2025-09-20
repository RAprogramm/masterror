use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Expr, ExprPath, Field as SynField,
    Fields as SynFields, GenericArgument, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned
};

use crate::template_support::{DisplayTemplate, TemplateIdentifierSpec, parse_display_template};

#[derive(Debug)]
pub struct ErrorInput {
    pub ident:    Ident,
    pub generics: syn::Generics,
    pub data:     ErrorData
}

#[derive(Debug)]
pub enum ErrorData {
    Struct(Box<StructData>),
    Enum(Vec<VariantData>)
}

#[derive(Debug)]
pub struct StructData {
    pub fields:      Fields,
    pub display:     DisplaySpec,
    #[allow(dead_code)]
    pub format_args: FormatArgsSpec
}

#[derive(Debug)]
pub struct VariantData {
    pub ident:       Ident,
    pub fields:      Fields,
    pub display:     DisplaySpec,
    #[allow(dead_code)]
    pub format_args: FormatArgsSpec,
    pub span:        Span
}

#[derive(Debug)]
pub enum Fields {
    Unit,
    Named(Vec<Field>),
    Unnamed(Vec<Field>)
}

impl Fields {
    pub fn len(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Named(fields) | Self::Unnamed(fields) => fields.len()
        }
    }

    pub fn iter(&self) -> FieldIter<'_> {
        match self {
            Self::Unit => FieldIter::Empty,
            Self::Named(fields) | Self::Unnamed(fields) => FieldIter::Slice(fields.iter())
        }
    }

    pub fn get_named(&self, name: &str) -> Option<&Field> {
        match self {
            Self::Named(fields) => fields
                .iter()
                .find(|field| field.ident.as_ref().is_some_and(|ident| ident == name)),
            _ => None
        }
    }

    pub fn get_positional(&self, index: usize) -> Option<&Field> {
        match self {
            Self::Unnamed(fields) => fields.get(index),
            _ => None
        }
    }

    pub fn from_syn(fields: &SynFields, errors: &mut Vec<Error>) -> Self {
        match fields {
            SynFields::Unit => Self::Unit,
            SynFields::Named(named) => {
                let mut items = Vec::new();
                for (index, field) in named.named.iter().enumerate() {
                    items.push(Field::from_syn(field, index, errors));
                }
                Self::Named(items)
            }
            SynFields::Unnamed(unnamed) => {
                let mut items = Vec::new();
                for (index, field) in unnamed.unnamed.iter().enumerate() {
                    items.push(Field::from_syn(field, index, errors));
                }
                Self::Unnamed(items)
            }
        }
    }

    pub fn first_from_field(&self) -> Option<&Field> {
        self.iter().find(|field| field.attrs.from.is_some())
    }

    pub fn backtrace_field(&self) -> Option<&Field> {
        self.iter().find(|field| field.attrs.has_backtrace())
    }
}

pub enum FieldIter<'a> {
    Empty,
    Slice(std::slice::Iter<'a, Field>)
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = &'a Field;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FieldIter::Empty => None,
            FieldIter::Slice(iter) => iter.next()
        }
    }
}

#[derive(Debug)]
pub struct Field {
    pub ident:  Option<Ident>,
    pub member: syn::Member,
    pub ty:     syn::Type,
    pub attrs:  FieldAttrs,
    pub span:   Span,
    pub index:  usize
}

impl Field {
    fn from_syn(field: &SynField, index: usize, errors: &mut Vec<Error>) -> Self {
        let ident = field.ident.clone();
        let member = match &ident {
            Some(name) => syn::Member::Named(name.clone()),
            None => syn::Member::Unnamed(syn::Index::from(index))
        };

        let attrs = FieldAttrs::from_attrs(&field.attrs, ident.as_ref(), &field.ty, errors);

        Self {
            ident,
            member,
            ty: field.ty.clone(),
            attrs,
            span: field.span(),
            index
        }
    }
}

#[derive(Debug, Default)]
pub struct FieldAttrs {
    pub from:           Option<Attribute>,
    pub source:         Option<Attribute>,
    pub backtrace:      Option<Attribute>,
    inferred_source:    bool,
    inferred_backtrace: bool
}

impl FieldAttrs {
    fn from_attrs(
        attrs: &[Attribute],
        ident: Option<&Ident>,
        ty: &syn::Type,
        errors: &mut Vec<Error>
    ) -> Self {
        let mut result = FieldAttrs::default();

        for attr in attrs {
            if path_is(attr, "from") {
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                    continue;
                }
                if result.from.is_some() {
                    errors.push(Error::new_spanned(attr, "duplicate #[from] attribute"));
                    continue;
                }
                result.from = Some(attr.clone());
            } else if path_is(attr, "source") {
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                    continue;
                }
                if result.source.is_some() {
                    errors.push(Error::new_spanned(attr, "duplicate #[source] attribute"));
                    continue;
                }
                result.source = Some(attr.clone());
            } else if path_is(attr, "backtrace") {
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                    continue;
                }
                if result.backtrace.is_some() {
                    errors.push(Error::new_spanned(attr, "duplicate #[backtrace] attribute"));
                    continue;
                }
                result.backtrace = Some(attr.clone());
            }
        }

        if result.source.is_none()
            && let Some(attr) = &result.from
        {
            result.source = Some(attr.clone());
        }

        if result.source.is_none() && ident.is_some_and(|ident| ident == "source") {
            result.inferred_source = true;
        }

        if result.backtrace.is_none() {
            if is_option_type(ty) {
                if option_inner_type(ty).is_some_and(is_backtrace_type) {
                    result.inferred_backtrace = true;
                }
            } else if is_backtrace_type(ty) {
                result.inferred_backtrace = true;
            }
        }

        result
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some() || self.inferred_source
    }

    pub fn has_backtrace(&self) -> bool {
        self.backtrace.is_some() || self.inferred_backtrace
    }

    pub fn is_backtrace_inferred(&self) -> bool {
        self.inferred_backtrace
    }

    pub fn source_attribute(&self) -> Option<&Attribute> {
        self.source.as_ref()
    }

    pub fn backtrace_attribute(&self) -> Option<&Attribute> {
        self.backtrace.as_ref()
    }
}

#[derive(Debug)]
pub enum DisplaySpec {
    Transparent {
        attribute: Box<Attribute>
    },
    Template(DisplayTemplate),
    #[allow(dead_code)]
    TemplateWithArgs {
        template: DisplayTemplate,
        args:     FormatArgsSpec
    },
    #[allow(dead_code)]
    FormatterPath {
        path: ExprPath,
        args: FormatArgsSpec
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FormatArgsSpec {
    pub args: Vec<FormatArg>
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FormatArg {
    pub tokens: TokenStream,
    pub expr:   Expr,
    pub kind:   FormatBindingKind,
    pub span:   Span
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum FormatBindingKind {
    Named(Ident),
    Positional(usize),
    Implicit(usize)
}

pub fn parse_input(input: DeriveInput) -> Result<ErrorInput, Error> {
    let mut errors = Vec::new();

    let ident = input.ident;
    let generics = input.generics;

    let data = match input.data {
        Data::Struct(data) => parse_struct(&ident, &input.attrs, data, &mut errors),
        Data::Enum(data) => parse_enum(&input.attrs, data, &mut errors),
        Data::Union(union) => {
            errors.push(Error::new(
                union.union_token.span(),
                "Error cannot be derived for unions"
            ));
            Err(())
        }
    };

    let data = match data {
        Ok(value) => value,
        Err(()) => {
            return Err(collect_errors(errors));
        }
    };

    if errors.is_empty() {
        Ok(ErrorInput {
            ident,
            generics,
            data
        })
    } else {
        Err(collect_errors(errors))
    }
}

fn parse_struct(
    ident: &Ident,
    attrs: &[Attribute],
    data: DataStruct,
    errors: &mut Vec<Error>
) -> Result<ErrorData, ()> {
    let display = extract_display_spec(attrs, ident.span(), errors)?;
    let fields = Fields::from_syn(&data.fields, errors);

    validate_from_usage(&fields, &display, errors);
    validate_backtrace_usage(&fields, errors);
    validate_transparent(&fields, &display, errors, None);

    Ok(ErrorData::Struct(Box::new(StructData {
        fields,
        display,
        format_args: FormatArgsSpec::default()
    })))
}

fn parse_enum(
    attrs: &[Attribute],
    data: DataEnum,
    errors: &mut Vec<Error>
) -> Result<ErrorData, ()> {
    for attr in attrs {
        if path_is(attr, "error") {
            errors.push(Error::new_spanned(
                attr,
                "type-level #[error] attributes are not supported"
            ));
        }
    }

    let mut variants = Vec::new();
    for variant in data.variants {
        variants.push(parse_variant(variant, errors)?);
    }

    Ok(ErrorData::Enum(variants))
}

fn parse_variant(variant: syn::Variant, errors: &mut Vec<Error>) -> Result<VariantData, ()> {
    let span = variant.span();
    for attr in &variant.attrs {
        if path_is(attr, "from") {
            errors.push(Error::new_spanned(
                attr,
                "not expected here; the #[from] attribute belongs on a specific field"
            ));
        }
    }

    let display = extract_display_spec(&variant.attrs, span, errors)?;
    let fields = Fields::from_syn(&variant.fields, errors);

    validate_from_usage(&fields, &display, errors);
    validate_backtrace_usage(&fields, errors);
    validate_transparent(&fields, &display, errors, Some(&variant));

    Ok(VariantData {
        ident: variant.ident,
        fields,
        display,
        format_args: FormatArgsSpec::default(),
        span
    })
}

fn extract_display_spec(
    attrs: &[Attribute],
    missing_span: Span,
    errors: &mut Vec<Error>
) -> Result<DisplaySpec, ()> {
    let mut display = None;

    for attr in attrs {
        if !path_is(attr, "error") {
            continue;
        }

        if display.is_some() {
            errors.push(Error::new_spanned(attr, "duplicate #[error] attribute"));
            continue;
        }

        match parse_error_attribute(attr) {
            Ok(spec) => display = Some(spec),
            Err(err) => errors.push(err)
        }
    }

    match display {
        Some(spec) => Ok(spec),
        None => {
            errors.push(Error::new(missing_span, "missing #[error(...)] attribute"));
            Err(())
        }
    }
}

fn parse_error_attribute(attr: &Attribute) -> Result<DisplaySpec, Error> {
    mod kw {
        syn::custom_keyword!(transparent);
        syn::custom_keyword!(fmt);
    }

    attr.parse_args_with(|input: ParseStream| {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            let template = parse_display_template(lit)?;
            let args = parse_format_args(input)?;

            if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "unexpected tokens after format arguments"
                ));
            }

            if args.args.is_empty() {
                Ok(DisplaySpec::Template(template))
            } else {
                Ok(DisplaySpec::TemplateWithArgs {
                    template,
                    args
                })
            }
        } else if input.peek(kw::transparent) {
            let _: kw::transparent = input.parse()?;

            if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "format arguments are not supported with #[error(transparent)]"
                ));
            }

            Ok(DisplaySpec::Transparent {
                attribute: Box::new(attr.clone())
            })
        } else if input.peek(kw::fmt) {
            input.parse::<kw::fmt>()?;
            input.parse::<Token![=]>()?;
            let path: ExprPath = input.parse()?;
            let args = parse_format_args(input)?;

            for arg in &args.args {
                if let FormatBindingKind::Named(ident) = &arg.kind
                    && ident == "fmt"
                {
                    return Err(Error::new(arg.span, "duplicate `fmt` handler specified"));
                }
            }

            if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "`fmt = ...` cannot be combined with additional arguments"
                ));
            }

            Ok(DisplaySpec::FormatterPath {
                path,
                args
            })
        } else {
            Err(Error::new(
                input.span(),
                "expected string literal, `transparent`, or `fmt = ...`"
            ))
        }
    })
}

fn parse_format_args(input: ParseStream) -> Result<FormatArgsSpec, Error> {
    let mut args = FormatArgsSpec::default();

    if input.is_empty() {
        return Ok(args);
    }

    let leading_comma = if input.peek(Token![,]) {
        let comma: Token![,] = input.parse()?;
        Some(comma.span)
    } else {
        None
    };

    if input.is_empty() {
        if let Some(span) = leading_comma {
            return Err(Error::new(span, "expected format argument after comma"));
        }
        return Ok(args);
    }

    let parsed = syn::punctuated::Punctuated::<RawFormatArg, Token![,]>::parse_terminated(input)?;

    let mut seen_named = HashSet::new();

    let mut positional_index = 0usize;

    for raw in parsed {
        match raw {
            RawFormatArg::Named {
                ident,
                expr,
                span
            } => {
                let name_key = ident.to_string();
                if !seen_named.insert(name_key) {
                    return Err(Error::new(
                        ident.span(),
                        format!("duplicate format argument `{ident}`")
                    ));
                }

                let tokens = quote!(#ident = #expr);
                args.args.push(FormatArg {
                    tokens,
                    expr,
                    kind: FormatBindingKind::Named(ident),
                    span
                });
            }
            RawFormatArg::Positional {
                expr,
                span
            } => {
                let index = positional_index;
                positional_index += 1;
                let tokens = expr.to_token_stream();
                args.args.push(FormatArg {
                    tokens,
                    expr,
                    kind: FormatBindingKind::Positional(index),
                    span
                });
            }
        }
    }

    Ok(args)
}

enum RawFormatArg {
    Named {
        ident: Ident,
        expr:  Expr,
        span:  Span
    },
    Positional {
        expr: Expr,
        span: Span
    }
}

impl Parse for RawFormatArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![=]) {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let expr: Expr = input.parse()?;
            let span = ident
                .span()
                .join(expr.span())
                .unwrap_or_else(|| ident.span());
            Ok(Self::Named {
                ident,
                expr,
                span
            })
        } else {
            let expr: Expr = input.parse()?;
            let span = expr.span();
            Ok(Self::Positional {
                expr,
                span
            })
        }
    }
}

fn validate_from_usage(fields: &Fields, display: &DisplaySpec, errors: &mut Vec<Error>) {
    let mut from_fields = fields.iter().filter(|field| field.attrs.from.is_some());
    let first = from_fields.next();
    let second = from_fields.next();

    if let Some(field) = first {
        if second.is_some() {
            if let Some(attr) = &field.attrs.from {
                errors.push(Error::new_spanned(
                    attr,
                    "multiple #[from] fields are not supported"
                ));
            }
            return;
        }

        let mut has_unexpected_companions = false;
        for companion in fields.iter() {
            if companion.index == field.index {
                continue;
            }

            if companion.attrs.has_backtrace() {
                continue;
            }

            if companion.attrs.has_source() {
                if companion.attrs.from.is_none() && !is_option_type(&companion.ty) {
                    if let Some(attr) = companion.attrs.source_attribute() {
                        errors.push(Error::new_spanned(
                            attr,
                            "additional #[source] fields used with #[from] must be Option<_>"
                        ));
                    } else {
                        errors.push(Error::new(
                            companion.span,
                            "additional #[source] fields used with #[from] must be Option<_>"
                        ));
                    }
                }
                continue;
            }

            has_unexpected_companions = true;
        }

        if has_unexpected_companions && let Some(attr) = &field.attrs.from {
            errors.push(Error::new_spanned(
                attr,
                "deriving From requires no fields other than source and backtrace"
            ));
        }

        if matches!(display, DisplaySpec::Transparent { .. })
            && fields.len() != 1
            && let Some(attr) = &field.attrs.from
        {
            errors.push(Error::new_spanned(
                attr,
                "#[error(transparent)] requires exactly one field"
            ));
        }
    }
}

fn validate_backtrace_usage(fields: &Fields, errors: &mut Vec<Error>) {
    let backtrace_fields: Vec<_> = fields
        .iter()
        .filter(|field| field.attrs.has_backtrace())
        .collect();

    for field in &backtrace_fields {
        validate_backtrace_field_type(field, errors);
    }

    if backtrace_fields.len() <= 1 {
        return;
    }

    for field in backtrace_fields.iter().skip(1) {
        if let Some(attr) = field.attrs.backtrace_attribute() {
            errors.push(Error::new_spanned(
                attr,
                "multiple #[backtrace] fields are not supported"
            ));
        } else {
            errors.push(Error::new(
                field.span,
                "multiple #[backtrace] fields are not supported"
            ));
        }
    }
}

fn validate_backtrace_field_type(field: &Field, errors: &mut Vec<Error>) {
    let Some(attr) = field.attrs.backtrace_attribute() else {
        if field.attrs.is_backtrace_inferred() {
            return;
        }
        return;
    };

    let ty = &field.ty;
    if is_option_type(ty) {
        if option_inner_type(ty).is_some_and(is_backtrace_type) {
            return;
        }
    } else if is_backtrace_type(ty) {
        return;
    }

    errors.push(Error::new_spanned(
        attr,
        "fields with #[backtrace] must be std::backtrace::Backtrace or Option<std::backtrace::Backtrace>"
    ));
}

fn validate_transparent(
    fields: &Fields,
    display: &DisplaySpec,
    errors: &mut Vec<Error>,
    variant: Option<&syn::Variant>
) {
    if fields.len() == 1 {
        return;
    }

    if let DisplaySpec::Transparent {
        attribute
    } = display
    {
        match variant {
            Some(variant) => {
                errors.push(Error::new_spanned(
                    variant,
                    "#[error(transparent)] requires exactly one field"
                ));
            }
            None => {
                errors.push(Error::new_spanned(
                    attribute.as_ref(),
                    "#[error(transparent)] requires exactly one field"
                ));
            }
        }
    }
}

fn path_is(attr: &Attribute, expected: &str) -> bool {
    attr.path().is_ident(expected)
}

fn collect_errors(errors: Vec<Error>) -> Error {
    let mut iter = errors.into_iter();
    let mut root = iter
        .next()
        .unwrap_or_else(|| Error::new(Span::call_site(), "unexpected error"));
    for err in iter {
        root.combine(err);
    }
    root
}

pub fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(path) = ty {
        if path.qself.is_some() {
            return false;
        }
        if let Some(last) = path.path.segments.last()
            && last.ident == "Option"
        {
            return true;
        }
    }
    false
}

fn option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    if path.qself.is_some() {
        return None;
    }
    let last = path.path.segments.last()?;
    if last.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(arguments) = &last.arguments else {
        return None;
    };
    arguments.args.iter().find_map(|argument| match argument {
        GenericArgument::Type(inner) => Some(inner),
        _ => None
    })
}

fn is_backtrace_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    if path.qself.is_some() {
        return false;
    }
    let Some(last) = path.path.segments.last() else {
        return false;
    };
    last.ident == "Backtrace" && matches!(last.arguments, syn::PathArguments::None)
}

pub fn placeholder_error(span: Span, identifier: &TemplateIdentifierSpec) -> Error {
    match identifier {
        TemplateIdentifierSpec::Named(name) => {
            Error::new(span, format!("unknown field `{}`", name))
        }
        TemplateIdentifierSpec::Positional(index) => {
            Error::new(span, format!("field `{}` is not available", index))
        }
        TemplateIdentifierSpec::Implicit(index) => {
            Error::new(span, format!("field `{}` is not available", index))
        }
    }
}
