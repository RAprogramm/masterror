// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Attribute parsing for error definitions.
//!
//! Handles parsing of `#[error(...)]`, `#[app_error(...)]`, and
//! `#[masterror(...)]` attributes from derive macro input.

use proc_macro2::Span;
use syn::{
    Attribute, Error, Expr, ExprPath, Ident, LitBool, LitStr, Token, TypePath,
    ext::IdentExt,
    parse::{ParseBuffer, ParseStream},
    spanned::Spanned
};

use super::{
    parse_format::parse_format_args,
    types::{
        AppErrorSpec, DisplaySpec, FieldRedactionKind, FieldRedactionSpec, FormatBindingKind,
        MasterrorSpec, ProvideSpec, RedactSpec
    },
    utils::path_is
};
use crate::template_support::parse_display_template;

/// Extracts masterror specification from attributes.
pub(crate) fn extract_masterror_spec(
    attrs: &[Attribute],
    errors: &mut Vec<Error>
) -> Result<Option<MasterrorSpec>, ()> {
    let mut spec = None;
    let mut had_error = false;
    for attr in attrs {
        if !path_is(attr, "masterror") {
            continue;
        }
        if spec.is_some() {
            errors.push(Error::new_spanned(
                attr,
                "duplicate #[masterror(...)] attribute"
            ));
            had_error = true;
            continue;
        }
        match parse_masterror_attribute(attr) {
            Ok(parsed) => spec = Some(parsed),
            Err(err) => {
                errors.push(err);
                had_error = true;
            }
        }
    }
    if had_error { Err(()) } else { Ok(spec) }
}

/// Extracts app_error specification from attributes.
pub(crate) fn extract_app_error_spec(
    attrs: &[Attribute],
    errors: &mut Vec<Error>
) -> Result<Option<AppErrorSpec>, ()> {
    let mut spec = None;
    let mut had_error = false;
    for attr in attrs {
        if !path_is(attr, "app_error") {
            continue;
        }
        if spec.is_some() {
            errors.push(Error::new_spanned(
                attr,
                "duplicate #[app_error(...)] attribute"
            ));
            had_error = true;
            continue;
        }
        match parse_app_error_attribute(attr) {
            Ok(parsed) => spec = Some(parsed),
            Err(err) => {
                errors.push(err);
                had_error = true;
            }
        }
    }
    if had_error { Err(()) } else { Ok(spec) }
}

/// Extracts display specification from error attributes.
pub(crate) fn extract_display_spec(
    attrs: &[Attribute],
    missing_span: Span,
    errors: &mut Vec<Error>
) -> Result<DisplaySpec, ()> {
    let mut display = None;
    let mut saw_error_attribute = false;
    for attr in attrs {
        if !path_is(attr, "error") {
            continue;
        }
        saw_error_attribute = true;
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
            if !saw_error_attribute {
                errors.push(Error::new(missing_span, "missing #[error(...)] attribute"));
            }
            Err(())
        }
    }
}

/// Parses #[app_error(...)] attribute contents.
fn parse_app_error_attribute(attr: &Attribute) -> Result<AppErrorSpec, Error> {
    attr.parse_args_with(|input: ParseStream| {
        let mut kind = None;
        let mut code = None;
        let mut expose_message = false;
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let name = ident.to_string();
            match name.as_str() {
                "kind" => {
                    if kind.is_some() {
                        return Err(Error::new(ident.span(), "duplicate kind specification"));
                    }
                    input.parse::<Token![=]>()?;
                    let value: ExprPath = input.parse()?;
                    kind = Some(value);
                }
                "code" => {
                    if code.is_some() {
                        return Err(Error::new(ident.span(), "duplicate code specification"));
                    }
                    input.parse::<Token![=]>()?;
                    let value: ExprPath = input.parse()?;
                    code = Some(value);
                }
                "message" => {
                    if expose_message {
                        return Err(Error::new(ident.span(), "duplicate message flag"));
                    }
                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                        let value: LitBool = input.parse()?;
                        expose_message = value.value;
                    } else {
                        expose_message = true;
                    }
                }
                other => {
                    return Err(Error::new(
                        ident.span(),
                        format!("unknown #[app_error] option `{}`", other)
                    ));
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "expected `,` or end of input in #[app_error(...)]"
                ));
            }
        }
        let kind = match kind {
            Some(kind) => kind,
            None => {
                return Err(Error::new(
                    attr.span(),
                    "missing `kind = ...` in #[app_error(...)]"
                ));
            }
        };
        Ok(AppErrorSpec {
            kind,
            code,
            expose_message,
            attribute_span: attr.span()
        })
    })
}

