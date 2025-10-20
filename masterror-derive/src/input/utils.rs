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
