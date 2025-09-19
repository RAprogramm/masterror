use proc_macro2::Span;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field as SynField,
    Fields as SynFields, Ident, LitStr, spanned::Spanned
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
    pub fields:  Fields,
    pub display: DisplaySpec
}

#[derive(Debug)]
pub struct VariantData {
    pub ident:   Ident,
    pub fields:  Fields,
    pub display: DisplaySpec,
    pub span:    Span
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

        let attrs = FieldAttrs::from_attrs(&field.attrs, errors);

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
    pub from:      Option<Attribute>,
    pub source:    Option<Attribute>,
    pub backtrace: Option<Attribute>
}

impl FieldAttrs {
    fn from_attrs(attrs: &[Attribute], errors: &mut Vec<Error>) -> Self {
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

        result
    }
}

#[derive(Debug)]
pub enum DisplaySpec {
    Transparent { attribute: Box<Attribute> },
    Template(DisplayTemplate)
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
    validate_transparent(&fields, &display, errors, None);

    Ok(ErrorData::Struct(Box::new(StructData {
        fields,
        display
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
    validate_transparent(&fields, &display, errors, Some(&variant));

    Ok(VariantData {
        ident: variant.ident,
        fields,
        display,
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
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "unexpected tokens after string literal"
                ));
            }
            let template = parse_display_template(lit)?;
            Ok(DisplaySpec::Template(template))
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            if ident != "transparent" {
                return Err(Error::new(
                    ident.span(),
                    "expected string literal or `transparent`"
                ));
            }
            if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "unexpected tokens after `transparent`"
                ));
            }
            Ok(DisplaySpec::Transparent {
                attribute: Box::new(attr.clone())
            })
        } else {
            Err(Error::new(
                input.span(),
                "expected string literal or `transparent`"
            ))
        }
    })
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

        if fields.len() > 1
            && let Some(attr) = &field.attrs.from
        {
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

pub fn placeholder_error(span: Span, identifier: &TemplateIdentifierSpec) -> Error {
    match identifier {
        TemplateIdentifierSpec::Named(name) => {
            Error::new(span, format!("unknown field `{}`", name))
        }
        TemplateIdentifierSpec::Positional(index) => {
            Error::new(span, format!("field `{}` is not available", index))
        }
    }
}