/// Parses #[masterror(...)] attribute contents.
fn parse_masterror_attribute(attr: &Attribute) -> Result<MasterrorSpec, Error> {
    attr.parse_args_with(|input: ParseStream| {
        let mut code = None;
        let mut category = None;
        let mut expose_message = false;
        let mut redact = RedactSpec::default();
        let mut seen_redact = false;
        let mut telemetry = None;
        let mut map_grpc = None;
        let mut map_problem = None;
        while !input.is_empty() {
            let ident: Ident = input.call(Ident::parse_any)?;
            match ident.to_string().as_str() {
                "code" => {
                    if code.is_some() {
                        return Err(Error::new(ident.span(), "duplicate code specification"));
                    }
                    input.parse::<Token![=]>()?;
                    let value: Expr = input.parse()?;
                    code = Some(value);
                }
                "category" => {
                    if category.is_some() {
                        return Err(Error::new(ident.span(), "duplicate category specification"));
                    }
                    input.parse::<Token![=]>()?;
                    let value: ExprPath = input.parse()?;
                    category = Some(value);
                }
                "message" => {
                    if expose_message {
                        return Err(Error::new(ident.span(), "duplicate message flag"));
                    }
                    expose_message = parse_flag_value(input)?;
                }
                "redact" => {
                    if seen_redact {
                        return Err(Error::new(ident.span(), "duplicate redact(...) block"));
                    }
                    redact = parse_redact_block(input, ident.span())?;
                    seen_redact = true;
                }
                "telemetry" => {
                    if telemetry.is_some() {
                        return Err(Error::new(ident.span(), "duplicate telemetry(...) block"));
                    }
                    telemetry = Some(parse_telemetry_block(input, ident.span())?);
                }
                "map" => {
                    input.parse::<Token![.]>()?;
                    let sub: Ident = input.call(Ident::parse_any)?;
                    match sub.to_string().as_str() {
                        "grpc" => {
                            if map_grpc.is_some() {
                                return Err(Error::new(
                                    sub.span(),
                                    "duplicate map.grpc specification"
                                ));
                            }
                            input.parse::<Token![=]>()?;
                            let value: Expr = input.parse()?;
                            map_grpc = Some(value);
                        }
                        "problem" => {
                            if map_problem.is_some() {
                                return Err(Error::new(
                                    sub.span(),
                                    "duplicate map.problem specification"
                                ));
                            }
                            input.parse::<Token![=]>()?;
                            let value: Expr = input.parse()?;
                            map_problem = Some(value);
                        }
                        other => {
                            return Err(Error::new(
                                sub.span(),
                                format!("unknown #[masterror] mapping `map.{other}`")
                            ));
                        }
                    }
                }
                other => {
                    return Err(Error::new(
                        ident.span(),
                        format!("unknown #[masterror] option `{other}`")
                    ));
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "expected `,` or end of input in #[masterror(...)]"
                ));
            }
        }
        let code = match code {
            Some(value) => value,
            None => {
                return Err(Error::new(
                    attr.span(),
                    "missing `code = ...` in #[masterror(...)]"
                ));
            }
        };
        let category = match category {
            Some(value) => value,
            None => {
                return Err(Error::new(
                    attr.span(),
                    "missing `category = ...` in #[masterror(...)]"
                ));
            }
        };
        Ok(MasterrorSpec {
            code,
            category,
            expose_message,
            redact,
            telemetry: telemetry.unwrap_or_default(),
            map_grpc,
            map_problem,
            attribute_span: attr.span()
        })
    })
}

