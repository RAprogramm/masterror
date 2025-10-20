// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Core data structures for parsed error definitions.
//!
//! This module defines all types used to represent parsed error attributes,
//! fields, display specifications, and format arguments from derive macro
//! input.

use proc_macro2::Span;
use syn::{
    AngleBracketedGenericArguments, Attribute, Error, Expr, ExprPath, Field as SynField,
    Fields as SynFields, Ident, LitStr, Token, TypePath, punctuated::Punctuated, spanned::Spanned,
    token::Paren
};

use crate::template_support::DisplayTemplate;

/// Top-level parsed error type definition.
///
/// Represents either a struct or enum error type with its associated metadata.
#[derive(Debug)]
pub struct ErrorInput {
    pub ident:    Ident,
    pub generics: syn::Generics,
    pub data:     ErrorData
}

/// Error definition data (struct or enum).
#[derive(Debug)]
pub enum ErrorData {
    Struct(Box<StructData>),
    Enum(Vec<VariantData>)
}

/// Parsed struct error data.
#[derive(Debug)]
pub struct StructData {
    pub fields:      Fields,
    pub display:     DisplaySpec,
    #[allow(dead_code)]
    pub format_args: FormatArgsSpec,
    pub app_error:   Option<AppErrorSpec>,
    pub masterror:   Option<MasterrorSpec>
}

/// Parsed enum variant error data.
#[derive(Debug)]
pub struct VariantData {
    pub ident:       Ident,
    pub fields:      Fields,
    pub display:     DisplaySpec,
    #[allow(dead_code)]
    pub format_args: FormatArgsSpec,
    pub app_error:   Option<AppErrorSpec>,
    pub masterror:   Option<MasterrorSpec>,
    pub span:        Span
}

/// AppError attribute specification.
///
/// Configures error kind, code, and message exposure for app-level errors.
#[derive(Clone, Debug)]
pub struct AppErrorSpec {
    pub kind:           ExprPath,
    pub code:           Option<ExprPath>,
    pub expose_message: bool,
    pub attribute_span: Span
}

/// Masterror attribute specification.
///
/// Configures error code, category, redaction, telemetry, and transport
/// mappings.
#[derive(Clone, Debug)]
pub struct MasterrorSpec {
    pub code:           Expr,
    pub category:       ExprPath,
    pub expose_message: bool,
    pub redact:         RedactSpec,
    pub telemetry:      Vec<Expr>,
    pub map_grpc:       Option<Expr>,
    pub map_problem:    Option<Expr>,
    #[allow(dead_code)]
    pub attribute_span: Span
}

/// Field redaction configuration.
///
/// Specifies whether to redact the message and which fields to redact.
#[derive(Clone, Debug, Default)]
pub struct RedactSpec {
    pub message: bool,
    pub fields:  Vec<FieldRedactionSpec>
}

/// Individual field redaction specification.
#[derive(Clone, Debug)]
pub struct FieldRedactionSpec {
    pub name:   LitStr,
    pub policy: FieldRedactionKind
}

/// Field redaction strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldRedactionKind {
    None,
    Redact,
    Hash,
    Last4
}

/// Parsed fields (unit, named, or unnamed).
#[derive(Debug)]
pub enum Fields {
    Unit,
    Named(Vec<Field>),
    Unnamed(Vec<Field>)
}

impl Fields {
    /// Returns the number of fields.
    pub fn len(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Named(fields) | Self::Unnamed(fields) => fields.len()
        }
    }

    /// Returns an iterator over fields.
    pub fn iter(&self) -> FieldIter<'_> {
        match self {
            Self::Unit => FieldIter::Empty,
            Self::Named(fields) | Self::Unnamed(fields) => FieldIter::Slice(fields.iter())
        }
    }

    /// Finds a named field by identifier.
    pub fn get_named(&self, name: &str) -> Option<&Field> {
        match self {
            Self::Named(fields) => fields
                .iter()
                .find(|field| field.ident.as_ref().is_some_and(|ident| ident == name)),
            _ => None
        }
    }

    /// Finds an unnamed field by index.
    pub fn get_positional(&self, index: usize) -> Option<&Field> {
        match self {
            Self::Unnamed(fields) => fields.get(index),
            _ => None
        }
    }

    /// Parses fields from syn AST.
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

    /// Finds the first field with a `#[from]` attribute.
    pub fn first_from_field(&self) -> Option<&Field> {
        self.iter().find(|field| field.attrs.from.is_some())
    }

    /// Finds the backtrace field if present.
    pub fn backtrace_field(&self) -> Option<BacktraceField<'_>> {
        self.iter().find_map(|field| {
            field
                .attrs
                .backtrace_kind()
                .map(|kind| BacktraceField::new(field, kind))
        })
    }
}

