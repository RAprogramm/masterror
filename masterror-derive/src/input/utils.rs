// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Utility functions for input parsing and validation.
//!
//! Provides type checking, validation, and error collection utilities
//! used throughout the input parsing process.

use proc_macro2::Span;
use syn::{Attribute, Error, GenericArgument};

use super::types::{DisplaySpec, Field, Fields};
use crate::template_support::TemplateIdentifierSpec;

/// Validates #[from] attribute usage across fields.
///
/// Ensures only one #[from] field exists and validates companion fields
/// (source and backtrace fields must be compatible).
pub(crate) fn validate_from_usage(
    fields: &Fields,
    display: &DisplaySpec,
    errors: &mut Vec<Error>
) {
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

/// Validates #[backtrace] attribute usage across fields.
///
/// Ensures only one backtrace field exists and validates field types.
pub(crate) fn validate_backtrace_usage(fields: &Fields, errors: &mut Vec<Error>) {
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

/// Validates backtrace field type is correct.
fn validate_backtrace_field_type(field: &Field, errors: &mut Vec<Error>) {
    let Some(attr) = field.attrs.backtrace_attribute() else {
        return;
    };

    if is_backtrace_storage(&field.ty) {
        return;
    }

    if field.attrs.has_source() {
        return;
    }

    errors.push(Error::new_spanned(
        attr,
        "fields with #[backtrace] must be std::backtrace::Backtrace or Option<std::backtrace::Backtrace>"
    ));
}

/// Validates transparent attribute requires exactly one field.
pub(crate) fn validate_transparent(
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

/// Checks if attribute path matches expected identifier.
pub(crate) fn path_is(attr: &Attribute, expected: &str) -> bool {
    attr.path().is_ident(expected)
}

/// Combines multiple errors into a single error.
pub(crate) fn collect_errors(errors: Vec<Error>) -> Error {
    let mut iter = errors.into_iter();
    let mut root = iter
        .next()
        .unwrap_or_else(|| Error::new(Span::call_site(), "unexpected error"));
    for err in iter {
        root.combine(err);
    }
    root
}

/// Checks if type is Option<T>.
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

/// Extracts inner type from Option<T>.
pub(crate) fn option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
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

/// Checks if type is Arc<T>.
pub(crate) fn is_arc_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    if path.qself.is_some() {
        return false;
    }
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Arc")
}

/// Checks if type is Backtrace.
pub(crate) fn is_backtrace_type(ty: &syn::Type) -> bool {
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

/// Checks if type can store backtrace (Backtrace or Option<Backtrace>).
pub(crate) fn is_backtrace_storage(ty: &syn::Type) -> bool {
    if is_option_type(ty) {
        option_inner_type(ty).is_some_and(is_backtrace_type)
    } else {
        is_backtrace_type(ty)
    }
}

/// Creates error for unknown template placeholder.
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

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    fn make_template() -> DisplaySpec {
        let lit: syn::LitStr = parse_quote! { "error message" };
        let template = crate::template_support::parse_display_template(lit).unwrap();
        DisplaySpec::Template(template)
    }

    #[test]
    fn validate_from_usage_multiple_from_fields() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, #[from] y: io::Error }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let display = make_template();
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_from_usage_unexpected_companions() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, y: String }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let display = make_template();
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_from_usage_source_companion_non_option() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, source: io::Error }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let display = make_template();
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_from_usage_source_companion_option_ok() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, source: Option<io::Error> }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let display = make_template();
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_from_usage_backtrace_companion_ok() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, #[backtrace] bt: Backtrace }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let display = make_template();
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_from_usage_transparent_not_single() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let attr: syn::Attribute = parse_quote! { #[error(transparent)] };
        let display = DisplaySpec::Transparent {
            attribute: Box::new(attr)
        };
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(errors.is_empty()); // Single field is OK
    }

    #[test]
    fn validate_backtrace_usage_single() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[backtrace] bt: Backtrace }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        validate_backtrace_usage(&parsed, &mut errors);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_backtrace_usage_multiple() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[backtrace] bt1: Backtrace, #[backtrace] bt2: Backtrace }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        validate_backtrace_usage(&parsed, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_backtrace_usage_invalid_type() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[backtrace] bt: String }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        validate_backtrace_usage(&parsed, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_backtrace_usage_source_ok() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[backtrace] #[source] e: io::Error }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        validate_backtrace_usage(&parsed, &mut errors);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_transparent_single_field() {
        let fields: syn::FieldsUnnamed = parse_quote! { (io::Error) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);

        let attr: syn::Attribute = parse_quote! { #[error(transparent)] };
        let display = DisplaySpec::Transparent {
            attribute: Box::new(attr)
        };
        validate_transparent(&parsed, &display, &mut errors, None);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_transparent_multiple_fields_struct() {
        let fields: syn::FieldsUnnamed = parse_quote! { (io::Error, String) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        errors.clear();

        let attr: syn::Attribute = parse_quote! { #[error(transparent)] };
        let display = DisplaySpec::Transparent {
            attribute: Box::new(attr)
        };
        validate_transparent(&parsed, &display, &mut errors, None);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_transparent_multiple_fields_variant() {
        let fields: syn::FieldsUnnamed = parse_quote! { (io::Error, String) };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Unnamed(fields), &mut errors);
        errors.clear();

        let attr: syn::Attribute = parse_quote! { #[error(transparent)] };
        let display = DisplaySpec::Transparent {
            attribute: Box::new(attr)
        };
        let variant: syn::Variant = parse_quote! { Foo(io::Error, String) };
        validate_transparent(&parsed, &display, &mut errors, Some(&variant));
        assert!(!errors.is_empty());
    }

    #[test]
    fn path_is_match() {
        let attr: syn::Attribute = parse_quote! { #[from] };
        assert!(path_is(&attr, "from"));
    }

    #[test]
    fn path_is_no_match() {
        let attr: syn::Attribute = parse_quote! { #[source] };
        assert!(!path_is(&attr, "from"));
    }

    #[test]
    fn collect_errors_single() {
        let err = Error::new(Span::call_site(), "test error");
        let result = collect_errors(vec![err]);
        assert!(result.to_string().contains("test error"));
    }

    #[test]
    fn collect_errors_multiple() {
        let err1 = Error::new(Span::call_site(), "error 1");
        let err2 = Error::new(Span::call_site(), "error 2");
        let result = collect_errors(vec![err1, err2]);
        let s = result.to_string();
        assert!(s.contains("error 1") || s.contains("error 2"));
    }

    #[test]
    fn collect_errors_empty() {
        let result = collect_errors(vec![]);
        assert!(result.to_string().contains("unexpected error"));
    }

    #[test]
    fn is_option_type_true() {
        let ty: syn::Type = parse_quote! { Option<i32> };
        assert!(is_option_type(&ty));
    }

    #[test]
    fn is_option_type_false() {
        let ty: syn::Type = parse_quote! { i32 };
        assert!(!is_option_type(&ty));
    }

    #[test]
    fn is_option_type_with_qself() {
        let ty: syn::Type = parse_quote! { <Self as Foo>::Option };
        assert!(!is_option_type(&ty));
    }

    #[test]
    fn option_inner_type_some() {
        let ty: syn::Type = parse_quote! { Option<i32> };
        assert!(option_inner_type(&ty).is_some());
    }

    #[test]
    fn option_inner_type_none() {
        let ty: syn::Type = parse_quote! { i32 };
        assert!(option_inner_type(&ty).is_none());
    }

    #[test]
    fn option_inner_type_with_qself() {
        let ty: syn::Type = parse_quote! { <Self as Foo>::Option };
        assert!(option_inner_type(&ty).is_none());
    }

    #[test]
    fn option_inner_type_no_angle_brackets() {
        let ty: syn::Type = parse_quote! { Option };
        assert!(option_inner_type(&ty).is_none());
    }

    #[test]
    fn is_arc_type_true() {
        let ty: syn::Type = parse_quote! { Arc<String> };
        assert!(is_arc_type(&ty));
    }

    #[test]
    fn is_arc_type_false() {
        let ty: syn::Type = parse_quote! { Box<String> };
        assert!(!is_arc_type(&ty));
    }

    #[test]
    fn is_arc_type_with_qself() {
        let ty: syn::Type = parse_quote! { <Self as Foo>::Arc };
        assert!(!is_arc_type(&ty));
    }

    #[test]
    fn is_backtrace_type_true() {
        let ty: syn::Type = parse_quote! { Backtrace };
        assert!(is_backtrace_type(&ty));
    }

    #[test]
    fn is_backtrace_type_false() {
        let ty: syn::Type = parse_quote! { String };
        assert!(!is_backtrace_type(&ty));
    }

    #[test]
    fn is_backtrace_type_with_qself() {
        let ty: syn::Type = parse_quote! { <Self as Foo>::Backtrace };
        assert!(!is_backtrace_type(&ty));
    }

    #[test]
    fn is_backtrace_type_with_args() {
        let ty: syn::Type = parse_quote! { Backtrace<'a> };
        assert!(!is_backtrace_type(&ty));
    }

    #[test]
    fn is_backtrace_storage_backtrace() {
        let ty: syn::Type = parse_quote! { Backtrace };
        assert!(is_backtrace_storage(&ty));
    }

    #[test]
    fn is_backtrace_storage_option_backtrace() {
        let ty: syn::Type = parse_quote! { Option<Backtrace> };
        assert!(is_backtrace_storage(&ty));
    }

    #[test]
    fn is_backtrace_storage_false() {
        let ty: syn::Type = parse_quote! { String };
        assert!(!is_backtrace_storage(&ty));
    }

    #[test]
    fn is_backtrace_storage_option_string() {
        let ty: syn::Type = parse_quote! { Option<String> };
        assert!(!is_backtrace_storage(&ty));
    }

    #[test]
    fn placeholder_error_named() {
        let ident = TemplateIdentifierSpec::Named("foo".to_string());
        let err = placeholder_error(Span::call_site(), &ident);
        assert!(err.to_string().contains("unknown field `foo`"));
    }

    #[test]
    fn placeholder_error_positional() {
        let ident = TemplateIdentifierSpec::Positional(0);
        let err = placeholder_error(Span::call_site(), &ident);
        assert!(err.to_string().contains("field `0` is not available"));
    }

    #[test]
    fn placeholder_error_implicit() {
        let ident = TemplateIdentifierSpec::Implicit(1);
        let err = placeholder_error(Span::call_site(), &ident);
        assert!(err.to_string().contains("field `1` is not available"));
    }

    #[test]
    fn validate_from_usage_source_companion_implicit_source_without_attr() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, #[source] y: io::Error }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let display = make_template();
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_backtrace_usage_single_with_extra_field() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[backtrace] bt1: Backtrace, other: String }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        validate_backtrace_usage(&parsed, &mut errors);
        assert!(errors.is_empty());
    }

    #[test]
    fn is_arc_type_non_path_type() {
        let ty: syn::Type = parse_quote! { [u8; 32] };
        assert!(!is_arc_type(&ty));
    }

    #[test]
    fn is_arc_type_tuple() {
        let ty: syn::Type = parse_quote! { (u8, u8) };
        assert!(!is_arc_type(&ty));
    }

    #[test]
    fn is_backtrace_type_non_path() {
        let ty: syn::Type = parse_quote! { [u8; 32] };
        assert!(!is_backtrace_type(&ty));
    }

    #[test]
    fn is_backtrace_type_tuple() {
        let ty: syn::Type = parse_quote! { (u8, u8) };
        assert!(!is_backtrace_type(&ty));
    }

    #[test]
    fn is_backtrace_type_empty_segments() {
        let ty: syn::Type = parse_quote! { String };
        assert!(!is_backtrace_type(&ty));
    }

    #[test]
    fn option_inner_type_non_type_arg() {
        let ty: syn::Type = parse_quote! { Option<'a> };
        assert!(option_inner_type(&ty).is_none());
    }

    #[test]
    fn option_inner_type_lifetime_arg() {
        let ty: syn::Type = parse_quote! { std::option::Option<'static> };
        assert!(option_inner_type(&ty).is_none());
    }

    #[test]
    fn validate_from_usage_transparent_multiple_fields() {
        let fields: syn::FieldsNamed = parse_quote! {
            { #[from] x: io::Error, y: String }
        };
        let mut errors = Vec::new();
        let parsed = Fields::from_syn(&syn::Fields::Named(fields), &mut errors);
        errors.clear();

        let attr: syn::Attribute = parse_quote! { #[error(transparent)] };
        let display = DisplaySpec::Transparent {
            attribute: Box::new(attr)
        };
        validate_from_usage(&parsed, &display, &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn is_arc_type_no_segments() {
        let ty: syn::Type = parse_quote! { fn() };
        assert!(!is_arc_type(&ty));
    }

    #[test]
    fn is_option_type_empty_path() {
        let ty: syn::Type = parse_quote! { Vec<i32> };
        assert!(!is_option_type(&ty));
    }
}