/// Parses boolean flag value (either explicit or implicit true).
fn parse_flag_value(input: ParseStream) -> Result<bool, Error> {
    if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let value: LitBool = input.parse()?;
        Ok(value.value)
    } else {
        Ok(true)
    }
}

/// Parses redact(...) block contents.
fn parse_redact_block(input: ParseStream, span: Span) -> Result<RedactSpec, Error> {
    let content;
    syn::parenthesized!(content in input);
    if content.is_empty() {
        return Err(Error::new(span, "redact(...) requires at least one option"));
    }
    let mut spec = RedactSpec::default();
    while !content.is_empty() {
        let ident: Ident = content.call(Ident::parse_any)?;
        match ident.to_string().as_str() {
            "message" => {
                if spec.message {
                    return Err(Error::new(ident.span(), "duplicate redact(message) option"));
                }
                if content.peek(Token![=]) {
                    content.parse::<Token![=]>()?;
                    let value: LitBool = content.parse()?;
                    spec.message = value.value;
                } else {
                    spec.message = true;
                }
            }
            "fields" => {
                if !spec.fields.is_empty() {
                    return Err(Error::new(
                        ident.span(),
                        "duplicate redact(fields(...)) option"
                    ));
                }
                spec.fields = parse_redact_fields(&content, ident.span())?;
            }
            other => {
                return Err(Error::new(
                    ident.span(),
                    format!("unknown redact option `{other}`")
                ));
            }
        }
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        } else if !content.is_empty() {
            return Err(Error::new(
                content.span(),
                "expected `,` or end of input in redact(...)"
            ));
        }
    }
    Ok(spec)
}

/// Parses redact(fields(...)) contents.
fn parse_redact_fields(
    content: &ParseBuffer<'_>,
    span: Span
) -> Result<Vec<FieldRedactionSpec>, Error> {
    let inner;
    syn::parenthesized!(inner in *content);
    if inner.is_empty() {
        return Err(Error::new(
            span,
            "redact(fields(...)) requires at least one field"
        ));
    }
    let mut fields = Vec::new();
    while !inner.is_empty() {
        let name: LitStr = inner.parse()?;
        let policy = if inner.peek(Token![=]) {
            inner.parse::<Token![=]>()?;
            let ident: Ident = inner.call(Ident::parse_any)?;
            match ident.to_string().to_ascii_lowercase().as_str() {
                "none" => FieldRedactionKind::None,
                "redact" => FieldRedactionKind::Redact,
                "hash" => FieldRedactionKind::Hash,
                "last4" | "last_four" => FieldRedactionKind::Last4,
                other => {
                    return Err(Error::new(
                        ident.span(),
                        format!("unknown redact policy `{other}` in fields(...)")
                    ));
                }
            }
        } else {
            FieldRedactionKind::Redact
        };
        fields.push(FieldRedactionSpec {
            name,
            policy
        });
        if inner.peek(Token![,]) {
            inner.parse::<Token![,]>()?;
        } else if !inner.is_empty() {
            return Err(Error::new(
                inner.span(),
                "expected `,` or end of input in redact(fields(...))"
            ));
        }
    }
    Ok(fields)
}

/// Parses telemetry(...) block contents.
fn parse_telemetry_block(input: ParseStream, span: Span) -> Result<Vec<Expr>, Error> {
    let content;
    syn::parenthesized!(content in input);
    let mut entries = Vec::new();
    while !content.is_empty() {
        let expr: Expr = content.parse()?;
        entries.push(expr);
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            if content.is_empty() {
                return Err(Error::new(
                    span,
                    "expected expression after comma in telemetry(...)"
                ));
            }
        } else if !content.is_empty() {
            return Err(Error::new(
                content.span(),
                "expected `,` or end of input in telemetry(...)"
            ));
        }
    }
    Ok(entries)
}