/// Backtrace field detection mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BacktraceFieldKind {
    Explicit,
    Inferred
}

/// Reference to a field containing backtrace data.
#[derive(Clone, Copy, Debug)]
pub struct BacktraceField<'a> {
    field: &'a Field,
    kind:  BacktraceFieldKind
}

impl<'a> BacktraceField<'a> {
    /// Creates a new backtrace field reference.
    pub fn new(field: &'a Field, kind: BacktraceFieldKind) -> Self {
        Self {
            field,
            kind
        }
    }

    /// Returns the underlying field.
    pub fn field(&self) -> &'a Field {
        self.field
    }

    /// Returns the detection mode.
    pub fn kind(&self) -> BacktraceFieldKind {
        self.kind
    }

    /// Checks if field type can store backtrace.
    pub fn stores_backtrace(&self) -> bool {
        super::utils::is_backtrace_storage(&self.field.ty)
    }

    /// Returns the field index.
    pub fn index(&self) -> usize {
        self.field.index
    }
}

/// Iterator over fields.
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

/// Parsed field with attributes and metadata.
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
    /// Parses field from syn AST.
    pub(crate) fn from_syn(field: &SynField, index: usize, errors: &mut Vec<Error>) -> Self {
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

/// Field-level attributes.
#[derive(Debug, Default)]
pub struct FieldAttrs {
    pub from:           Option<Attribute>,
    pub source:         Option<Attribute>,
    pub backtrace:      Option<Attribute>,
    pub provides:       Vec<ProvideSpec>,
    inferred_source:    bool,
    inferred_backtrace: bool
}

/// Provide attribute specification.
#[derive(Debug)]
pub struct ProvideSpec {
    pub reference: Option<TypePath>,
    pub value:     Option<TypePath>
}

impl FieldAttrs {
    /// Parses field attributes from syn AST.
    pub(crate) fn from_attrs(
        attrs: &[Attribute],
        ident: Option<&Ident>,
        ty: &syn::Type,
        errors: &mut Vec<Error>
    ) -> Self {
        use super::utils::{is_backtrace_type, is_option_type, option_inner_type, path_is};

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
            } else if path_is(attr, "provide") {
                match super::parse_attr::parse_provide_attribute(attr) {
                    Ok(spec) => result.provides.push(spec),
                    Err(err) => errors.push(err)
                }
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

    /// Checks if field has source attribute.
    pub fn has_source(&self) -> bool {
        self.source.is_some() || self.inferred_source
    }

    /// Checks if field has backtrace attribute.
    pub fn has_backtrace(&self) -> bool {
        self.backtrace.is_some() || self.inferred_backtrace
    }

    /// Returns backtrace detection kind.
    pub fn backtrace_kind(&self) -> Option<BacktraceFieldKind> {
        if self.backtrace.is_some() {
            Some(BacktraceFieldKind::Explicit)
        } else if self.inferred_backtrace {
            Some(BacktraceFieldKind::Inferred)
        } else {
            None
        }
    }

    /// Returns source attribute if present.
    pub fn source_attribute(&self) -> Option<&Attribute> {
        self.source.as_ref()
    }

    /// Returns backtrace attribute if present.
    pub fn backtrace_attribute(&self) -> Option<&Attribute> {
        self.backtrace.as_ref()
    }
}

/// Display specification for error messages.
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

/// Format arguments specification.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FormatArgsSpec {
    pub args: Vec<FormatArg>
}

/// Single format argument.
#[allow(dead_code)]
#[derive(Debug)]
pub struct FormatArg {
    pub value: FormatArgValue,
    pub kind:  FormatBindingKind,
    pub span:  Span
}

/// Format argument value (expression or shorthand).
#[allow(dead_code)]
#[derive(Debug)]
pub enum FormatArgValue {
    Expr(Expr),
    Shorthand(FormatArgShorthand)
}

/// Format argument shorthand notation.
#[allow(dead_code)]
#[derive(Debug)]
pub enum FormatArgShorthand {
    Projection(FormatArgProjection)
}

/// Field projection for format arguments.
#[derive(Debug)]
pub struct FormatArgProjection {
    pub segments: Vec<FormatArgProjectionSegment>,
    pub span:     Span
}

/// Projection segment (field access, indexing, method call).
#[derive(Debug)]
pub enum FormatArgProjectionSegment {
    Field(Ident),
    Index { index: usize, span: Span },
    MethodCall(FormatArgProjectionMethodCall)
}

impl FormatArgProjectionSegment {
    /// Returns the span of this segment.
    pub(crate) fn span(&self) -> Span {
        match self {
            Self::Field(ident) => ident.span(),
            Self::Index {
                span, ..
            } => *span,
            Self::MethodCall(call) => call.span
        }
    }
}

/// Method call in projection chain.
#[derive(Debug)]
pub struct FormatArgProjectionMethodCall {
    pub method:    Ident,
    pub turbofish: Option<FormatArgMethodTurbofish>,
    pub args:      Punctuated<Expr, Token![,]>,
    pub span:      Span
}

/// Turbofish syntax for method calls.
#[derive(Debug)]
pub struct FormatArgMethodTurbofish {
    pub colon2_token: Token![::],
    pub generics:     AngleBracketedGenericArguments
}

/// Method call suffix type alias.
pub(crate) type MethodCallSuffix = Option<(
    Option<FormatArgMethodTurbofish>,
    Paren,
    Punctuated<Expr, Token![,]>
)>;

/// Format argument binding kind.
#[allow(dead_code)]
#[derive(Debug)]
pub enum FormatBindingKind {
    Named(Ident),
    Positional(usize),
    Implicit(usize)
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn fields_len_unit() {
        let fields = Fields::Unit;
        assert_eq!(fields.len(), 0);
    }

    #[test]
    fn fields_len_named() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32, y: String } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert_eq!(parsed.len(), 2);
        assert!(errors.is_empty());
    }

    #[test]
    fn fields_len_unnamed() {
        let fields: syn::FieldsUnnamed = parse_quote! { (i32, String) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        assert_eq!(parsed.len(), 2);
        assert!(errors.is_empty());
    }

    #[test]
    fn fields_iter_empty() {
        let fields = Fields::Unit;
        let mut iter = fields.iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn fields_iter_named() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32 } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        let count = parsed.iter().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn fields_get_named_found() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32, y: String } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert!(parsed.get_named("x").is_some());
        assert!(parsed.get_named("y").is_some());
    }

    #[test]
    fn fields_get_named_not_found() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32 } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert!(parsed.get_named("z").is_none());
    }

    #[test]
    fn fields_get_named_on_unit() {
        let fields = Fields::Unit;
        assert!(fields.get_named("x").is_none());
    }

    #[test]
    fn fields_get_named_on_unnamed() {
        let fields: syn::FieldsUnnamed = parse_quote! { (i32) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        assert!(parsed.get_named("x").is_none());
    }

    #[test]
    fn fields_get_positional_found() {
        let fields: syn::FieldsUnnamed = parse_quote! { (i32, String) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        assert!(parsed.get_positional(0).is_some());
        assert!(parsed.get_positional(1).is_some());
    }

    #[test]
    fn fields_get_positional_not_found() {
        let fields: syn::FieldsUnnamed = parse_quote! { (i32) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        assert!(parsed.get_positional(5).is_none());
    }

    #[test]
    fn fields_get_positional_on_unit() {
        let fields = Fields::Unit;
        assert!(fields.get_positional(0).is_none());
    }

    #[test]
    fn fields_get_positional_on_named() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32 } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert!(parsed.get_positional(0).is_none());
    }

    #[test]
    fn fields_first_from_field_found() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, y: String }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert!(parsed.first_from_field().is_some());
    }

    #[test]
    fn fields_first_from_field_not_found() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32, y: String } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert!(parsed.first_from_field().is_none());
    }

    #[test]
    fn fields_backtrace_field_explicit() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[backtrace] bt: std::backtrace::Backtrace }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        let bt_field = parsed.backtrace_field();
        assert!(bt_field.is_some());
        assert_eq!(bt_field.unwrap().kind(), BacktraceFieldKind::Explicit);
    }

    #[test]
    fn fields_backtrace_field_inferred() {
        let fields: syn::FieldsUnnamed = parse_quote! { (std::backtrace::Backtrace) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        let bt_field = parsed.backtrace_field();
        assert!(bt_field.is_some());
        assert_eq!(bt_field.unwrap().kind(), BacktraceFieldKind::Inferred);
    }

    #[test]
    fn fields_backtrace_field_not_found() {
        let fields: syn::FieldsNamed = parse_quote! { { x: i32 } };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        assert!(parsed.backtrace_field().is_none());
    }

    #[test]
    fn backtrace_field_methods() {
        let field: SynField = parse_quote! { #[backtrace] bt: std::backtrace::Backtrace };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        let bt = BacktraceField::new(&parsed, BacktraceFieldKind::Explicit);

        assert_eq!(bt.field().index, 0);
        assert_eq!(bt.kind(), BacktraceFieldKind::Explicit);
        assert!(bt.stores_backtrace());
        assert_eq!(bt.index(), 0);
    }

    #[test]
    fn backtrace_field_stores_option_backtrace() {
        let field: SynField = parse_quote! { bt: Option<std::backtrace::Backtrace> };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        let bt = BacktraceField::new(&parsed, BacktraceFieldKind::Inferred);

        assert!(bt.stores_backtrace());
    }

    #[test]
    fn field_attrs_has_source_explicit() {
        let field: SynField = parse_quote! { #[source] e: io::Error };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.has_source());
        assert!(parsed.attrs.source_attribute().is_some());
    }

    #[test]
    fn field_attrs_has_source_inferred() {
        let field: SynField = parse_quote! { source: io::Error };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.has_source());
    }

    #[test]
    fn field_attrs_has_source_from() {
        let field: SynField = parse_quote! { #[from] e: io::Error };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.has_source());
    }

    #[test]
    fn field_attrs_has_backtrace_explicit() {
        let field: SynField = parse_quote! { #[backtrace] bt: Backtrace };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.has_backtrace());
        assert!(parsed.attrs.backtrace_attribute().is_some());
    }

    #[test]
    fn field_attrs_has_backtrace_inferred() {
        let field: SynField = parse_quote! { bt: std::backtrace::Backtrace };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.has_backtrace());
    }

    #[test]
    fn field_attrs_has_backtrace_option() {
        let field: SynField = parse_quote! { bt: Option<Backtrace> };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.has_backtrace());
    }

    #[test]
    fn field_attrs_backtrace_kind_none() {
        let field: SynField = parse_quote! { x: i32 };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert!(parsed.attrs.backtrace_kind().is_none());
    }

    #[test]
    fn field_attrs_duplicate_from() {
        let field: SynField = parse_quote! { #[from] #[from] e: io::Error };
        let mut errors = Vec::new();
        let _ = Field::from_syn(&field, 0, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn field_attrs_duplicate_source() {
        let field: SynField = parse_quote! { #[source] #[source] e: io::Error };
        let mut errors = Vec::new();
        let _ = Field::from_syn(&field, 0, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn field_attrs_duplicate_backtrace() {
        let field: SynField = parse_quote! { #[backtrace] #[backtrace] bt: Backtrace };
        let mut errors = Vec::new();
        let _ = Field::from_syn(&field, 0, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn field_attrs_provide() {
        let field: SynField = parse_quote! { #[provide(ref = ErrorCode)] e: io::Error };
        let mut errors = Vec::new();
        let parsed = Field::from_syn(&field, 0, &mut errors);
        assert_eq!(parsed.attrs.provides.len(), 1);
        assert!(errors.is_empty());
    }

    #[test]
    fn format_arg_projection_segment_span_field() {
        let ident: Ident = parse_quote! { foo };
        let segment = FormatArgProjectionSegment::Field(ident);
        let _ = segment.span();
    }

    #[test]
    fn format_arg_projection_segment_span_index() {
        let segment = FormatArgProjectionSegment::Index {
            index: 0,
            span:  Span::call_site()
        };
        let _ = segment.span();
    }

    #[test]
    fn format_arg_projection_segment_span_method_call() {
        let call = FormatArgProjectionMethodCall {
            method:    parse_quote! { to_string },
            turbofish: None,
            args:      Punctuated::new(),
            span:      Span::call_site()
        };
        let segment = FormatArgProjectionSegment::MethodCall(call);
        let _ = segment.span();
    }
}