/// Parses #[error(...)] attribute contents.
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

/// Parses #[provide(...)] attribute contents.
pub(crate) fn parse_provide_attribute(attr: &Attribute) -> Result<ProvideSpec, Error> {
    attr.parse_args_with(|input: ParseStream| {
        let mut reference = None;
        let mut value = None;
        while !input.is_empty() {
            let ident: Ident = input.call(Ident::parse_any)?;
            let name = ident.to_string();
            match name.as_str() {
                "ref" => {
                    if reference.is_some() {
                        return Err(Error::new(ident.span(), "duplicate `ref` specification"));
                    }
                    input.parse::<Token![=]>()?;
                    let ty: TypePath = input.parse()?;
                    reference = Some(ty);
                }
                "value" => {
                    if value.is_some() {
                        return Err(Error::new(ident.span(), "duplicate `value` specification"));
                    }
                    input.parse::<Token![=]>()?;
                    let ty: TypePath = input.parse()?;
                    value = Some(ty);
                }
                other => {
                    return Err(Error::new(
                        ident.span(),
                        format!("unknown #[provide] option `{}`", other)
                    ));
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    "expected `,` or end of input in #[provide(...)]"
                ));
            }
        }
        if reference.is_none() && value.is_none() {
            return Err(Error::new(
                attr.span(),
                "`#[provide]` requires at least one of `ref = ...` or `value = ...`"
            ));
        }
        Ok(ProvideSpec {
            reference,
            value
        })
    })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn extract_masterror_spec_none() {
        let attrs: Vec<Attribute> = vec![];
        let mut errors = Vec::new();
        let result = extract_masterror_spec(&attrs, &mut errors);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn extract_masterror_spec_duplicate() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[masterror(code = 1, category = Cat::A)] },
            parse_quote! { #[masterror(code = 2, category = Cat::B)] },
        ];
        let mut errors = Vec::new();
        let result = extract_masterror_spec(&attrs, &mut errors);
        assert!(result.is_err());
        assert!(!errors.is_empty());
    }

    #[test]
    fn extract_app_error_spec_none() {
        let attrs: Vec<Attribute> = vec![];
        let mut errors = Vec::new();
        let result = extract_app_error_spec(&attrs, &mut errors);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn extract_app_error_spec_duplicate() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[app_error(kind = Kind::A)] },
            parse_quote! { #[app_error(kind = Kind::B)] },
        ];
        let mut errors = Vec::new();
        let result = extract_app_error_spec(&attrs, &mut errors);
        assert!(result.is_err());
        assert!(!errors.is_empty());
    }

    #[test]
    fn extract_display_spec_missing() {
        let attrs: Vec<Attribute> = vec![];
        let mut errors = Vec::new();
        let result = extract_display_spec(&attrs, Span::call_site(), &mut errors);
        assert!(result.is_err());
        assert!(!errors.is_empty());
    }

    #[test]
    fn extract_display_spec_duplicate() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[error("error 1")] },
            parse_quote! { #[error("error 2")] },
        ];
        let mut errors = Vec::new();
        let result = extract_display_spec(&attrs, Span::call_site(), &mut errors);
        assert!(result.is_ok());
        assert!(!errors.is_empty());
    }

    #[test]
    fn parse_provide_attribute_ref_only() {
        let attr: Attribute = parse_quote! { #[provide(ref = ErrorCode)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.reference.is_some());
        assert!(spec.value.is_none());
    }

    #[test]
    fn parse_provide_attribute_value_only() {
        let attr: Attribute = parse_quote! { #[provide(value = ErrorCode)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.reference.is_none());
        assert!(spec.value.is_some());
    }

    #[test]
    fn parse_provide_attribute_both() {
        let attr: Attribute = parse_quote! { #[provide(ref = Code, value = Code)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert!(spec.reference.is_some());
        assert!(spec.value.is_some());
    }

    #[test]
    fn parse_provide_attribute_empty() {
        let attr: Attribute = parse_quote! { #[provide()] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_provide_attribute_duplicate_ref() {
        let attr: Attribute = parse_quote! { #[provide(ref = A, ref = B)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_provide_attribute_duplicate_value() {
        let attr: Attribute = parse_quote! { #[provide(value = A, value = B)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_provide_attribute_unknown_option() {
        let attr: Attribute = parse_quote! { #[provide(foo = Bar)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_app_error_attribute_basic() {
        let attr: Attribute = parse_quote! { #[app_error(kind = ErrorKind::Internal)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_app_error_attribute_with_code() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, code = C)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_app_error_attribute_with_message_flag() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, message)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().expose_message);
    }

    #[test]
    fn parse_app_error_attribute_with_message_true() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, message = true)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().expose_message);
    }

    #[test]
    fn parse_app_error_attribute_with_message_false() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, message = false)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_ok());
        assert!(!result.unwrap().expose_message);
    }

    #[test]
    fn parse_app_error_attribute_duplicate_kind() {
        let attr: Attribute = parse_quote! { #[app_error(kind = A, kind = B)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_app_error_attribute_duplicate_code() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, code = A, code = B)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_app_error_attribute_duplicate_message() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, message, message)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_app_error_attribute_missing_kind() {
        let attr: Attribute = parse_quote! { #[app_error(code = C)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_app_error_attribute_unknown_option() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K, foo = bar)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_app_error_attribute_missing_comma() {
        let attr: Attribute = parse_quote! { #[app_error(kind = K code = C)] };
        let result = parse_app_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_basic() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = Cat::A)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_masterror_attribute_with_message() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, message)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().expose_message);
    }

    #[test]
    fn parse_masterror_attribute_with_message_false() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, message = false)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(!result.unwrap().expose_message);
    }

    #[test]
    fn parse_masterror_attribute_with_redact() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(message))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().redact.message);
    }

    #[test]
    fn parse_masterror_attribute_with_telemetry() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, telemetry(x, y))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().telemetry.len(), 2);
    }

    #[test]
    fn parse_masterror_attribute_with_map_grpc() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, map.grpc = Code::INTERNAL)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().map_grpc.is_some());
    }

    #[test]
    fn parse_masterror_attribute_with_map_problem() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, map.problem = "error")] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().map_problem.is_some());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_code() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, code = 2, category = C)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_category() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = A, category = B)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_message() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, message, message)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_redact() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, redact(message), redact(message))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_telemetry() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, telemetry(x), telemetry(y))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_map_grpc() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, map.grpc = A, map.grpc = B)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_duplicate_map_problem() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, map.problem = A, map.problem = B)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_unknown_map() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, map.foo = bar)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_unknown_option() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, foo = bar)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_missing_code() {
        let attr: Attribute = parse_quote! { #[masterror(category = C)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_missing_category() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_masterror_attribute_missing_comma() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1 category = C)] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_block_empty() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, redact())] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_block_message_only() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(message))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().redact.message);
    }

    #[test]
    fn parse_redact_block_message_explicit_true() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(message = true))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(result.unwrap().redact.message);
    }

    #[test]
    fn parse_redact_block_message_explicit_false() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(message = false))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert!(!result.unwrap().redact.message);
    }

    #[test]
    fn parse_redact_block_duplicate_message() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(message, message))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_block_fields_single() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("password")))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().redact.fields.len(), 1);
    }

    #[test]
    fn parse_redact_block_fields_multiple() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, redact(fields("password", "token")))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().redact.fields.len(), 2);
    }

    #[test]
    fn parse_redact_block_duplicate_fields() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, redact(fields("a"), fields("b")))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_block_unknown_option() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, redact(foo))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_block_missing_comma() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(message fields("a")))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_fields_empty() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields()))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_fields_policy_none() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("f" = none)))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_redact_fields_policy_redact() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("f" = redact)))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_redact_fields_policy_hash() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("f" = hash)))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_redact_fields_policy_last4() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("f" = last4)))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_redact_fields_policy_last_four() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("f" = last_four)))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_redact_fields_unknown_policy() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("f" = unknown)))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_redact_fields_missing_comma() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, redact(fields("a" "b")))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_telemetry_block_single() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, telemetry(x))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().telemetry.len(), 1);
    }

    #[test]
    fn parse_telemetry_block_multiple() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, telemetry(x, y, z))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().telemetry.len(), 3);
    }

    #[test]
    fn parse_telemetry_block_trailing_comma_error() {
        let attr: Attribute = parse_quote! { #[masterror(code = 1, category = C, telemetry(x,))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_telemetry_block_missing_comma() {
        let attr: Attribute =
            parse_quote! { #[masterror(code = 1, category = C, telemetry(x y))] };
        let result = parse_masterror_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_attribute_template() {
        let attr: Attribute = parse_quote! { #[error("error message")] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_template_with_args() {
        let attr: Attribute = parse_quote! { #[error("error: {}", msg)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_transparent() {
        let attr: Attribute = parse_quote! { #[error(transparent)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_transparent_with_args() {
        let attr: Attribute = parse_quote! { #[error(transparent, foo)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_attribute_fmt() {
        let attr: Attribute = parse_quote! { #[error(fmt = custom_formatter)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_fmt_with_args() {
        let attr: Attribute = parse_quote! { #[error(fmt = formatter, arg)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_fmt_duplicate() {
        let attr: Attribute = parse_quote! { #[error(fmt = f, fmt = g)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn parse_error_attribute_fmt_with_args_ok() {
        let attr: Attribute = parse_quote! { #[error(fmt = f, arg, extra)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_with_args() {
        let attr: Attribute = parse_quote! { #[error("msg", foo, bar, baz)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_attribute_invalid_start() {
        let attr: Attribute = parse_quote! { #[error(123)] };
        let result = parse_error_attribute(&attr);
        assert!(result.is_err());
    }

    #[test]
    fn extract_masterror_spec_valid() {
        let attrs: Vec<Attribute> =
            vec![parse_quote! { #[masterror(code = 1, category = Cat::A)] }];
        let mut errors = Vec::new();
        let result = extract_masterror_spec(&attrs, &mut errors);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn extract_masterror_spec_error() {
        let attrs: Vec<Attribute> = vec![parse_quote! { #[masterror(invalid)] }];
        let mut errors = Vec::new();
        let result = extract_masterror_spec(&attrs, &mut errors);
        assert!(result.is_err());
        assert!(!errors.is_empty());
    }

    #[test]
    fn extract_app_error_spec_valid() {
        let attrs: Vec<Attribute> = vec![parse_quote! { #[app_error(kind = Kind::A)] }];
        let mut errors = Vec::new();
        let result = extract_app_error_spec(&attrs, &mut errors);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn extract_app_error_spec_error() {
        let attrs: Vec<Attribute> = vec![parse_quote! { #[app_error(invalid)] }];
        let mut errors = Vec::new();
        let result = extract_app_error_spec(&attrs, &mut errors);
        assert!(result.is_err());
        assert!(!errors.is_empty());
    }

    #[test]
    fn extract_display_spec_valid() {
        let attrs: Vec<Attribute> = vec![parse_quote! { #[error("message")] }];
        let mut errors = Vec::new();
        let result = extract_display_spec(&attrs, Span::call_site(), &mut errors);
        assert!(result.is_ok());
    }

    #[test]
    fn extract_display_spec_error() {
        let attrs: Vec<Attribute> = vec![parse_quote! { #[error(invalid)] }];
        let mut errors = Vec::new();
        let result = extract_display_spec(&attrs, Span::call_site(), &mut errors);
        assert!(result.is_err());
    }

    #[test]
    fn parse_provide_attribute_missing_comma() {
        let attr: Attribute = parse_quote! { #[provide(ref = A value = B)] };
        let result = parse_provide_attribute(&attr);
        assert!(result.is_err());
    }
}
